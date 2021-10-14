#[derive(Default,Debug,Clone,PartialEq)]
pub struct Vertex {
    pub point: Trivector,
}

impl for Vertex {
    fn new(x: Float, y: Float, z: Float) -> Self {
        Vertex {
            point: Trivector {
                a11: z,
                a12: y,
                a13: x,
                a14: 1.,
            }
        }
    }
}
