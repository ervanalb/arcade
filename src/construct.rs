use crate::pga::*;
use crate::global::*;
use crate::curve::*;
use std::sync::Arc;

pub fn point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 1.)
}

pub fn inf_point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 0.)
}

pub fn three_point_arc(p0: Trivector, p1: Trivector, p2: Trivector) -> Curve {
    fn perpendicular_bisector(a: Trivector, b: Trivector) -> Vector {
        ((a & b) | (a + b)).hat()
    }

    let bisector0 = perpendicular_bisector(p0, p1);
    let bisector1 = perpendicular_bisector(p1, p2);
    let axis = bisector1 ^ bisector0;
    let plane0 = p0 & axis;
    let plane1 = p2 & axis;
    let angle = (plane1 ^ plane0).norm().atan2(plane1 | plane0);
    let angle = (angle + 2. * PI) % (2. * PI);

    Curve::TrimmedCurve(TrimmedCurve {
        curve: Arc::from(Curve::Circle(Circle {p0: p0, a: axis})),
        t_start: 0.,
        t_end: angle,
    })
}
