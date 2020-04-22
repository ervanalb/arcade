use crate::vertex::Vertex;
use crate::error::Error;
use crate::error::Result;
use crate::types::{Vec3, Vec4, VecN, Mat4xN, Mat3xN};
use crate::limits;
use nalgebra::U3;

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

    pub fn new(pt1: &Vertex, pt2: &Vertex) -> Result<Segment> {
        let result = Segment {
            a: pt1.point(),
            b: pt2.point() - pt1.point()
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
}

#[derive(Debug)]
#[derive(Clone)]
struct BaseCubicNURBSCurve {
    points: Mat4xN,
    knots: VecN,
}

impl BaseCubicNURBSCurve {
    // Returns the knot span that the given parameter lies within
    // parameter value u lies falls within [span, span+1)
    // with the exception of the top span, which is closed on top: [span, span+1]
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

    // Returns a point on the curve at parameter u in homogeneous coordinates
    fn curve_point_homo(&self, u: f64) -> Vec4 {
        // See "The NURBS Book", page 82, algorithm A3.1
        let span = self.find_span(u);
        let basis_functions = self.calc_basis_functions(span, u);

        let mut result = Vec4::zeros();

        for i in 0..4 {
            let point = self.points.column(span + i - 3);
            let basis_function = basis_functions[i];
            result = result + point * basis_function;
        }

        result
    }

    // Returns a point on the curve at parameter u in 3D
    fn curve_point(&self, u: f64) -> Vec3 {
        let homo = self.curve_point_homo(u);
        homo.xyz() / homo[3]
    }

    // Returns a set of points, the convex hull of which bounds the given curve
    fn bounding_points(&self) -> Mat3xN {
        self.points.fixed_rows::<U3>(0).into()
    }

    // This function returns a curve that matches the given one on the given range,
    // but uses only the necessary knots and control points.
    fn coarse_trimmed(&self, start_span: usize, end_span: usize) -> BaseCubicNURBSCurve {
        let first_knot = start_span - 3;
        let last_knot = end_span + 5; // This is actually one after the last knot, so that the knot range is first_cp..last_cp
        let first_cp = start_span - 3;
        let last_cp = end_span + 1; // this is actually one after the last cp, so that the cp range is first_cp..last_cp

        BaseCubicNURBSCurve {
            points: self.points.columns(first_cp, last_cp - first_cp).into(),
            knots: self.knots.rows(first_knot, last_knot - first_knot).into(),
        }
    }

    fn insert_knot(&self, u: f64, repeat: usize) -> BaseCubicNURBSCurve {
        // See "The NURBS Book", page 151, algorithm A5.1

        assert!(repeat > 0);

        let mut new_points = Mat4xN::zeros(self.points.ncols() + repeat);
        let mut new_knots = VecN::zeros(self.knots.len() + repeat);
        let span = self.find_span(u);
        // Load new knot vector
        new_knots.rows_mut(0, span + 1).copy_from(&self.knots.rows(0, span + 1));
        new_knots.rows_mut(span + 1, repeat).copy_from(&VecN::repeat(repeat, u));
        new_knots.rows_mut(span + 1 + repeat, new_knots.len() - (span + 1 + repeat)).copy_from(&self.knots.rows(span + 1, self.knots.len() - (span + 1)));

        // Save unaltered control points
        new_points.columns_mut(0, span - 2).copy_from(&self.points.columns(0, span - 2));
        new_points.columns_mut(span + repeat, new_points.ncols() - (span + repeat)).copy_from(&self.points.columns(span, self.points.ncols() - span));

        // Create temporary Rw vector
        let mut temp_points: Mat4xN = self.points.columns(span - 3, 4).into();

        for j in 0..repeat {
            let l = span - 2 + j;
            for i in 0..(3 - j) {
                let alpha = (u - self.knots[l + i]) / (self.knots[i + span + 1] - self.knots[l + i]);
                let new_pt = alpha * temp_points.column(i + 1) + (1. - alpha) * temp_points.column(i);
                temp_points.column_mut(i).copy_from(&new_pt);
            }
            new_points.column_mut(l).copy_from(&temp_points.column(0));
            new_points.column_mut(span + repeat - j - 1).copy_from(&temp_points.column(2 - j));
        }
        let l = span + repeat - 3;
        if span > l + 1 {
            new_points.columns_mut(l + 1, span - l - 1).copy_from(&temp_points.columns(1, span - l - 1));
        }

        BaseCubicNURBSCurve {
            points: new_points,
            knots: new_knots,
        }
    }

    // Returns a new curve on the interval u_start to u_end
    // with only the necessary knots and control points
    fn trimmed(&self, u_start: f64, u_end: f64) -> BaseCubicNURBSCurve {

        let start_span = self.find_span(u_start);
        let end_span = self.find_span(u_end);
        let temp_curve = self.coarse_trimmed(start_span, end_span);
        let temp_curve = temp_curve.insert_knot(u_start, 3);
        let temp_curve = temp_curve.insert_knot(u_end, 3);
        let start_span = self.find_span(u_start);
        let end_span = self.find_span(u_end);
        let first_knot = start_span + 1;
        let last_knot = end_span + 7; // is actually one past the last knot
        let first_cp = start_span;
        let last_cp = end_span + 4; // is actually one past the last cp

        let mut new_points = Mat4xN::zeros(last_cp - first_cp);
        let mut new_knots = VecN::zeros(last_knot - first_knot + 2);

        new_knots.rows_mut(1, last_knot - first_knot).copy_from(&temp_curve.knots.rows(first_knot, last_knot - first_knot));
        new_knots[0] = u_start;
        new_knots[last_knot - first_knot + 1] = u_end;
        new_points.copy_from(&temp_curve.points.columns(first_cp, last_cp - first_cp));

        BaseCubicNURBSCurve {
            points: new_points,
            knots: new_knots,
        }
    }

    // The smallest valid parameter value, according to the knot vector
    fn min_u(&self) -> f64 {
        self.knots[3]
    }

    // The largest valid parameter value, according to the knot vector
    fn max_u(&self) -> f64 {
        self.knots[self.knots.len() - 4]
    }

}

#[derive(Debug)]
pub struct CubicNURBSCurve {
    curve: BaseCubicNURBSCurve,
    start: f64,
    end: f64,
}

impl CubicNURBSCurve {
    fn check(&self) -> Result<()> {
        // Check that there are at least two curve.points
        if self.curve.points.ncols() < 2 {
            return Err(Error::DegenerateCurve);
        }

        // Check that there are the right number of curve.knots
        if self.curve.knots.len() != self.curve.points.ncols() + 4 {
            return Err(Error::InvalidParameters);
        }

        // Check that curve.knots vector is non-decreasing
        let mut knots_iter = self.curve.knots.iter();
        let mut prev = knots_iter.next().expect("Empty curve.knots vector");
        for knot in knots_iter {
            if knot < prev {
                return Err(Error::InvalidParameters);
            }
            prev = knot;
        }

        // Check that the knots vector has non-zero span
        if self.curve.max_u() - self.curve.min_u() < limits::MINIMUM_PARAMETER_SEPARATION {
            return Err(Error::DegenerateCurve);
        }

        // Check that the bounds are between the curve.knots and are non-zero span
        if self.end - self.start < limits::MINIMUM_PARAMETER_SEPARATION || self.start < self.curve.knots[0] || self.end > self.curve.knots[self.curve.knots.len() - 1] {
            return Err(Error::InvalidParameters);
        }

        // TODO: check for C0 discontinuities
        // TODO: check for self-intersections
        // TODO: check endpoint separation

        Ok(())
    }

    fn weights_to_homo(points: &Mat4xN) -> Mat4xN {
        let mut homo_points = points.clone();
        for mut col in homo_points.column_iter_mut() {
            col.component_mul_assign(&Vec4::new(col[3], col[3], col[3], 1.));
        }
        homo_points
    }

    pub fn new(points: &Mat4xN, knots: &VecN) -> Result<CubicNURBSCurve> {
        // Do the conversion to homogeneous coordinates right away
        let curve = BaseCubicNURBSCurve {
            points: Self::weights_to_homo(points),
            knots: knots.clone(),
        };

        let result = CubicNURBSCurve {
            start: curve.min_u(),
            end: curve.max_u(),
            curve: curve,
        };

        result.check()?;

        Ok(result)
    }

    pub fn insert_knot(&self, t: f64, repeat: usize) -> Result<CubicNURBSCurve> {
        assert!(self.parameter_within_bounds(t));

        let result = CubicNURBSCurve {
            curve: self.curve.insert_knot(self.t_to_u(t), repeat),
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
        self.curve.curve_point(u)
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
        (Some((self.curve.min_u() - self.start) / width), Some(1. - (self.curve.max_u() - self.end) / width))
    }

    fn trimmed(&self, start: f64, end: f64) -> Result<CubicNURBSCurve> {
        assert!(self.parameter_within_bounds(start));
        assert!(self.parameter_within_bounds(end));

        // Bypass the ::new() constructor so that we don't have to convert from/to homogeneous coordinates unnecessarily
        let result = CubicNURBSCurve {
            curve: self.curve.clone(),
            start: self.t_to_u(start),
            end: self.t_to_u(end)
        };
        result.check()?;
        Ok(result)
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
