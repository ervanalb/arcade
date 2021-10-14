use crate::pga::*;
use crate::global::*;
use crate::construct::*;

#[derive(Default,Debug,Clone,PartialEq)]
pub struct Vertex {
    pub point: Trivector,
}

impl Vertex {
    pub fn new(x: Float, y: Float, z: Float) -> Vertex {
        Vertex {
            point: point_from_xyz(x, y, z)
        }
    }
}
