use crate::vec::Vec3;
use crate::vertex::Vertex;
use crate::error::Error;
use crate::error::Result;
use crate::limits;

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
    pub fn new(pt1: Vertex, pt2: Vertex) -> Result<Segment> {
        pt1.check_vertex_separation(&pt2)?;

        Ok(Segment {
            a: pt1.point(),
            b: pt2.point() - pt1.point()
        })
    }
}

impl Edge for Segment {
    fn eval(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}

#[derive(Debug)]
pub struct Arc {
    // Implements a circular arc parameterized as C + A cos(theta) + B sin(theta)
    // a and b are radii of the circle, and are perpendicular to each other. a points to the start.
    // theta = t * angle
    c: Vec3, // Center point
    a: Vec3, // A radius of the circle pointing to the first point
    b: Vec3, // A radius of the circle perpendicular to a
    angle: f64, // How much of an arc to sweep out
}

impl Arc {
    //pub fn from_three_points(pt1: Vertex, pt2: Vertex, pt3: Vertex) -> Result<Arc> {
    //}
}
