/// A surface defines a parametric 2D region of space.
/// Surfaces may be bounded, infinite or periodic
/// in both their U and V directions.
/// Typically, a subset of a surface will be used
/// when building up topology inside a Topo.
///
/// Surfaces as defined by this module may exist outside the context of a Topo
/// and are at a lower level of abstraction.

use crate::pga::*;
use crate::global::*;

#[derive(Debug, Clone)]
pub enum Surface {
    Plane(Plane),
}

impl Surface {
    // Evaluate the point on the surface at parameter values u, v
    pub fn d0(&self, u: Float, v: Float) -> Trivector {
        match &self {
            Surface::Plane(x) => x.d0(u, v),
        }
    }
}

// A plane is parameterized by a (euclidean) point p0 and two infinite lines du and dv.
// The plane starts at p0 and extends mutually orthogonal to du and dv.
// The parameters u and v are the signed distance from p0 in the du and dv directions.
// The parametric equation for the point r at parameter values (u, v) on the plane is:
// r(u, v) = m(u, v) * p0 * ~m(u, v)
// where m(u, v) = exp(u / 2 * du) * exp(v / 2 * dv) = exp(1/2 * (u * du + v * dv))
#[derive(Debug, Clone)]
pub struct Plane {
    pub p0: Trivector,
    pub du: Bivector,
    pub dv: Bivector,
}

impl Plane {
    pub fn d0(&self, u: Float, v: Float) -> Trivector {
        self.p0.transform((0.5 * (u * self.du + v * self.dv)).exp())
    }
}

pub fn surfaces_coincident(s0: &Surface, s1: &Surface) -> Option<Direction> {
    // Some(Forward) => Surfaces are coincident and have the same "sense"
    // Some(Reverse) => Surfaces are coincident but have opposing "sense"
    // None => Surfaces are not coincident

    // TODO implement this
    None
}
