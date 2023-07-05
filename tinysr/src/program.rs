use crate::interpolate::Interpolate;

pub trait Program {
    type Vertex;
    type VertexOut: Interpolate + Clone;

    fn vertex(&self, v: &Self::Vertex, position: &mut [f32;3]) -> Self::VertexOut;
    fn fragment(&self, vin: Self::VertexOut, color: &mut [f32;4]) -> bool;
}