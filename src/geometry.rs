use crate::limits;
use crate::types::{Vec3, Vec4, VecN, Mat4xN, Mat3xN};
use nalgebra as na;

pub fn weights_to_homo(points: &Mat4xN) -> Mat4xN {
    let mut homo_points = points.clone();
    for mut col in homo_points.column_iter_mut() {
        col.component_mul_assign(&Vec4::new(col[3], col[3], col[3], 1.));
    }
    homo_points
}

pub fn homo_to_weights(points: &Mat4xN) -> Mat4xN {
    let mut weighted_points = points.clone();
    for mut col in weighted_points.column_iter_mut() {
        let inv_w = 1. / col[3];
        col.component_mul_assign(&Vec4::new(inv_w, inv_w, inv_w, 1.));
    }
    weighted_points
}

#[derive(Debug)]
#[derive(Clone)]
pub struct BaseSegment {
    pub a: Vec3, // First point
    pub b: Vec3, // Second point - First point
}

impl BaseSegment {
    pub fn curve_point(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }

    pub fn point_dist_to_line(&self, point: &Vec3) -> f64 {
        // Returns the distance from a point to the infinite line containing this segment.

        // Calculate the parallelogram area using the cross product,
        // and then divide by the base length to get its height.
        let c = point - self.a;
        self.b.cross(&c).norm() / self.b.norm()
    }

    pub fn closest_t_to_point(&self, point: &Vec3) -> f64 {
        // Simply project vector C (which goes from A to point) onto vector B
        let c = point - self.a;
        self.b.dot(&c)
    }

    pub fn point_dist_to_segment(&self, point: &Vec3) -> f64 {
        // Returns the distance from a point to this segment.
        // (the closest distance may be to one of the endpoints)
        let c = point - self.a;
        let t = self.b.dot(&c);
        if t <= 0. {
            // Closest to A
            return c.norm();
        } else if t >= 1. {
            // Closest to A + B
            return (self.b + c).norm();
        }
        // Closest to middle of segment
        (c - self.b * t).norm()
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct BaseCubicNURBSCurve {
    pub points: Mat4xN,
    pub knots: VecN,
}

impl BaseCubicNURBSCurve {
    // Returns the knot span that the given parameter lies within
    // parameter value u falls within [ knots[span], knots[span+1] )
    // with the exception of the top span, which is closed on top: [ knots[span], knots[span+1] ]
    pub fn find_span(&self, u: f64) -> usize {
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
    pub fn calc_basis_functions(&self, span: usize, u: f64) -> [f64; 4] {

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
    pub fn curve_point_homo(&self, u: f64) -> Vec4 {
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
    pub fn curve_point(&self, u: f64) -> Vec3 {
        let homo = self.curve_point_homo(u);
        homo.xyz() / homo[3]
    }

    // Returns a set of points, the convex hull of which bounds the given curve
    pub fn bounding_points(&self) -> Mat3xN {
        homo_to_weights(&self.points).fixed_rows::<na::U3>(0).into()
    }

    // This function returns a curve that matches the given one on the given range,
    // but uses only the necessary knots and control points.
    pub fn coarse_trimmed(&self, start_span: usize, end_span: usize) -> BaseCubicNURBSCurve {
        let first_knot = start_span - 3;
        let last_knot = end_span + 5; // This is actually one after the last knot, so that the knot range is first_cp..last_cp
        let first_cp = start_span - 3;
        let last_cp = end_span + 1; // this is actually one after the last cp, so that the cp range is first_cp..last_cp

        BaseCubicNURBSCurve {
            points: self.points.columns(first_cp, last_cp - first_cp).into(),
            knots: self.knots.rows(first_knot, last_knot - first_knot).into(),
        }
    }

    pub fn insert_knot(&self, u: f64, repeat: usize) -> BaseCubicNURBSCurve {
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
    pub fn trimmed(&self, u_start: f64, u_end: f64) -> BaseCubicNURBSCurve {

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

        // Sanity check
        let weighted_cp = homo_to_weights(&new_points);
        let start_cp = weighted_cp.column(0).xyz();
        let start_diff = self.curve_point(u_start) - start_cp;
        let end_cp: Vec3 = weighted_cp.column(weighted_cp.ncols() - 1).xyz();
        let end_diff: Vec3 = self.curve_point(u_end) - end_cp;
        debug_assert!(start_diff.norm() < limits::EPSILON_VERTEX_COINCIDENT);
        debug_assert!(end_diff.norm() < limits::EPSILON_VERTEX_COINCIDENT);

        BaseCubicNURBSCurve {
            points: new_points,
            knots: new_knots,
        }
    }

    // The smallest valid parameter value, according to the knot vector
    pub fn min_u(&self) -> f64 {
        self.knots[3]
    }

    // The largest valid parameter value, according to the knot vector
    pub fn max_u(&self) -> f64 {
        self.knots[self.knots.len() - 4]
    }
}

