use crate::pga::*;
//use crate::global::*;
//use crate::curve::*;
use crate::topo::*;

pub fn reflect(topo: &Topo3D, plane: Vector) -> Topo3D {
    // Returns a new Topo3D containing a reflected copy of the given selection

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
