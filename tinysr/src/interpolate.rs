pub trait Interpolate {
    fn interpolate<const N: usize>(v: &[Self;N], weights: &[f32;N]) -> Self where Self: Sized;
}

impl Interpolate for f32 {
    fn interpolate<const N: usize>(v: &[Self;N], weights: &[f32;N]) -> Self {
        let mut res = 0.0;
        for i in 0..N {
            res += v[i] * weights[i];
        }
        res
    }
}

impl<const M: usize> Interpolate for [f32;M] {
    fn interpolate<const N: usize>(v: &[Self;N], weights: &[f32;N]) -> Self {
        let mut res = [0.0;M];
        for j in 0..M {
            for i in 0..N {
                res[j] += v[i][j] * weights[i];
            }
        }
        res
    }
}
