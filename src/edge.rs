use crate::vec::Vec3;
use crate::vertex::Vertex;

trait Edge {
    // Evaluate the edge at the given parameter value
    fn eval(&self, t: f64) -> Vec3;
}

#[derive(Debug)]
pub struct Segment {
    // Implements a segment parameterized as A + B * t where t ranges from 0 to 1
    a: Vec3, // First point
    b: Vec3, // Second point - First point
}

impl Segment {
    pub fn new(pt1: Vertex, pt2: Vertex) -> Segment {
        Segment {
            a: pt1.point(),
            b: pt2.point() - pt1.point()
        }
    }
}

impl Edge for Segment {
    fn eval(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}
