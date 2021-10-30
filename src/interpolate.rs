use crate::pga::*;
use crate::global::*;
use crate::curve::Curve;

// Interpolates n points on a curve evenly spaced in the parameter t,
// starting at t_start and ending at t_end.
// The result is a vector of the interpolated points.
// Precondition: t_start and t_end must be valid parameter values for the curve
// (i.e. lie between t_min and t_max, inclusive)
// Precondition: t_end must be greater than t_start unless the curve is closed.
// Precondition: n >= 2.
pub fn interpolate_curve_subset_fixed(c: &Curve, t_start: Float, t_end: Float, n: usize) -> Vec<Trivector> {
    assert!(n >= 2);

    let mut result = vec![Trivector::zero(); n];

    match c.t_min() {
        Some(t_min) => {
            assert!(t_start >= t_min, "t_start must be at least t_min");
            assert!(t_end >= t_min, "t_end must be at least t_min");
        },
        None => {},
    }

    match c.t_max() {
        Some(t_max) => {
            assert!(t_start <= t_max, "t_start must be at most t_max");
            assert!(t_end <= t_max, "t_end must be at most t_max");
        },
        None => {},
    }

    if t_end < t_start {
        // Interpolate through the parameter discontinuity
        // on a closed curve
        assert!(c.closed(), "Unless the curve is closed, t_end must be greater than t_start");
        let t_min = c.t_min().unwrap();
        let t_max = c.t_max().unwrap();
        let new_t_end = t_end + (t_max - t_min);

        for i in 0..n {
            let alpha = i as Float / (n - 1) as Float;
            let t = t_start + alpha * (new_t_end - t_start);
            let t = (t - t_min) % (t_max - t_min) + t_min;
            result[i] = c.d0(t);
        }
    } else {
        for i in 0..n {
            let alpha = i as Float / (n - 1) as Float;
            let t = t_start + alpha * (t_end - t_start);
            result[i] = c.d0(t);
        }
    }

    result
}

// Interpolates n points on a closed curve evenly spaced in the parameter t.
// The result is a vector of the interpolated points.
// Precondition: n >= 2.
// Precondition: the curve is closed.
pub fn interpolate_closed_curve_fixed(c: &Curve, n: usize) -> Vec<Trivector> {
    assert!(n >= 2);
    assert!(c.closed(), "The curve must be closed");

    let mut result = vec![Trivector::zero(); n];

    let t_min = c.t_min().unwrap();
    let t_max = c.t_max().unwrap();

    for i in 0..n {
        let alpha = i as Float / n as Float;
        let t = t_min + alpha * (t_max - t_min);
        result[i] = c.d0(t);
    }

    result
}
