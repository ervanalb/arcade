use crate::pga::*;
use crate::global::*;
use crate::curve::Curve;

// Interpolates n points on a curve evenly spaced in the parameter t.
// The result is a vector of the interpolated points.
// Precondition: n >= 2.
pub fn interpolate_curve_fixed(c: &Curve, n: usize) -> Vec<Trivector> {
    assert!(n >= 2);

    let mut result = vec![Trivector::zero(); n];

    let t_min = c.t_min().expect("Cannot interpolate an unbounded curve");
    let t_max = c.t_max().expect("Cannot interpolate an unbounded curve");

    for i in 0..n {
        let alpha = i as Float / (n - 1) as Float;
        let t = t_min + alpha * (t_max - t_min);
        result[i] = c.d0(t);
    }

    return result;
}
