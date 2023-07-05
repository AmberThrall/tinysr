pub struct Rect<T> {
    pub origin: [T; 2],
    pub size: [T; 2],
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            origin: [x, y],
            size: [width, height],
        }  
    }
}

impl<T: Default> Default for Rect<T> {
    fn default() -> Self {
        Self {
            origin: [T::default(), T::default()],
            size: [T::default(), T::default()],
        }
    }
}