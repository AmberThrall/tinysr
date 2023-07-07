use crate::utils::cross_product;
use super::{Primitive, Program, ScreenBuffer, Fragment};
use crate::interpolate::Interpolate;

pub struct Triangles;
impl Primitive for Triangles {
    fn draw<P: Program>(program: &P, vertices: &[&P::Vertex], target: &mut ScreenBuffer) {
        let ntris = vertices.len() / 3;
        for i in 0..ntris {
            // compute vertex shader for 3 vertices
            let mut a_hom = [0.0;4];
            let a_data = program.vertex(&vertices[i * 3], &mut a_hom);
            let mut b_hom = [0.0;4];
            let b_data = program.vertex(&vertices[i * 3 + 1], &mut b_hom);
            let mut c_hom = [0.0;4];
            let c_data = program.vertex(&vertices[i * 3 + 2], &mut c_hom);

            // Convert homogenous to euclidean
            let a = [0,1,2].map(|i| a_hom[i]/a_hom[3]);
            let b = [0,1,2].map(|i| b_hom[i]/b_hom[3]);
            let c = [0,1,2].map(|i| c_hom[i]/c_hom[3]);
        
            // compute framebuffer coordinates
            let a_scr = target.conv_ndc_coords(a[0], a[1]);
            let b_scr = target.conv_ndc_coords(b[0], b[1]);
            let c_scr = target.conv_ndc_coords(c[0], c[1]);
            
            // generate bounding box
            let mut bboxmin = [0,0];
            let mut bboxmax = [0,0];
            let clamp = target.viewport().size;
            for j in 0..2 {
                bboxmin[j] = a_scr[j].min(b_scr[j]).min(c_scr[j]).max(0).min(clamp[j]);
                bboxmax[j] = a_scr[j].max(b_scr[j]).max(c_scr[j]).max(0).min(clamp[j]);
            }
        
            // check each pixel in the bounding box.
            for x in bboxmin[0]..=bboxmax[0] {
                for y in bboxmin[1]..=bboxmax[1] {
                    let bc = barycentric(
                        [a_scr[0] as f32, a_scr[1] as f32, 0.0],
                        [b_scr[0] as f32, b_scr[1] as f32, 0.0],
                        [c_scr[0] as f32, c_scr[1] as f32, 0.0],
                        [x as f32 + 0.5, y as f32 + 0.5, 0.0]
                    );
                    if bc[0] < 0.0 || bc[1] < 0.0 || bc[2] < 0.0 { continue; }
                    
                    let z = a[2] * bc[0] + b[2] * bc[1] + c[2] * bc[2];

                    // check zbuffer
                    if target.write_zbuffer(x, y, z) {
                        let data_interp = P::VertexOut::interpolate(
                            &[a_data.clone(), b_data.clone(), c_data.clone()], 
                            &bc
                        );

                        let mut color = [0.0;4];
                        if program.fragment(data_interp, &mut color) == Fragment::Keep {
                            target.draw(x, y, color);
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