/// Construct geometric primitives such as:
/// Points / Trivectors, Lines / Bivectors, Planes / Vectors,
/// curves, and surfaces.
///
/// All of these primitives may exist outside the context of a Topo
/// and are at a lower level of abstraction
/// (i.e. this module does not deal with topological connectivity)

use crate::pga::*;
use crate::global::*;
use crate::curve::*;

pub fn point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 1.)
}

pub fn inf_point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 0.)
}

pub fn plane_from_standard_form(a: Float, b: Float, c: Float, d: Float) -> Vector {
    // Returns a plane from the standard-form coefficients.
    // Standard form equation is ax + by + cz + d = 0
    Vector::new(d, a, b, c)
}

pub fn line_from_two_points(p0: Trivector, p1: Trivector) -> Curve {
    let line = p0 & p1;
    let length = line.norm();
    assert!(length > FLOAT_DIVISION_EPSILON, "p0 and p1 are coincident");
    let direction = line * I * (1. / length);

    Curve::Line(Line {p0: p0, d: direction})
}

pub fn circle_from_three_points(p0: Trivector, p1: Trivector, p2: Trivector) -> Curve {
    fn perpendicular_bisector(a: Trivector, b: Trivector) -> Vector {
        (a & b) | (a + b)
    }

    let bisector0 = perpendicular_bisector(p0, p1);
    assert!(bisector0.is_finite(), "p0 and p1 are coincident");
    let bisector1 = perpendicular_bisector(p1, p2);
    assert!(bisector0.is_finite(), "p1 and p2 are coincident");
    let axis = bisector1 ^ bisector0;
    assert!(axis.is_finite(), "p0, p1, and p2 are too close together");
    let axis = axis.hat();
    assert!(bisector0.is_finite());

    //let plane0 = (p0 & axis).hat();
    //let plane1 = (p2 & axis).hat();

    // This angle will only be between 0 and 180 degrees.
    // We must calculate the sense of the arc and adjust it if necessary.
    //let angle = (plane1 | plane0).acos();

    // Compute a separating plane perpendicular to the circle through p0 and p2
    let circle_plane = axis | p1;
    let separating_plane = circle_plane | (p0 & p2);

    // Find the center point of the circle.
    // If the center point of the circle is on the positive side of the separating plane,
    // the arc is > 180 degrees. Otherwise it is < 180 degrees.

    // Since the normalization process above always minimizes the angle of the arc,
    // we should flip the axis direction if the angle is > 180.
    let center = axis ^ circle_plane;
    let sense = separating_plane & center > 0.;

    let axis = match sense {
        true => axis.reverse(),
        false => axis,
    };

    Curve::Circle(Circle {p0: p0, a: axis})
}
