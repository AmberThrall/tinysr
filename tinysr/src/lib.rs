mod buffer;

pub use buffer::Buffer2d;

pub fn draw_line<T: Clone>(a: [i32; 2], b: [i32; 2], buffer: &mut Buffer2d<T>, color: T) {
    let dx = (b[0]-a[0]).abs();
    let sx: i32 = if a[0] < b[0] { 1 } else { -1 };
    let dy = -(b[1]-a[1]).abs();
    let sy: i32 = if a[1] < b[1] { 1 } else { -1 };
    let mut error = dx + dy;

    let mut x = a[0];
    let mut y = a[1];
    loop {
        if let Some(elem) = buffer.get_mut(x as usize, y as usize) {
            *elem = color.clone();
        }
        if x == b[0] && y == b[1] { break; }
        let e2 = 2 * error;
        if e2 >= dy {
            if x == b[0] { break; }
            error += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == b[1] { break; }
            error += dx;
            y += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
