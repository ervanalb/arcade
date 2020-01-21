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
    fn check_segment_length(pt1: &Vertex, pt2: &Vertex) -> Result<()> {
        match (pt2.point() - pt1.point()).is_within(limits::MINIMUM_VERTEX_SEPARATION) {
            true => Err(Error::VerticesTooClose),
            false => Ok(())
        }
    }

    pub fn new(pt1: Vertex, pt2: Vertex) -> Result<Segment> {
        Segment::check_segment_length(&pt1, &pt2)?;

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
