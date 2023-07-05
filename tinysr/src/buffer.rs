pub struct Buffer<T> {
    stride: usize,
    data: Vec<T>,
}

impl<T> Buffer<T> {
    pub fn new_empty(stride: usize) -> Self {
        Self {
            stride,
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() / self.stride
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn get(&self, i: usize) -> Option<&[T]> {
        self.data.get((i*self.stride)..((i+1)*self.stride))
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut [T]> {
        self.data.get_mut((i*self.stride)..((i+1)*self.stride))
    }
}

impl<T: Clone> Buffer<T> {
    pub fn new(stride: usize, v: &[T]) -> Result<Self, String> {
        let mut data = Vec::new();
        if data.len() % stride != 0 {
            Err(format!("data of len {} does not align with buffer's stride ({}).", data.len(), stride))
        } else {
            for x in v { data.push(x.clone()); }
            Ok(Self {
                stride,
                data,
            })
        }
    }

    pub fn push(&mut self, v: &[T]) -> Result<(), String> {
        if v.len() % self.stride != 0 {
            Err(format!("data of len {} does not align with buffer's stride ({}).", v.len(), self.stride))
        } else {
            for x in v { self.data.push(x.clone()); }
            Ok(())
        }
    }

    pub fn insert(&mut self, index: usize, v: &[T]) -> Result<(), String> {
        if v.len() % self.stride != 0 {
            Err(format!("data of len {} does not align with buffer's stride ({}).", v.len(), self.stride))
        } else {
            let mut i = index;
            for x in v { 
                self.data.insert(i, x.clone()); 
                i += 1;
            }
            Ok(())
        }
    }
}

impl<T> Default for Buffer<T> {
    fn default() -> Self {
        Self::new_empty(1)
    }
}

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
        if x > self.size[0] || y > self.size[1] {
            return None;
        }
        self.data.get(y * self.size[0] + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x > self.size[0] || y > self.size[1] {
            return None;
        }
        self.data.get_mut(y * self.size[0] + x)
    }
}

impl<T> Default for Buffer2d<T> {
    fn default() -> Self {
        Self {
            size: [0,0],
            data: vec![],
        }
    }
}