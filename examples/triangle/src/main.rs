use tinysr::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

struct Shader;

impl Program for Shader {
    type Vertex = [f32; 6];
    type VertexOut = [f32;3];
    
    fn vertex(&self, v: &Self::Vertex) -> ([f32;3], Self::VertexOut) {
        (
            [v[0],v[1],v[2]], 
            [v[3],v[4],v[5]]
        )
    }

    fn fragment(&self, v: Self::VertexOut) -> [f32;4] {
        [v[0], v[1], v[2], 1.0]
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

    let shader = Shader;

    let vertices = vec![
        //  X     Y    Z      R    G    B
        [-0.5, -0.5, 0.0,   1.0, 0.0, 0.0],
        [ 0.5, -0.5, 0.0,   0.0, 1.0, 0.0],
        [ 0.0,  0.5, 0.0,   0.0, 0.0, 1.0],
    ];
    tinysr.draw_array::<Triangle,_>(&shader, &vertices);

    // Save the screen buffer to image
    let mut img = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = convert_color(*tinysr.get_screen_buffer().get(x as i32,y as i32).unwrap());
    }
    img.save("output.png").unwrap();
}
