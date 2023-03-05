use crate::pga::*;
//use crate::global::*;
//use crate::curve::*;
use crate::topo::*;

pub fn reflect(mut topo: Topo, plane: Vector) -> Topo {
    // Returns a new Topo containing a reflected copy of the given one

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
