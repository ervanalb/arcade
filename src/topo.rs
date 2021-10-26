use crate::pga::*;
use crate::curve::*;
use crate::surface::*;
//use crate::global::*;
//use crate::construct::*;

pub type VertexIndex = usize;
pub type EdgeIndex = usize;
pub type FaceIndex = usize;
pub type SolidIndex = usize;

pub type CurveIndex = usize;
pub type SurfaceIndex = usize;

#[derive(Debug,Clone)]
pub enum Direction {
    Forward,
    Reverse,
}

#[derive(Debug,Clone,Default)]
pub struct Arena3D {
    // Memory arena representing objects with up to 3 dimensions
    pub vertices: Vec<Trivector>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub solids: Vec<Solid>,

    pub curves: Vec<Curve>,
    pub surfaces: Vec<Surface>,
}

impl Arena3D {
    pub fn new() -> Arena3D {
        Default::default()
    }

    pub fn assert_valid_vertex_index(&self, ix: VertexIndex) {
        assert!(ix < self.vertices.len(), "Invalid vertex index {:?}", ix);
    }

    pub fn assert_valid_curve_index(&self, ix: VertexIndex) {
        assert!(ix < self.curves.len(), "Invalid curve index {:?}", ix);
    }

    pub fn add_vertex(&mut self, vertex: Trivector) -> VertexIndex {
        let ix = self.vertices.len();
        self.vertices.push(vertex);
        ix
    }

    pub fn add_edge(&mut self, edge: Edge) -> EdgeIndex {
        let ix = self.edges.len();
        self.edges.push(edge);
        ix
    }

    pub fn add_edge_with_endpoints(&mut self, curve: CurveIndex, start: VertexIndex, end: VertexIndex) -> EdgeIndex {
        self.assert_valid_curve_index(curve);
        self.assert_valid_vertex_index(start);
        self.assert_valid_vertex_index(end);

        let ix = self.edges.len();
        self.edges.push(Edge {
            curve,
            bounds: Some(EdgeEndpoints {
                start,
                end,
            }),
        });
        ix
    }

    pub fn add_periodic_edge(&mut self, curve: CurveIndex) -> EdgeIndex {
        self.assert_valid_curve_index(curve);

        let ix = self.edges.len();
        self.edges.push(Edge {
            curve,
            bounds: None,
        });
        ix
    }

    pub fn add_face(&mut self, face: Face) -> FaceIndex {
        let ix = self.faces.len();
        self.faces.push(face);
        ix
    }

    pub fn add_solid(&mut self, solid: Solid) -> SolidIndex {
        let ix = self.solids.len();
        self.solids.push(solid);
        ix
    }

    pub fn add_curve(&mut self, curve: Curve) -> CurveIndex {
        let ix = self.curves.len();
        self.curves.push(curve);
        ix
    }

    pub fn add_surface(&mut self, surface: Surface) -> SurfaceIndex {
        let ix = self.surfaces.len();
        self.surfaces.push(surface);
        ix
    }
}

#[derive(Debug,Clone)]
pub struct EdgeEndpoints {
    // Inner struct for Edge.
    // Contains the indices of the start & end points
    pub start: VertexIndex,
    pub end: VertexIndex,
}

#[derive(Debug,Clone)]
pub struct Edge {
    // An edge is a section of a curve.
    // If bounds is None, then the curve must be closed / periodic (e.g. a circle.)
    // If bounds.start == bounds.end, then the curve is closed via one point (e.g. a teardrop.)
    pub curve: CurveIndex,
    pub bounds: Option<EdgeEndpoints>,
}

#[derive(Debug,Clone)]
pub struct DirectedEdge {
    // Inner struct for Loop.
    // Includes the edge index, and the direction it is being used in.
    pub edge: EdgeIndex,
    pub direction: Direction,
}

#[derive(Debug,Clone)]
pub struct Loop {
    // Inner struct for Face.
    // A loop is a closed set of edges, to be used as a boundary for a face.
    // The edges are listed in-order and with a consistent winding direction
    // such that the face lies to the right of the loop.
    pub elements: Vec<DirectedEdge>,
}

#[derive(Debug,Clone)]
pub struct Face {
    // A face is a section of a surface.
    // If the bounds are empty, then the surface must be closed / periodic (e.g. a sphere.)
    // Otherwise, these loops should bound the surface
    // (e.g. trace a perimeter around the outside, along with any holes.)
    // "Ridges" contains a list of edges that trace any 1D discontinuities in the surface normals.
    // "Peaks" contains a list of points that mark any 0D discontinuities in the surface normals.
    pub surface: SurfaceIndex,
    pub bounds: Vec<Loop>,
    pub ridges: Vec<EdgeIndex>,
    pub peaks: Vec<VertexIndex>,
}

#[derive(Debug,Clone)]
pub struct DirectedFace {
    // Inner struct for Shell.
    // Includes the face index, and the direction it is being used in.
    pub face: FaceIndex,
    pub direction: Direction,
}

#[derive(Debug,Clone)]
pub struct Shell {
    // Inner struct for Solid
    // A shell is a closed (manifold) set of faces, to be used as the boundary for a solid.
    // The faces must be oriented with a consistent winding direction
    // such that the solid always lies on the positive side of the surface.
    pub elements: Vec<DirectedFace>,
}

#[derive(Debug,Clone)]
pub struct Solid {
    // A solid is a region of space bounded by shells.
    pub bounds: Vec<Shell>,
}
