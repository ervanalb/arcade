use crate::pga::*;
use crate::global::*;

pub fn vertices_coincident(p0: Trivector, p1: Trivector) -> bool {
    (p0 & p1).norm() < EPSILON_COINCIDENT_DISTANCE
}
