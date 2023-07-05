use super::{ScreenBuffer, Program};
use crate::utils::mix;

pub trait Primitive {
    fn draw<P: Program>(program: &P, vertices: &[&P::Vertex], target: &mut ScreenBuffer);
}

pub struct Points;
impl Primitive for Points {
    fn draw<P: Program>(program: &P, vertices: &[&P::Vertex], target: &mut ScreenBuffer) {
        for vertex in vertices {
            let mut trans_v = [0.0;3];
            let vert_out = program.vertex(vertex, &mut trans_v);
            let mut color = [0.0;4];
            let _discard = program.fragment(vert_out, &mut color);
            target.draw_ndc(trans_v[0], trans_v[1], color);
        }
    }
}

pub struct Lines;
impl Primitive for Lines {
    fn draw<P: Program>(program: &P, vertices: &[&P::Vertex], target: &mut ScreenBuffer) {
        for i in 0..vertices.len() {
            let mut a = [0.0;3];
            let mut color_a = [0.0;4];
            program.fragment(program.vertex(&vertices[i], &mut a), &mut color_a);
            let mut b = [0.0;3];
            let mut color_b = [0.0;4];
            program.fragment(program.vertex(&vertices[(i+1)%vertices.len()], &mut b), &mut color_b);
            draw_line(target, 
                target.conv_ndc_coords(a[0], a[1]),
                target.conv_ndc_coords(b[0], b[1]),
                color_a,
                color_b
            );  
        }
    }
}

fn draw_line(target: &mut ScreenBuffer, a: [i32;2], b: [i32;2], color_a: [f32;4], color_b: [f32;4]) {
    let dx = (b[0]-a[0]).abs();
    let sx: i32 = if a[0] < b[0] { 1 } else { -1 };
    let dy = -(b[1]-a[1]).abs();
    let sy: i32 = if a[1] < b[1] { 1 } else { -1 };
    let mut error = dx + dy;

    let total_dist_sq = (dx*dx + dy*dy) as f32;
    let mut x = a[0];
    let mut y = a[1];
    loop {
        let dist_x = (a[0]-x).abs();
        let dist_y = (a[1]-y).abs();
        let t = ((dist_x*dist_x+dist_y*dist_y) as f32)/total_dist_sq;
        let color = mix(color_a, color_b, t);
        target.draw(x, y, color);
        
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