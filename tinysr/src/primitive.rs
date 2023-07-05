use super::{ScreenBuffer, Program};

pub trait Primitive {
    fn draw<P: Program>(program: &P, vertices: &[P::Vertex], target: &mut ScreenBuffer);
}

pub struct Points;
impl Primitive for Points {
    fn draw<P: Program>(program: &P, vertices: &[P::Vertex], target: &mut ScreenBuffer) {
        for vertex in vertices {
            let trans_v = program.vertex(vertex);
            let color = program.fragment(trans_v.1);
            target.draw_ndc(trans_v.0[0], trans_v.0[1], color);
        }
    }
}