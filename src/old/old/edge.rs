use crate::vertex::Vertex;
use crate::error::Error;
use crate::error::Result;
use crate::types::{Vec3, VecN, Mat4xN, Mat3xN};
use crate::geometry::{BaseSegment, BaseCubicNURBSCurve, weights_to_homo};
use crate::limits;

pub trait Edge {
    // An Edge:
    // * is parameterized by a value t which goes from 0 to 1
    // * is C0 continuous
    // * is Non-self-intersecting
    // * is Non-zero length
    // * can be open or closed
    // * is trimmable and splittable
    // * may be extensible (evaluable outside of the range [0, 1]

    type TrimmedEdge: Edge;

    // Evaluate the edge at the given parameter value
    fn d0(&self, t: f64) -> Vec3; // Point on edge at parameter value t
    fn d1(&self, t: f64) -> Option<Vec3> { // First derivative with respect to parameter value t
        assert!(self.parameter_within_bounds(t));
        None
    }
    fn d2(&self, t: f64) -> Option<Vec3> { // Second derivative with respect to parameter value t
        assert!(self.parameter_within_bounds(t));
        None
    }

    // Returns true if the edge is closed
    fn is_closed(&self) -> bool {
        let v1 = Vertex::new(self.d0(0.)).unwrap();
        let v2 = Vertex::new(self.d0(1.)).unwrap();
        v1.is_coincident(v2)
    }

    // Returns an edge that corresponds to a subset of the current edge evaluated between t=start and t=end
    fn trimmed(&self, start: f64, end: f64) -> Result<Self::TrimmedEdge>;

    // Returns the valid bounds for extending the given edge, or None for no bound (infinitely extensible in that direction)
    fn parameter_bounds(&self) -> (Option<f64>, Option<f64>) {
        (Some(0.), Some(1.))
    }

    fn parameter_within_bounds(&self, t: f64) -> bool {
        let (lower_bound, upper_bound) = self.parameter_bounds();
        let check_lower = match lower_bound {
            Some(lower_bound_t) => (t >= lower_bound_t),
            None => true
        };

        let check_upper = match upper_bound {
            Some(upper_bound_t) => (t <= upper_bound_t),
            None => true
        };

        check_lower && check_upper
    }

    // Returns a set of points, the convex hull of which contains the portion of the Edge between start_t and end_t
    // In the limit as start_t approaches end_t, this set must converge on a single point.
    fn spatial_bounding_points(&self, start: f64, end: f64) -> Mat3xN;
}

#[derive(Debug)]
pub struct Segment {
    // Implements a segment parameterized as A + B * t where t ranges from 0 to 1
    g: BaseSegment,
}

impl Segment {
    fn check(&self) -> Result<()> {
        let v1 = Vertex::new(self.g.a)?;
        let v2 = Vertex::new(self.g.a + self.g.b)?;
        v1.check_vertex_separation(&v2)?;

        Ok(())
    }

    pub fn new(pt1: &Vertex, pt2: &Vertex) -> Result<Segment> {
        let result = Segment {
            g: BaseSegment {
                a: pt1.point(),
                b: pt2.point() - pt1.point()
            }
        };

        result.check()?;

        Ok(result)
    }
}

impl Edge for Segment {
    type TrimmedEdge = Segment;

    // Evaluate the edge at the given parameter value
    fn d0(&self, t: f64) -> Vec3 {
        assert!(self.parameter_within_bounds(t));

        self.g.curve_point(t)
    }

    // First derivative with respect to parameter value t
    fn d1(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(self.g.b)
    }

    // Second derivative with respect to parameter value t
    fn d2(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(Vec3::zeros())
    }

    fn is_closed(&self) -> bool {
        false
    }

    fn parameter_bounds(&self) -> (Option<f64>, Option<f64>) {
        (None, None)
    }

    fn trimmed(&self, start: f64, end: f64) -> Result<Segment> {
        assert!(self.parameter_within_bounds(start));
        assert!(self.parameter_within_bounds(end));

        let v1 = Vertex::new(self.d0(start))?;
        let v2 = Vertex::new(self.d0(end))?;
        Segment::new(&v1, &v2)
    }

    fn spatial_bounding_points(&self, start: f64, end: f64) -> Mat3xN {
        assert!(end > start);
        assert!(self.parameter_within_bounds(start));
        assert!(self.parameter_within_bounds(end));

        return Mat3xN::from_columns(&[self.g.curve_point(start), self.g.curve_point(end)]);
    }
}

#[derive(Debug)]
pub struct CubicNURBSCurve {
    g: BaseCubicNURBSCurve,
    start: f64,
    end: f64,
}

impl CubicNURBSCurve {
    fn check(&self) -> Result<()> {
        // Check that there are at least two g.points
        if self.g.points.ncols() < 2 {
            return Err(Error::DegenerateCurve);
        }

        // Check that there are the right number of g.knots
        if self.g.knots.len() != self.g.points.ncols() + 4 {
            return Err(Error::InvalidParameters);
        }

        // Check that g.knots vector is non-decreasing
        let mut knots_iter = self.g.knots.iter();
        let mut prev = knots_iter.next().expect("Empty g.knots vector");
        for knot in knots_iter {
            if knot < prev {
                return Err(Error::InvalidParameters);
            }
            prev = knot;
        }

        // Check that the knots vector has non-zero span
        if self.g.max_u() - self.g.min_u() < limits::MINIMUM_PARAMETER_SEPARATION {
            return Err(Error::DegenerateCurve);
        }

        // Check that the bounds are between the g.knots and are non-zero span
        if self.end - self.start < limits::MINIMUM_PARAMETER_SEPARATION || self.start < self.g.knots[0] || self.end > self.g.knots[self.g.knots.len() - 1] {
            return Err(Error::InvalidParameters);
        }

        // TODO: check for C0 discontinuities
        // TODO: check for self-intersections
        // TODO: check endpoint separation

        Ok(())
    }

    pub fn new(points: &Mat4xN, knots: &VecN) -> Result<CubicNURBSCurve> {
        // Do the conversion to homogeneous coordinates right away
        let g = BaseCubicNURBSCurve {
            points: weights_to_homo(points),
            knots: knots.clone(),
        };

        let result = CubicNURBSCurve {
            start: g.min_u(),
            end: g.max_u(),
            g: g,
        };

        result.check()?;

        Ok(result)
    }

    pub fn insert_knot(&self, t: f64, repeat: usize) -> Result<CubicNURBSCurve> {
        assert!(self.parameter_within_bounds(t));

        let result = CubicNURBSCurve {
            g: self.g.insert_knot(self.t_to_u(t), repeat),
            start: self.start,
            end: self.end,
        };
        result.check()?;
        Ok(result)
    }

    fn t_to_u(&self, t: f64) -> f64 {
        // t is the normalized parameter passed in to the Edge
        // u is the spline parameter
        // for t in [0, 1] u will be in [start, end]

        t * (self.end - self.start) + self.start
    }

}

impl Edge for CubicNURBSCurve {
    type TrimmedEdge = CubicNURBSCurve;

    // Evaluate the edge at the given parameter value
    fn d0(&self, t: f64) -> Vec3 {
        assert!(self.parameter_within_bounds(t));

        // See "The NURBS Book", page 82, algorithm A3.1
        let u = self.t_to_u(t);
        self.g.curve_point(u)
    }

    // First derivative with respect to parameter value t
    fn d1(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(Vec3::zeros()) // XXX
    }

    // Second derivative with respect to parameter value t
    fn d2(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(Vec3::zeros()) // XXX
    }

    fn parameter_bounds(&self) -> (Option<f64>, Option<f64>) {
        let width = self.end - self.start;
        assert!(width >= limits::EPSILON_PARAMETER);
        (Some((self.g.min_u() - self.start) / width), Some(1. - (self.g.max_u() - self.end) / width))
    }

    fn trimmed(&self, start: f64, end: f64) -> Result<CubicNURBSCurve> {
        assert!(self.parameter_within_bounds(start));
        assert!(self.parameter_within_bounds(end));

        // Bypass the ::new() constructor so that we don't have to convert from/to homogeneous coordinates unnecessarily
        let result = CubicNURBSCurve {
            g: self.g.clone(),
            start: self.t_to_u(start),
            end: self.t_to_u(end)
        };
        result.check()?;
        Ok(result)
    }

    fn spatial_bounding_points(&self, start: f64, end: f64) -> Mat3xN {
        assert!(self.parameter_within_bounds(start));
        assert!(self.parameter_within_bounds(end));
        assert!(end > start);

        self.g.trimmed(self.t_to_u(start), self.t_to_u(end)).bounding_points()
    }
}

/*
#[derive(Debug)]
pub struct Arc {
    // Implements a circular arc parameterized as C + A cos(theta) + B sin(theta)
    // a and b are radii of the circle, and are perpendicular to each other. a points to the start.
    // theta = t * angle
    c: Vector3, // Center point
    a: Vector3, // A radius of the circle pointing to the first point
    b: Vector3, // A radius of the circle perpendicular to a
    angle: f64, // How much of an arc to sweep out
}

impl Arc {
    pub fn from_three_points(pt1: &Vertex, pt2: &Vertex, pt3: &Vertex) -> Result<Arc> {
        pt1.check_colinear(pt2, pt3)?;

        // This from https://math.stackexchange.com/a/1743505

        // Compute a new coordinate frame uvw where the three points lie in the uv plane.
        // The first point is the origin of the coordinate frame
        // and the second point lies on the Y axis.
        let u1 = pt2.point() - pt1.point();
        let o1 = pt3.point() - pt1.point();
        let w1 = o1.cross(u1);
        let u = u1.normalized();
        let w = w1.normalized();
        let v = w.cross(u);

        // Compute two vectors b and c that go from the origin of the new coordinate system (pt1)
        // to pt2 (b) and pt3 (c)
        let b = Vec2::new(u1.length(), 0.);
        let c = Vec2::new(o1 * u, o1 * v);

        // The center of the circle lies on the line x = bx / 2
        // and is equidistant from the origin and C,
        // let us call this point (bx / 2, h)
        // Solving for h gives us:

        let h = ((c.x - 0.5 * b.x).sq() + c.y.sq() - (0.5 * b.x).sq()) / (2. * c.y);
        let center_2d = Vec2::new(0.5 * b.x, h);

        // Get two radii in 2D, the first pointing from center to pt1
        // and the second orthogonal to it in the direction that the arc goes.
        let radius1 = -center_2d;
        let radius2 = Vec2::new(-center_2d.y, center_2d.x);

        // Use atan2 in this coordinate frame to get the angular sweep of the arc.
        let angle = ((c - center_2d) * radius2).atan2((c - center_2d) * radius1);
        let angle = angle + (2. * PI) * (angle < 0.) as i32 as f64;

        // Reproject into 3D
        let center_3d = pt1.point() + center_2d.x * u + center_2d.y * v;
        let radius1_3d = pt1.point() + radius1.x * u + radius1.y * v;
        let radius2_3d = pt1.point() + radius2.x * u + radius2.y * v;

        Ok(Arc {
            c: center_3d,
            a: radius1_3d,
            b: radius2_3d,
            angle: angle
        })
    }
}

impl Edge for Arc {
    fn eval(&self, t: f64) -> Vector3 {
        let theta = t * self.angle;
        self.c + self.a * theta.cos() + self.b * theta.sin()
    }
}
*/

// TESTS
#[test]
fn segment_construction() {
    let v = limits::WORKSPACE_SIZE * Vec3::new(0.5, 0.9, -0.3);

    assert!(
        Segment::new(&Vertex::new(v).unwrap(),
                     &Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(1.5, 0.5, -2.)).unwrap())
        .is_ok());

    assert_eq!(
        Segment::new(&Vertex::new(v).unwrap(),
                     &Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(0.3, 0.5, -0.8)).unwrap()).unwrap_err(),
        Error::VerticesTooClose);
}

#[test]
fn segment_splitting() {
    let base_segment = Segment::new(&Vertex::new(Vec3::new(0.5, 0.9, -0.3)).unwrap(), &Vertex::new(Vec3::new(0.3, 0.5, -0.8)).unwrap()).unwrap();

    assert!(base_segment.trimmed(0., 0.5).is_ok());
    assert!(base_segment.trimmed(0., 2.).is_ok());
    assert_eq!(base_segment.trimmed(0., 0.).unwrap_err(), Error::VerticesTooClose);
}
