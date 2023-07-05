pub fn mix(a: [f32;4], b: [f32;4], t: f32) -> [f32;4] {
    let mut res = [0.0;4];
    for i in 0..4 {
        res[i] = a[i] + t * (b[i] - a[i]);
    }
    res
}

pub(crate) fn cross_product(a: [f32;3], b: [f32;3]) -> [f32;3] {
    [
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],    
    ]
}