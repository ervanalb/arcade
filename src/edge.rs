use crate::vec::*;
use crate::vertex::Vertex;
use crate::error::Error;
use crate::error::Result;
use crate::limits;
use std::fmt;

pub enum Edge<'a> {
    Segment(Segment),
    Arc(),
    CubicNURBSCurve(CubicNURBSCurve<'a>),
    Generic(Box<dyn GenericEdge>)
}

impl Edge<'_> {
    pub fn unwrap_generic(&self) -> &dyn GenericEdge {
        match self {
            Edge::Segment(e) => e,
            Edge::Arc() => panic!("Arc not implemented yet"),
            Edge::CubicNURBSCurve(e) => e,
            Edge::Generic(box_e) => box_e.as_ref()
        }
    }

    pub fn unwrap_segment(&self) -> &Segment {
        match self {
            Edge::Segment(s) => s,
            _ => panic!("Edge is not segment")
        }
    }
}

impl std::fmt::Debug for Edge<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Edge::Segment(e) => e.fmt(fmt),
            Edge::Arc() => write!(fmt, "Arc()"),
            Edge::CubicNURBSCurve(e) => e.fmt(fmt),
            Edge::Generic(box_e) => box_e.fmt(fmt)
        }
    }
}

pub trait GenericEdge {
    // An Edge:
    // * is parameterized by a value t which goes from 0 to 1
    // * is C0 continuous
    // * is Non-self-intersecting
    // * is Non-zero length
    // * can be open or closed
    // * is trimmable and splittable
    // * may be extensible (evaluable outside of the range [0, 1]

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
    fn trimmed(&self, start: f64, end: f64) -> Result<Edge>;

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
}

impl std::fmt::Debug for dyn GenericEdge {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        write!(fmt, "GenericEdge()")
    }
}

#[derive(Debug)]
pub struct Segment {
    // Implements a segment parameterized as A + B * t where t ranges from 0 to 1
    a: Vec3, // First point
    b: Vec3, // Second point - First point
}

impl Segment {
    fn check(&self) -> Result<()> {
        let v1 = Vertex::new(self.a)?;
        let v2 = Vertex::new(self.a + self.b)?;
        v1.check_vertex_separation(&v2)?;

        Ok(())
    }

    pub fn new(pt1: Vertex, pt2: Vertex) -> Result<Segment> {
        let result = Segment {
            a: pt1.point(),
            b: pt2.point() - pt1.point()
        };

        result.check()?;

        Ok(result)
    }
}

impl GenericEdge for Segment {
    // Evaluate the edge at the given parameter value
    fn d0(&self, t: f64) -> Vec3 {
        assert!(self.parameter_within_bounds(t));

        self.a + t * self.b
    }

    // First derivative with respect to parameter value t
    fn d1(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(self.b)
    }

    // Second derivative with respect to parameter value t
    fn d2(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(Vec3::ZERO)
    }

    fn is_closed(&self) -> bool {
        false
    }

    fn parameter_bounds(&self) -> (Option<f64>, Option<f64>) {
        (None, None)
    }

    fn trimmed(&self, start: f64, end: f64) -> Result<Edge> {
        assert!(self.parameter_within_bounds(start));
        assert!(self.parameter_within_bounds(end));

        let v1 = Vertex::new(self.d0(start))?;
        let v2 = Vertex::new(self.d0(end))?;
        Ok(Edge::Segment(Segment::new(v1, v2)?))
    }
}

#[derive(Debug)]
pub struct CubicNURBSCurve<'a> {
    points: &'a Vec<Vec4>,
    knots: &'a Vec<f64>,
    start: f64,
    end: f64,
}

impl CubicNURBSCurve<'_> {
    fn check(&self) -> Result<()> {

        // Check that there are at least two points
        if self.points.len() < 2 {
            return Err(Error::DegenerateCurve);
        }

        // Check that there are the right number of knots
        if self.knots.len() != self.points.len() + 4 {
            return Err(Error::InvalidParameters);
        }

        // Check that knots vector is non-decreasing
        let mut knots_iter = self.knots.iter();
        let prev = knots_iter.next().expect("Empty knots vector");
        for knot in knots_iter {
            if knot < prev {
                return Err(Error::InvalidParameters);
            }
        }

        // Check that the knots vector has non-zero span
        if self.knots.last().unwrap() - self.knots.first().unwrap() < limits::MINIMUM_PARAMETER_SEPARATION {
            return Err(Error::DegenerateCurve);
        }

        // Check that the bounds are between 0 and 1 and non-zero span
        if self.end - self.start < limits::MINIMUM_PARAMETER_SEPARATION || self.start < 0. || self.end > 1. {
            return Err(Error::InvalidParameters);
        }

        // TODO: check for C0 discontinuities
        // TODO: check for self-intersections

        Ok(())
    }

    pub fn new<'a>(points: &'a Vec<Vec4>, knots: &'a Vec<f64>, start: f64, end: f64) -> Result<CubicNURBSCurve<'a>> {
        let result = CubicNURBSCurve {
            points: points,
            knots: knots,
            start: start,
            end: end,
        };

        result.check()?;

        // TODO: check endpoint separation
        // TODO: check self-intersection and degeneracy
        Ok(result)
    }

    fn t_to_u(&self, t: f64) -> f64 {
        // t is the normalized parameter passed in to the Edge
        // u is the spline parameter
        // for t in [0, 1] u will be in [start, end]
        (t - self.start) / (self.end - self.start)
    }

    // Returns the knot span that the given parameter lies within
    // parameter value u lies falls within [span, span+1)
    fn find_span(&self, u: f64) -> usize {
        // See "The NURBS Book", page 68, algorithm A2.1

        let mut low = 3;
        let mut high = self.knots.len() - 4;

        if u >= self.knots[high] {
            return high - 1;
        }

        if u <= self.knots[low] {
            return low;
        }

        // Binary search
        let mut mid = (low + high) / 2;

        while u < self.knots[mid] || u >= self.knots[mid + 1] {
            if u < self.knots[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        mid
    }

    // Returns the values of the four basis functions evaluated at point u
    fn calc_basis_functions(&self, span: usize, u: f64) -> [f64; 4] {

        // See "The NURBS Book", page 70, algorithm A2.2
        let mut result = [1., 0., 0., 0.];
        let mut left = [0., 0., 0.];
        let mut right = [0., 0., 0.];

        for i in 1..4 {
            left[i - 1] = u - self.knots[span + 1 - i];
            right[i - 1] = self.knots[span + i] - u;

            let mut saved = 0.;

            for r in 0..i {
                let rv = right[r];
                let lv = left[i - 1 - r];
                assert!((rv + lv).abs() >= limits::EPSILON_PARAMETER);
                let temp = result[r] / (rv + lv);
                result[r] = saved + rv * temp;
                saved = lv * temp;
             }

             result[i] = saved;
         }

         return result;
    }
}

impl GenericEdge for CubicNURBSCurve<'_> {
    // Evaluate the edge at the given parameter value
    fn d0(&self, t: f64) -> Vec3 {
        assert!(self.parameter_within_bounds(t));

        // See "The NURBS Book", page 82, algorithm A3.1
        let u = self.t_to_u(t);
        let span = self.find_span(u);
        let basis_functions = self.calc_basis_functions(span, u);

        let mut result = Vec3::ZERO;

        for i in 0..4 {
            let point = self.points[span - 3 + i];
            let basis_function = basis_functions[i];
            result = result + point.xyz() * basis_function * point.w;
        }

        result
    }

    // First derivative with respect to parameter value t
    fn d1(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(Vec3::ZERO) // XXX
    }

    // Second derivative with respect to parameter value t
    fn d2(&self, t: f64) -> Option<Vec3> {
        assert!(self.parameter_within_bounds(t));

        Some(Vec3::ZERO)
    }

    fn parameter_bounds(&self) -> (Option<f64>, Option<f64>) {
        let width = self.end - self.start;
        assert!(width >= limits::EPSILON_PARAMETER);
        (Some(self.start / width), Some(1. - (1. - self.end) / width))
    }

    fn trimmed(&self, start: f64, end: f64) -> Result<Edge> {
        assert!(self.parameter_within_bounds(start));
        assert!(self.parameter_within_bounds(end));

        let result = CubicNURBSCurve::new(self.points, self.knots, self.t_to_u(start), self.t_to_u(end))?;
        Ok(Edge::CubicNURBSCurve(result))
    }
}

/*
#[derive(Debug)]
pub struct Arc {
    // Implements a circular arc parameterized as C + A cos(theta) + B sin(theta)
    // a and b are radii of the circle, and are perpendicular to each other. a points to the start.
    // theta = t * angle
    c: Vec3, // Center point
    a: Vec3, // A radius of the circle pointing to the first point
    b: Vec3, // A radius of the circle perpendicular to a
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
    fn eval(&self, t: f64) -> Vec3 {
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
        Segment::new(Vertex::new(v).unwrap(),
                     Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(1.5, 0.5, -2.)).unwrap())
        .is_ok());

    assert_eq!(
        Segment::new(Vertex::new(v).unwrap(),
                     Vertex::new(v + limits::MINIMUM_VERTEX_SEPARATION * Vec3::new(0.3, 0.5, -0.8)).unwrap()).unwrap_err(),
        Error::VerticesTooClose);
}

#[test]
fn segment_splitting() {
    let base_segment = Segment::new(Vertex::new(Vec3::new(0.5, 0.9, -0.3)).unwrap(), Vertex::new(Vec3::new(0.3, 0.5, -0.8)).unwrap()).unwrap();

    let seg = base_segment.trimmed(0., 0.5).unwrap().unwrap_segment();
    assert!(base_segment.trimmed(0., 0.5).is_ok());
    assert!(base_segment.trimmed(0., 2.).is_ok());
    assert_eq!(base_segment.trimmed(0., 0.).unwrap_err(), Error::VerticesTooClose);
}
