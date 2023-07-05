use crate::utils::cross_product;
use super::{Primitive, Program, ScreenBuffer};
use crate::interpolate::Interpolate;

pub struct Triangle;
impl Primitive for Triangle {
    fn draw<P: Program>(program: &P, vertices: &[&P::Vertex], target: &mut ScreenBuffer) {
        let ntris = vertices.len() / 3;
        for i in 0..ntris {
            let mut pts = Vec::new();
            for j in 0..3 {
                let mut pt = [0.0;3];
                let data = program.vertex(&vertices[i * 3 + j], &mut pt);
                pts.push((pt, data));
            }

            // Actually draw the triangle
            let mut pts_conv = Vec::new();
            for pt in &pts {
                let converted = target.conv_ndc_coords(pt.0[0], pt.0[1]);
                pts_conv.push([converted[0] as f32, converted[1] as f32, pt.0[2]]);
            }
        
            let mut bboxmin = [f32::MAX, f32::MAX];
            let mut bboxmax = [-f32::MAX, -f32::MAX];
        
            for pt in &pts {
                for j in 0..2 {
                    bboxmin[j] = bboxmin[j].min(pt.0[j]).max(-1.0);
                    bboxmax[j] = bboxmax[j].max(pt.0[j]).min(1.0);
                }
            }
        
            let bboxmin = target.conv_ndc_coords(bboxmin[0], bboxmin[1]);
            let bboxmax = target.conv_ndc_coords(bboxmax[0], bboxmax[1]);
            for x in bboxmin[0]..=bboxmax[0] {
                for y in bboxmin[1]..=bboxmax[1] {
                    let p = [x as f32, y as f32, 0.0];
                    let bc = barycentric(pts_conv[0], pts_conv[1], pts_conv[2], p);
                    if bc[0] >= 0.0 && bc[1] >= 0.0 && bc[2] >= 0.0 {
                        let mut z = 0.0;
                        for i in 0..3 { z += pts[i].0[2] * bc[i]; }

                        // check zbuffer
                        if target.write_zbuffer(x, y, z) {
                            let data_interp = P::VertexOut::interpolate(
                                &[pts[0].1.clone(), pts[1].1.clone(), pts[2].1.clone()], 
                                &bc
                            );
    
                            let mut color = [0.0;4];
                            if !program.fragment(data_interp, &mut color) {
                                target.draw(x, y, color);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn barycentric(a: [f32;3], b: [f32;3], c: [f32;3], p: [f32;3]) -> [f32;3] {
    let u1 = [c[0]-a[0], b[0]-a[0], a[0]-p[0]];
    let u2 = [c[1]-a[1], b[1]-a[1], a[1]-p[1]];
    let u = cross_product(u1, u2);

    if u[2].abs() < 1.0 { return [-1.0,1.0,1.0]; }
    [1.0 - (u[0]+u[1])/u[2], u[1]/u[2], u[0]/u[2]]
}