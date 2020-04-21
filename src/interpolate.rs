use crate::edge::Edge;
use crate::types::Mat3xN;

pub fn interpolate_edge_fixed<T: Edge>(edge: &T, n: usize) -> Mat3xN {
    // Interpolates n points on an edge evenly spaced in the parameter t.
    // The first and last points always occur at t=0 and t=1.
    // The result is a 3 x n matrix of the interpolated points.
    // Precondition: n >= 2.

    assert!(n >= 2);

    let mut result = Mat3xN::zeros(n);

    for i in 0..n {
        let t = i as f64 / (n - 1) as f64;
        let v = edge.d0(t);
        result.column_mut(i).copy_from(&v);
    }

    return result;
}

/*
pub fn interpolate_edge_adaptive<T: Edge>(edge: &T, max_angle: f64, min_length: f64) -> Mat3xN {
    // Interpolates n points on an edge evenly spaced in the parameter t.
    // The first and last points always occur at t=0 and t=1.
    // The result is a 3 x n matrix of the interpolated points.
    // Precondition: n >= 2.

}
*/
