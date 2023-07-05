mod buffer;
mod rect;
mod program;
mod screen;
mod primitive;
mod utils;
mod triangle;

pub use buffer::{Buffer, Buffer2d};
pub use screen::ScreenBuffer;
pub use program::Program;
pub use primitive::{Primitive, Points, Lines};
pub use triangle::Triangle;
use rect::Rect;

#[derive(Default)]
pub struct TinySR {
    screen: ScreenBuffer,
}

impl TinySR {
    /// Sets the renderer's viewport
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate of lower-left corner
    /// * `y` - y coordinate of lower-left corner
    /// * `width` - width of viewport
    /// * `height` - height of viewport
    pub fn set_viewport(&mut self, x: i32, y: i32, width: usize, height: usize) {
        self.screen.resize(x, y, width, height);
    }

    pub fn get_screen_buffer(&self) -> &ScreenBuffer {
        &self.screen
    }

    /// Draws a vertex array
    /// 
    /// # Arguments
    /// 
    /// * `P` - what primitive format to use
    /// * `program` - shader program to use
    /// * `vao` - vertex array to draw
    pub fn draw_array<P: Primitive,Pr: Program>(&mut self, program: &Pr, vao: &[Pr::Vertex]) {
        P::draw(program, vao, &mut self.screen);
    }

    // pub fn draw_line(&mut self, a: [i32;2], b: [i32;2], color: Color) {
    //     let dx = (b[0]-a[0]).abs();
    //     let sx: i32 = if a[0] < b[0] { 1 } else { -1 };
    //     let dy = -(b[1]-a[1]).abs();
    //     let sy: i32 = if a[1] < b[1] { 1 } else { -1 };
    //     let mut error = dx + dy;
    
    //     let mut x = a[0];
    //     let mut y = a[1];
    //     loop {
    //         self.set_pixel([x, y], color);
    //         if x == b[0] && y == b[1] { break; }
    //         let e2 = 2 * error;
    //         if e2 >= dy {
    //             if x == b[0] { break; }
    //             error += dy;
    //             x += sx;
    //         }
    //         if e2 <= dx {
    //             if y == b[1] { break; }
    //             error += dx;
    //             y += sy;
    //         }
    //     }
    // }
}