use tinysr::*;
use vek::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};
use image::GenericImageView;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const NFRAMES: usize = 120;

#[derive(Default, Clone)]
struct Vertex {
    position: Vec3<f32>,
    uv: Vec2<f32>,
    normal: Vec3<f32>,
}

struct Shader {
    m_projection: Mat4<f32>,
    m_view: Mat4<f32>,
    m_model: Mat4<f32>,
    eye: Vec3<f32>,
    light_pos: Vec3<f32>,
    texture: image::DynamicImage,
    texture_nm: image::DynamicImage,
    texture_spec: image::DynamicImage,
}

fn texture(texture: &image::DynamicImage, uv: Vec2<f32>) -> Vec4<f32> {
    let texcoords = Vec2::new(
        (uv.x * (texture.width() as f32)) as u32,
        texture.height() - (uv.y * (texture.height() as f32)) as u32,
    );
    let pixel = texture.get_pixel(texcoords.x, texcoords.y);
    Vec4::new(
        (pixel.0[0] as f32) / 255.0,
        (pixel.0[1] as f32) / 255.0,
        (pixel.0[2] as f32) / 255.0,
        (pixel.0[3] as f32) / 255.0,
    )
}

impl Program for Shader {
    type Vertex = Vertex;
    type VertexOut = [f32;5];
    
    fn vertex(&self, v: &Self::Vertex, position: &mut [f32;4]) -> Self::VertexOut {
        *position = (self.m_projection * self.m_view * self.m_model * Vec4::from_point(v.position)).into_array();
        let frag_pos = Vec3::from(self.m_model * Vec4::from_point(v.position));
        [v.uv.x, v.uv.y, frag_pos.x, frag_pos.y, frag_pos.z]
    }

    fn fragment(&self, vin: Self::VertexOut, color: &mut [f32;4]) -> Fragment {
        let uv = Vec2::new(vin[0], vin[1]);
        let frag_pos = Vec3::new(vin[2], vin[3], vin[4]);

        let n: Vec3<f32> = Vec3::from(texture(&self.texture_nm, uv) * 2.0 - Vec3::one()).normalized(); // normal vector
        let light_dir = (self.light_pos - frag_pos).normalized();
        let view_dir = (self.eye - frag_pos).normalized();
        let reflect_dir = (-view_dir).reflected(n);

        let ambient = 0.2;
        let specular = view_dir.dot(reflect_dir).max(0.0).powf(texture(&self.texture_spec, uv)[0]);
        let diffuse = n.dot(light_dir).max(0.0);
        let c = texture(&self.texture, uv);
        let light = ambient + diffuse + 0.3 * specular;

        *color = Rgba::from(c * light).clamped(Rgba::zero(), Rgba::one()).into_array();
        Fragment::Keep
    }
}

fn convert_color(color: [f32;4]) -> image::Rgb<u8> {
    let r = (color[0] * 255.0) as u8;
    let g = (color[1] * 255.0) as u8;
    let b = (color[2] * 255.0) as u8;
    image::Rgb([r,g,b])
}

fn main() {
    let mut tinysr = TinySR::default();
    tinysr.set_viewport(0,0, WIDTH,HEIGHT);

    let texture = image::open("examples/diablo/diablo3_diffuse.tga").unwrap();
    let texture_nm = image::open("examples/diablo/diablo3_nm.tga").unwrap();
    let texture_spec = image::open("examples/diablo/diablo3_spec.tga").unwrap();

    let eye = Vec3::new(-1.0, -1.0, 3.0);
    let target = Vec3::zero();
    let up = Vec3::new(0.0, 1.0, 0.0);

    let m_projection: Mat4<f32> = Mat4::perspective_fov_rh_no(0.785398, WIDTH as f32, HEIGHT as f32, 0.1, 100.0);
    let m_view: Mat4<f32> = Mat4::look_at_rh(eye, target, up);
    let m_model: Mat4<f32> = Mat4::identity();

    let mut shader = Shader {
        m_projection,
        m_view,
        m_model,
        eye,
        light_pos: Vec3::new(1.0,10.0,1.0),
        texture,
        texture_nm,
        texture_spec,
    };

    // Read in the OBJ.
    let (models, _materials) = tobj::load_obj("examples/diablo/diablo3.obj", &tobj::LoadOptions::default()).expect("failed to load OBJ file.");
    let m = &models[0];
    let mesh = &m.mesh;

    // build the vao
    let mut vertices = vec![Vertex::default(); mesh.positions.len()/3];
    for idx in 0..mesh.indices.len()/3 {
        for j in 0..3 {
            let pidx = mesh.indices[idx * 3 + j] as usize;
            let tidx = mesh.texcoord_indices[idx * 3 + j] as usize;
            let nidx = mesh.normal_indices[idx * 3 + j] as usize;
            vertices[pidx].position = Vec3::new(mesh.positions[pidx*3], mesh.positions[pidx*3+1], mesh.positions[pidx*3+2]);
            vertices[pidx].uv = Vec2::new(mesh.texcoords[tidx*2], mesh.texcoords[tidx*2+1]);
            vertices[pidx].normal = Vec3::new(mesh.normals[nidx*3], mesh.normals[nidx*3+1], mesh.normals[nidx*3+2]);
        }
    }

    // Setup the indices and render
    let indices: Vec<usize> = mesh.indices.iter().map(|idx| *idx as usize).collect();

    let pb = ProgressBar::new(NFRAMES as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] frame {pos}/{len} ({eta})")
        .unwrap());
    for frame_n in 0..NFRAMES {
        pb.set_position(frame_n as u64);

        // rotate the model
        let m_roty = Mat4::rotation_y(6.2831853 * (frame_n as f32 / NFRAMES as f32));
        shader.m_model = m_model * m_roty;

        // Render and save the frame
        tinysr.clear_screen([0.0,0.0,0.0,1.0]);
        tinysr.draw_elements::<Triangles,_>(&shader, &vertices, &indices);
        let mut img = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = convert_color(*tinysr.get_screen_buffer().get(x as i32,HEIGHT as i32 - 1 - y as i32).unwrap());
        }
        img.save(&format!("frame{:04}.png", frame_n + 1)).unwrap();
    }
    pb.finish_with_message("done");

    // Run ffmpeg
    println!("executing `ffmpeg -framerate 30 -pattern_type glob -i 'frame*.png' -c:v libx264 -pix_fmt yuv420p out.mp4`...");

    let mut cmd = Command::new("ffmpeg")
        .args([
            "-framerate", "30",
            "-pattern_type", "glob",
            "-i", "frame*.png",
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            "out.mp4",
            "-progress",
            "pipe:1",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    cmd.wait().unwrap();
}
