use crate::global::{Float, EPSILON};
use crate::pga::{Multivector, Point};

// This module implements cubic NURBS curves and surfaces using 3D PGA

pub struct Curve {
    pub points: Vec<Point>,
    pub knots: Vec<Float>,
}

impl Curve {
    pub fn find_span(&self, u: Float) -> usize {
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
    pub fn calc_basis_functions(&self, span: usize, u: Float) -> [Float; 4] {

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
                assert!((rv + lv).abs() >= EPSILON);
                let temp = result[r] / (rv + lv);
                result[r] = saved + rv * temp;
                saved = lv * temp;
             }

             result[i] = saved;
         }

         return result;
    }

    // Returns a point on the curve at parameter u
    pub fn curve_point(&self, u: Float) -> Point {
        // See "The NURBS Book", page 82, algorithm A3.1
        let span = self.find_span(u);
        let basis_functions = self.calc_basis_functions(span, u);

        let mut result = Point::zero();

        for i in 0..4 {
            let point = self.points[span + i - 3];
            let basis_function = basis_functions[i];
            result = result + point * basis_function;
        }

        result
    }

    // Interpolates n points on an edge evenly spaced in the parameter t.
    // The result is a vector of the interpolated points.
    // Precondition: n >= 2.
    pub fn interpolate_fixed(&self, n: usize) -> Vec<Point> {
        assert!(n >= 2);

        let mut result = vec![Point::zero(); n];

        let min_u = self.min_u();
        let max_u = self.max_u();

        for i in 0..n {
            let alpha = i as Float / (n - 1) as Float;
            let u = min_u + alpha * (max_u - min_u);
            let v = self.curve_point(u);
            result[i] = v;
        }

        return result;
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
