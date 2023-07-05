pub(crate) fn cross_product(a: [f32;3], b: [f32;3]) -> [f32;3] {
    [
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],    
    ]
}