 # tinysr

 tinysr is a small software renderer written in rust with no dependencies.

 # Example

 ```rust
 use tinysr::*;

struct Shader;
impl Program for Shader {
    type Vertex = [f32; 6];
    type VertexOut = [f32;3];
    
    fn vertex(&self, v: &Self::Vertex, position: &mut [f32;4]) -> Self::VertexOut {
        *position = [v[0], v[1], v[2], 1.0];
        [v[3],v[4],v[5]]
    }

    fn fragment(&self, v: Self::VertexOut, color: &mut [f32;4]) -> bool {
        *color = [v[0], v[1], v[2], 1.0];
        false
    }
}

fn main() {
    let mut tinysr = TinySR::default();
    tinysr.set_viewport(0,0, 800, 600);

    let shader = Shader;

    let vertices = vec![
        //  X     Y    Z      R    G    B
        [-0.5, -0.5, 0.0,   1.0, 0.0, 0.0],
        [ 0.5, -0.5, 0.0,   0.0, 1.0, 0.0],
        [ 0.0,  0.5, 0.0,   0.0, 0.0, 1.0],
    ];
    tinysr.draw_array::<Triangles,_>(&shader, &vertices);
}
```

## Sample Output

![](https://github.com/AmberThrall/tinysr/blob/main/sample.jpg?raw=true)