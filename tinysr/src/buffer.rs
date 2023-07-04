pub struct Buffer2d<T> {
    size: [usize; 2],
    data: Vec<T>,
}

impl<T: Clone> Buffer2d<T> {
    pub fn new(size: [usize; 2], fill: T) -> Self {
        Self {
            size,
            data: vec![fill; size[0] * size[1]],
        }
    }

    pub fn size(&self) -> [usize; 2] {
        self.size
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y * self.size[0] + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y * self.size[0] + x)
    }
}