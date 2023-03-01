use crate::pga::*;
//use crate::global::*;
//use crate::curve::*;
use crate::vertex::*;
use crate::topo::*;
use std::mem;

// TODO consider making this function consume Topo3D
pub fn reflect(topo: &Topo3D, plane: Vector) -> Topo3D {
    // Returns a new Topo3D containing a reflected copy of the given one

    let mut result_topo = Topo3D::new();

    // Since we just reflect everything,
    // we won't need to re-index

    for (ix, vertex) in topo.vertices.iter().enumerate() {
        let new_index = result_topo.add_vertex(vertex.reflect(plane));
        assert!(ix == new_index);
    }

    for (ix, edge) in topo.edges.iter().enumerate() {
        let new_index = result_topo.add_edge(edge.clone());
        assert!(ix == new_index);
    }

    for (ix, face) in topo.faces.iter().enumerate() {
        // TODO: do we need to reverse winding directions? Hopefully no
        let new_index = result_topo.add_face(face.clone());
        assert!(ix == new_index);
    }

    for (ix, solid) in topo.solids.iter().enumerate() {
        // TODO: do we need to reverse winding directions? Hopefully no
        let new_index = result_topo.add_solid(solid.clone());
        assert!(ix == new_index);
    }

    for (ix, curve) in topo.curves.iter().enumerate() {
        let new_index = result_topo.add_curve(curve.reflect(plane));
        assert!(ix == new_index);
    }

    // TODO implement .reflect() for surfaces
    //for (ix, surface) in topo.surfaces.iter().enumerate() {
    //    let new_index = result_topo.add_surface(surface.reflect(plane));
    //    assert!(ix == new_index);
    //}

    result_topo
}

pub fn combine(topos: &[Topo3D]) -> Topo3D {
    // Returns a Topo3D that has all the elements from both topos.
    // No simplification is done.

    let mut result_topo = Topo3D::new();

    let vertex_index_map = topos.iter().map(|topo| topo.vertices.iter().map(|vertex| result_topo.add_vertex(*vertex)).collect()).collect::<Vec<Vec<VertexIndex>>>();
    let curve_index_map = topos.iter().map(|topo| topo.curves.iter().map(|curve| result_topo.add_curve(curve.clone())).collect()).collect::<Vec<Vec<CurveIndex>>>();
    let surface_index_map = topos.iter().map(|topo| topo.surfaces.iter().map(|surface| result_topo.add_surface(surface.clone())).collect()).collect::<Vec<Vec<SurfaceIndex>>>();

    let edge_index_map = topos.iter().zip(vertex_index_map.iter().zip(curve_index_map.iter())).map(|(topo, (vertex_index_map, curve_index_map))| topo.edges.iter().map(|edge| result_topo.add_edge(edge.remap(vertex_index_map, curve_index_map))).collect()).collect::<Vec<Vec<EdgeIndex>>>();

    // TODO faces and solids

    result_topo
}

fn simplify_vertices(mut topo: Topo3D) -> Topo3D {
    let old_vertices = mem::replace(&mut topo.vertices, Vec::<Trivector>::new());

    let vertex_remap: Vec<VertexIndex> = old_vertices.iter().map(|&vertex| {
        let find_existing_vertex = topo.vertices.iter().enumerate().find(|(_, &new_vertex)| vertices_coincident(vertex, new_vertex)).map(|(i, _)| i);
        match find_existing_vertex {
            Some(vertex_index) => vertex_index,
            None => topo.add_vertex(vertex),
        }
    }).collect();

    // No curve remap
    let curve_remap: Vec<CurveIndex> = (0..topo.curves.len()).collect();

    // Remap geometry that used these vertices
    for edge in topo.edges.iter_mut() {
        *edge = edge.remap(&vertex_remap, &curve_remap)
    };

    topo
}

pub fn simplify(topo: Topo3D) -> Topo3D {
    // Returns a Topo3D with the following operations applied:
    // * All overlapping vertices are reduced to a single vertex
    // * All overlapping edges are reduced to a single edge (TODO)
    // * All overlapping faces are reduced to a single face (TODO)
    // * All overlapping solids are reduced to a single solid (TODO)

    simplify_vertices(topo)
}
