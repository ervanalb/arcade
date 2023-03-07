use crate::pga::*;
use crate::global::*;
//use crate::curve::*;
use crate::topo::*;

/// Returns a new Topo containing everything in the old one, reflected across a mirror plane
pub fn reflect(mut topo: Topo, plane: Vector) -> Topo {
    // Since we just reflect everything,
    // we won't need to re-index

    for vertex in topo.vertices.iter_mut() {
        *vertex = vertex.reflect(plane);
    }

    for curve in topo.curves.iter_mut() {
        *curve = curve.reflect(plane);
    }

    // TODO implement .reflect() for surfaces
    //for surface in topo.surfaces.iter_mut() {
    //    *surface = surface.reflect(plane);
    //}

    topo
}

/// Returns a Topo that has all the elements from all input topos.
/// No boolean geometric operations are applied,
/// but coincident geometry will be merged.
pub fn combine(topos: &[Topo]) -> TopoResult<Topo> {

    let mut result_topo = Topo::empty();

    let vertex_index_map = topos.iter().map(|topo| topo.vertices.iter().map(|vertex| result_topo.push_vertex(*vertex)).collect()).collect::<TopoResult<Vec<Vec<VertexIndex>>>>()?;
    let curve_index_map = topos.iter().map(|topo| topo.curves.iter().map(|curve| result_topo.push_curve(curve.clone())).collect()).collect::<Vec<Vec<(CurveIndex, Direction)>>>();

    // TODO push surfaces and generate remap
    //let surface_index_map = topos.iter().map(|topo| topo.surfaces.iter().map(|surface| result_topo.push_surface(surface.clone())).collect()).collect::<Vec<Vec<SurfaceIndex>>>();

    let _edge_index_map = topos.iter().zip(vertex_index_map.iter().zip(curve_index_map.iter())).map(|(topo, (vertex_index_map, curve_index_map))| 
        topo.edges.iter().map(|edge| {
            let (edge, direction) = edge.remap(&vertex_index_map, &curve_index_map);
            let edge_index = result_topo.push_edge(edge);
            (edge_index, direction)
        }).collect()
    ).collect::<Vec<Vec<(EdgeIndex, Direction)>>>();

    // TODO push & remap faces and solids

    Ok(result_topo)
}

/// Returns a Topo representing planar faces constructed from the input topo's edges.
/// Any closed loop of planar edges will be turned into a face.
/// This function will 
pub fn planar_face(mut topo: Topo) -> TopoResult<Topo> {

    // 1. Find edge loops
    let loops = topo.possible_loops();

    // 2. See which loops are planar
    let loop_planes = loops.iter().map(|l| {
        let mut loop_vertices: Vec<usize> = l.elements.iter().map(|DirectedEdge {edge, direction: _}| {
            match topo.edges[*edge].bounds {
                Some(EdgeEndpoints {start, end}) => vec![start, end],
                None => vec![],
            }
        }).flatten().collect();
        loop_vertices.sort();
        loop_vertices.dedup();

        // First, try to define a plane from a set of 3 vertices
        let mut plane: Option<Vector> = None;
        if loop_vertices.len() >= 3 {
            for i in 0..loop_vertices.len() - 3 {
                let pt0 = topo.vertices[loop_vertices[i + 0]];
                let pt1 = topo.vertices[loop_vertices[i + 1]];
                let pt2 = topo.vertices[loop_vertices[i + 2]];
                let test_plane = plane_from_three_points(pt0, pt1, pt2);
                if test_plane.is_finite() {
                    plane = Some(test_plane);
                    break;
                }
            }
        }

        // If that didn't work, then the plane is defined by a curve.
        if plane.is_none() {
            for DirectedEdge {edge, direction: _} in l.elements.iter() {
                let curve = &topo.curves[topo.edges[*edge].curve];
                let (t_start, t_end) = curve_bounds_for_edge(&topo, *edge);
                // Look at a curve's bounding hull to determine if it is planar
                // If a curve has 2 or fewer hull points, then it is a line and doesn't define a plane
                // If a curve has 3 or more hull points, then they must all be planar for the curve to be planar.
                let hull_pts = curve.hull(t_start, t_end);
                for i in 0..hull_pts.len() - 3 {
                    let pt0 = hull_pts[i + 0];
                    let pt1 = hull_pts[i + 1];
                    let pt2 = hull_pts[i + 2];
                    let test_plane = plane_from_three_points(pt0, pt1, pt2);
                    if test_plane.is_finite() {
                        plane = Some(test_plane);
                        break;
                    }
                }
            }
        }
        let plane = plane.expect("Loop is degenerate (0D or 1D)");

        // Now, ensure that all edges lie within the plane.
        for DirectedEdge {edge, direction: _} in l.elements.iter() {
            let curve = &topo.curves[topo.edges[*edge].curve];
            let (t_start, t_end) = curve_bounds_for_edge(&topo, *edge);
            // Look at a curve's bounding hull points to determine if it lies in the plane
            let hull_pts = curve.hull(t_start, t_end);
            for &pt in hull_pts.iter() {
                let distance = (plane & pt).norm();
                if distance > EPSILON_COINCIDENT_DISTANCE {
                    return None; // This loop contains a curve that lies outside the test plane
                }
            }
        }

        // All edges passed the test, so this loop is planar
        Some(plane)
    });

    let loops_and_planes: Vec<(Loop, Vector)> = loops.iter().zip(loop_planes.into_iter()).filter_map(|(l, pl)| Some((l.clone(), pl?))).collect();

    // Remove any loops that are not planar, and construct Plane surfaces for those that are.
    let faces: Vec<FaceIndex> = loops_and_planes.into_iter().map(|(l, pl)| {
        let (surface, _direction) = topo.push_surface(Surface::plane(pl));
        topo.push_face(Face {
            surface,
            bounds: vec![l.clone()],
        })
    }).collect();

    Ok(topo.select(&[], &faces, &[], &[]))
}
