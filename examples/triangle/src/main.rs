use tinysr::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

struct Shader;

impl Program for Shader {
    type Vertex = [f32; 3];
    type VertexOut = ();
    
    fn vertex(&self, pos: &Self::Vertex) -> ([f32;4], Self::VertexOut) {
        ([pos[0], pos[1], pos[2], 0.0], ())
    }

    fn fragment(&self, _vin: Self::VertexOut) -> [f32;4] {
        [1.0, 0.0, 0.0, 1.0]
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
        [-0.5, -0.5, 0.0],
        [ 0.5, -0.5, 0.0],
        [ 0.0,  0.5, 0.0],
    ];
    tinysr.draw_array(&shader, &vertices, 0, 3);

    // Save the screen buffer to image
    let mut img = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = convert_color(*tinysr.get_screen_buffer().get(x as i32,y as i32).unwrap());
    }
    img.save("output.png").unwrap();
}
