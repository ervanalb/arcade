use crate::pga::*;
use crate::global::*;

pub fn point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 1.)
}

pub fn inf_point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 0.)
}
