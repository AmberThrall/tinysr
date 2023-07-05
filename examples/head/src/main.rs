use tinysr::*;
use vek::*;
use image::GenericImageView;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[derive(Default, Clone)]
struct Vertex {
    position: Vec3<f32>,
    uv: Vec2<f32>,
    normal: Vec3<f32>,
}

struct Shader {
    mvp: Mat4<f32>,
    light_dir: Vec3<f32>,
    texture: image::DynamicImage,
}

impl Program for Shader {
    type Vertex = Vertex;
    type VertexOut = [f32;5];
    
    fn vertex(&self, v: &Self::Vertex, position: &mut [f32;3]) -> Self::VertexOut {
        let p = (self.mvp * Vec4::from_point(v.position)).into_array();
        *position = [p[0], p[1], p[2]];
        [v.uv.x, v.uv.y, v.normal.x, v.normal.y, v.normal.z]
    }

    fn fragment(&self, vin: Self::VertexOut, color: &mut [f32;4]) -> bool {
        let uv = Vec2::new(vin[0], vin[1]);
        let n = Vec3::new(vin[2], vin[3], vin[4]);

        let texcoords = Vec2::new(
            (uv.x * (self.texture.width() as f32)) as u32,
            self.texture.height() - (uv.y * (self.texture.height() as f32)) as u32,
        );
        let pixel = self.texture.get_pixel(texcoords.x, texcoords.y);
        let texture_color = Rgba::new(
            (pixel.0[0] as f32) / 255.0,
            (pixel.0[1] as f32) / 255.0,
            (pixel.0[2] as f32) / 255.0,
            (pixel.0[3] as f32) / 255.0,
        );

        let ambient = 0.2;
        let diffuse =  n.dot(self.light_dir).max(0.0).min(1.0) * 0.5;
        let specular = self.light_dir
            .reflected(Vec3::from(self.mvp * Vec4::from(n)).normalized())
            .dot(-Vec3::unit_z())
            .powf(20.0);
        let light = ambient + diffuse + specular;

        *color = (texture_color * light).clamped(Rgba::zero(), Rgba::one()).into_array();
        false
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

    let texture = image::open("examples/head/head_diffuse.tga").unwrap();

    let shader = Shader {
        mvp: Mat4::perspective_fov_rh_zo(1.3, WIDTH as f32, HEIGHT as f32, 0.01, 100.0) *
            Mat4::translation_3d(Vec3::new(0.0, 0.0, -2.5)) *
            Mat4::rotation_y(3.14) *
            Mat4::scaling_3d(0.6),
        light_dir: Vec3::new(1.0,1.0,1.0).normalized(),
        texture,
    };

    // Read in the teapot obj
    let (models, _materials) = tobj::load_obj("examples/head/head.obj", &tobj::LoadOptions::default()).expect("failed to load OBJ file.");
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("Rendering \"{}\"...", m.name);

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
        tinysr.draw_elements::<Triangle,_>(&shader, &vertices, &indices);
    }

    // Save the screen buffer to image
    let mut img = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = convert_color(*tinysr.get_screen_buffer().get(x as i32,y as i32).unwrap());
    }
    img.save("output.png").unwrap();
}
