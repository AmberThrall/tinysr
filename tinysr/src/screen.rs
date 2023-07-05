use super::{Rect, Buffer2d};

#[derive(Default)]
pub struct ScreenBuffer {
    viewport: Rect<i32>,
    buffer: Buffer2d<[f32;4]>,
}

impl ScreenBuffer {
    /// Creates a screen buffer
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate of lower-left corner
    /// * `y` - y coordinate of lower-left corner
    /// * `width` - width of viewport
    /// * `height` - height of viewport
    pub fn new(x: i32, y: i32, width: usize, height: usize) -> Self {
        let mut s = Self::default();
        s.resize(x, y, width, height);
        s
    }

    /// Resizes the screen buffer
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate of lower-left corner
    /// * `y` - y coordinate of lower-left corner
    /// * `width` - width of viewport
    /// * `height` - height of viewport
    pub fn resize(&mut self, x: i32, y: i32, width: usize, height: usize) {
        self.viewport = Rect::new(x, y, width as i32, height as i32);
        self.buffer = Buffer2d::new([width, height], [0.0, 0.0, 0.0, 1.0]);
    }

    /// Writes a color to the screen buffer.
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate
    /// * `y` - y coordinate
    /// * `color` - color to write
    pub fn draw(&mut self, x: i32, y: i32, color: [f32;4]) {
        let p = [x - self.viewport.origin[0], self.viewport.size[1] - y + self.viewport.origin[1]];
        if let Some(elem) = self.buffer.get_mut(p[0] as usize, p[1] as usize) {
            *elem = color.clone();
        }
    }

    /// Writes a color to the screen buffer given NDC coordinates
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate
    /// * `y` - y coordinate
    /// * `color` - color to write
    pub fn draw_ndc(&mut self, x: f32, y: f32, color: [f32;4]) {
        let p = self.conv_ndc_coords(x, y);
        self.draw(p[0], p[1], color);
    }

    /// Returns the viewport
    pub fn viewport(&self) -> &Rect<i32> {
        &self.viewport
    }

    /// Reads a color to the screen buffer.
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate
    /// * `y` - y coordinates
    pub fn get(&self, x: i32, y: i32) -> Option<&[f32;4]> {
        self.buffer.get(x as usize, y as usize)
    }

    /// Reads a color to the screen buffer given NDC coordinates
    /// 
    /// # Arguments
    /// 
    /// * `x` - x coordinate
    /// * `y` - y coordinates
    pub fn get_ndc(&self, x: f32, y: f32) -> Option<&[f32;4]> {
        let p = self.conv_ndc_coords(x, y);
        self.get(p[0], p[1])
    }

    pub fn conv_ndc_coords(&self, x: f32, y: f32) -> [i32;2] {
        let x = (self.viewport.size[0] as f32 / 2.0) * (x + 1.0);
        let y = (self.viewport.size[1] as f32 / 2.0) * (y + 1.0);
        [x as i32, y as i32]
    }
}