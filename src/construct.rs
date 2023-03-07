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
use crate::surface::*;

pub fn point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 1.)
}

pub fn inf_point_from_xyz(x: Float, y: Float, z: Float) -> Trivector {
    Trivector::new(x, y, z, 0.)
}

/// Returns a plane from the standard-form coefficients.
/// Standard form equation is ax + by + cz + d = 0
pub fn plane_from_standard_form(a: Float, b: Float, c: Float, d: Float) -> Vector {
    Vector::new(d, a, b, c)
}

/// Returns a plane that intersects the three given points.
pub fn plane_from_three_points(p0: Trivector, p1: Trivector, p2: Trivector) -> Vector {
    p0 & p1 & p2
}

impl Curve {
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
}

impl Surface {
    pub fn plane(p: Vector) -> Surface {
        let projected_origin = point_from_xyz(0., 0., 0.).project(p);
        let cardinal_points = [
            point_from_xyz(1., 0., 0.).project(p),
            point_from_xyz(0., 1., 0.).project(p),
            point_from_xyz(0., 0., 1.).project(p),
        ];

        // Figure out which cardinal axis projects the least onto the given plane
        let least_axis = cardinal_points.iter().map(|&p| (projected_origin & p).norm()).enumerate().min_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap()).map(|(i, _)| i).unwrap();

        // Use the other two axes as the basis
        let selected_cardinal_points: Vec<Trivector> = cardinal_points.iter().enumerate().filter_map(|(i, &pt)| if i == least_axis {None} else {Some(pt)}).collect();

        let primary = selected_cardinal_points[0];
        let secondary = selected_cardinal_points[1];

        // Redefine plane to get consistent ordering
        let p = projected_origin & primary & secondary;

        let lu = (projected_origin & primary).hat();
        let lv = (p ^ (lu | projected_origin)).hat();

        Surface::Plane(Plane {
            p0: projected_origin,
            du: lu * I,
            dv: lv * I,
        })
    }

    pub fn plane_from_three_points(p0: Trivector, p1: Trivector, p2: Trivector) -> Surface {
        let lu = p0 & p1;
        let length_u = lu.norm();
        assert!(length_u > FLOAT_DIVISION_EPSILON, "p0 and p1 are coincident");
        let du = lu * I * (1. / length_u);

        // TODO don't use p2, use a perpendicular vector

        let lv = p0 & p2;
        let length_v = lv.norm();
        assert!(length_v > FLOAT_DIVISION_EPSILON, "p0 and p2 are coincident");
        let dv = lv * I * (1. / length_v);

        Surface::Plane(Plane {p0, du, dv})
    }
}
