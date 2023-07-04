use tinysr::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut screen = Buffer2d::<[u8;3]>::new([WIDTH, HEIGHT], [0,0,0]);
    draw_line([10,10], [30,30], &mut screen, [255,0,0]);
    draw_line([50,50], [0,10], &mut screen, [0,255,0]);
    draw_line([90,50], [10,50], &mut screen, [0,0,255]);
    draw_line([50,90], [50,10], &mut screen, [255,0,255]);

    // Save the screen buffer to image
    let mut img = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb(screen.get(x as usize,y as usize).unwrap().clone());
    }
    img.save("output.png").unwrap();
}
