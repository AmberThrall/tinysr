pub trait Program {
    type Vertex;
    type VertexOut;

    fn vertex(&self, vao: &Self::Vertex) -> ([f32;3], Self::VertexOut);
    fn fragment(&self, vin: Self::VertexOut) -> [f32;4];
}