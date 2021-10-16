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
    let axis = (bisector1 ^ bisector0).hat();

    let plane0 = (p0 & axis).hat();
    let plane1 = (p2 & axis).hat();
    let angle = (plane1 | plane0).acos();

    // Calculate sense of circle
    let circle_plane = p0 & p1 & p2;
    let separating_plane = circle_plane | (p0 & p2);
    let center = axis ^ circle_plane;
    let sense = (separating_plane & center) * (separating_plane & p1);

    println!("plane & center {:?}, plane & p1 {:?}", (separating_plane & center), (separating_plane & p1));

    let angle = match sense < 0. {
        false => 2. * PI - angle,
        true => angle,
    };

    Curve::TrimmedCurve(TrimmedCurve {
        curve: Arc::from(Curve::Circle(Circle {p0: p0, a: axis})),
        t_start: 0.,
        t_end: angle,
    })
}
