pub fn mix(a: [f32;4], b: [f32;4], t: f32) -> [f32;4] {
    let mut res = [0.0;4];
    for i in 0..4 {
        res[i] = a[i] + t * (b[i] - a[i]);
    }
    res
}