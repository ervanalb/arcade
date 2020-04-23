use crate::edge::Edge;
use crate::types::{Vec3, Mat3xN};
use crate::geometry::BaseSegment;
use crate::limits;

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

fn interpolate_adaptive_recursively<T: Edge>(edge: &T, max_error: f64, result: &mut Vec<Vec3>, t_start: f64, pt_start: Vec3, t_end: f64, pt_end: Vec3, level: usize) {
    assert!(level <= limits::MAX_SUBDIVISION);

    println!("Interpolate section: {} to {}", t_start, t_end);

    let seg = BaseSegment {
        a: pt_start,
        b: pt_end - pt_start,
    };

    println!("{} to {}", pt_start, pt_end);

    let bounding_points = edge.spatial_bounding_points(t_start, t_end);
    println!("Bounding points: {}", bounding_points);
    let mut bounding_points_iter = bounding_points.column_iter();
    let mut max_dist = seg.point_dist_to_segment(&bounding_points_iter.next().unwrap().into());
    for pt in bounding_points_iter {
        let dist = seg.point_dist_to_segment(&pt.into());
        if dist > max_dist {
            max_dist = dist;
        }
    }
    println!("Max dist is: {}", max_dist);
    if max_dist <= max_error {
        result.push(pt_end);
    } else {
        let t_mid = 0.5 * (t_start + t_end);
        let pt_mid = edge.d0(t_mid);
        interpolate_adaptive_recursively(edge, max_error, result, t_start, pt_start, t_mid, pt_mid, level + 1);
        interpolate_adaptive_recursively(edge, max_error, result, t_mid, pt_mid, t_end, pt_end, level + 1);
    }
}

pub fn interpolate_edge_adaptive<T: Edge>(edge: &T, max_error: f64) -> Mat3xN {
    // Interpolates points on an edge.
    // These points approximate the edge with an error of less than max_error at every point.
    // The first and last points always occur at t=0 and t=1.
    // The result is a 3 x n matrix of the interpolated points.
    // Precondition: n >= 2.

    let pt0 = edge.d0(0.);

    let mut result = Vec::<Vec3>::new();
    result.push(pt0);
    interpolate_adaptive_recursively(edge, max_error, &mut result, 0., pt0, 1., edge.d0(1.), 0);

    println!("Interpolated curve into {} points", result.len());

    Mat3xN::from_columns(&result)
}
