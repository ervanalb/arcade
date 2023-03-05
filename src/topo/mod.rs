use crate::pga::*;
use crate::vertex::*;
use crate::curve::*;
use crate::surface::*;
use crate::construct::*;
//use crate::global::*;

pub type VertexIndex = usize;
pub type EdgeIndex = usize;
pub type FaceIndex = usize;
pub type SolidIndex = usize;

pub type CurveIndex = usize;
pub type SurfaceIndex = usize;

pub type TopoResult<T> = Result<T, TopoError>;

#[derive(Debug,Clone,Default)]
pub struct Topo {
    // Memory arena representing objects with up to 3 dimensions
    pub vertices: Vec<Trivector>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub solids: Vec<Solid>,

    // Underlying geometry
    pub curves: Vec<Curve>,
    pub surfaces: Vec<Surface>,
}

#[derive(Debug,Clone)]
pub enum TopoError {
}

impl Topo {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn assert_valid_vertex_index(&self, ix: VertexIndex) {
        assert!(ix < self.vertices.len(), "Invalid vertex index {:?}", ix);
    }

    pub fn assert_valid_curve_index(&self, ix: VertexIndex) {
        assert!(ix < self.curves.len(), "Invalid curve index {:?}", ix);
    }

    fn push_vertex(&mut self, vertex: Trivector) -> TopoResult<VertexIndex> {
        // TODO consider returning Err if vertex is close but not coincident to another vertex

        let existing_vertex_index = self.vertices.iter().enumerate().find_map(|(i, &existing_vertex)| {
            if vertices_coincident(vertex, existing_vertex) {
                Some(i)
            } else {
                None
            }
        });

        let vertex_index = existing_vertex_index.unwrap_or_else(|| {
            let ix = self.vertices.len();
            self.vertices.push(vertex);
            ix
        });

        Ok(vertex_index)
    }

    // Returns the CurveIndex and a direction
    // indicating whether the returned curve has a reversed travel direction
    // from the provided one
    // (This means that e.g. edge endpoints need to be swapped)
    fn push_curve(&mut self, curve: Curve) -> (CurveIndex, Direction) {
        for (i, existing_curve) in self.curves.iter().enumerate() {
            if let Some(direction) = curves_coincident(&curve, existing_curve) {
                return (i, direction);
            }
        };

        let ix = self.curves.len();
        self.curves.push(curve);
        (ix, Direction::Forward)
    }

    fn push_edge(&mut self, edge: Edge) -> EdgeIndex {
        // This function probably doesn't need to return a Direction
        // because curve consolidation will have already happened,
        // (which means there won't be a coincident but reversed curve)
        // so it's not possible for a coincident Edge to be pushed
        // with the opposite direction of an existing edge.

        let existing_edge_index = self.edges.iter().enumerate().find_map(|(i, existing_edge)| {
            if &edge == existing_edge {
                Some(i)
            } else {
                None
            }
        });

        let edge_index = existing_edge_index.unwrap_or_else(|| {
            let ix = self.edges.len();
            self.edges.push(edge);
            ix
        });

        edge_index
    }

    /// Topology representing just a single vertex
    pub fn vertex(vertex: Trivector) -> Self {
        Topo {
            vertices: vec![vertex],
            ..Default::default()
        }
    }

    /// Topology representing an edge and its endpoint vertices, if it has any
    pub fn edge(curve: Curve, bounds: Option<(Trivector, Trivector)>) -> TopoResult<Self> {
        // TODO check that curve is not self-intersecting and is regular??
        // (or is that part of `Curve`?)

        let mut topo = Topo::empty();

        let (curve_index, direction) = topo.push_curve(curve);

        let bounds = bounds.map(|(start_pt, end_pt)|
            Ok(EdgeEndpoints::new_with_direction(
                topo.push_vertex(start_pt)?,
                topo.push_vertex(end_pt)?,
                direction,
            ))
        ).transpose()?;

        topo.push_edge(Edge {
            curve: curve_index,
            bounds,
        });

        Ok(topo)
    }

    /// Convenience function for making edges that are line segments
    pub fn line_segment_from_two_points(start: Trivector, end: Trivector) -> TopoResult<Self> {
        Self::edge(line_from_two_points(start, end), Some((start, end)))
    }

    /// Convenience function for making edges that are circular arcs defined by 3 points
    pub fn circular_arc_from_three_points(start: Trivector, middle: Trivector, end: Trivector) -> TopoResult<Self> {
        Self::edge(circle_from_three_points(start, middle, end), Some((start, end)))
    }

    //pub fn with_reflection(mut self, selection: &TopoSelection, plane: Vector) -> TopoResult<(Self, TopoSelection>) {
    //    // Adds a reflected copy of the given selection

    //    let reflected_vertices: Vec<Vertices> = selection.vertices.iter().map(|&i| {
    //        self.assert_valid_vertex_index(i);
    //        self.vertices[i].reflect(plane)
    //    }).collect();

    //    let reflected_edges: Vec<Edges> = selection.edges.iter().map(|&i| {
    //        let Edge {
    //            curve,
    //            bounds,
    //        } = self.edges[i];

    //        let curve = curve.reflect(plane);
    //        let bounds = bounds.map(|EdgeEndpoints {start, end}| {
    //            self.assert_valid_vertex_index(start);
    //            self.assert_valid_vertex_index(end);
    //            let start = self.vertices[start].reflect(plane)
    //            let end = self.vertices[end].reflect(plane)
    //            EdgeEndpoints {start, end};
    //        });

    //        // To reflect an edge, we must also reflect its vertices and cuves
    //    }).collect();

    //    // TODO implement reflection for faces and surfaces

    //    TopoSelection {
    //        vertices: reflected_vertices,
    //        edges: reflected_edges,
    //        ..Default::default(),
    //    }

    //    Ok((topo, selection))
    //}
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct EdgeEndpoints {
    // Inner struct for Edge.
    // Contains the indices of the start & end points
    pub start: VertexIndex,
    pub end: VertexIndex,
}

impl EdgeEndpoints {
    pub fn new_with_direction(start: VertexIndex, end: VertexIndex, direction: Direction) -> Self {
        match direction {
            Direction::Forward => EdgeEndpoints {start, end},
            Direction::Reverse => EdgeEndpoints {start: end, end: start},
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Edge {
    // An edge is a section of a curve.
    // If bounds is None, then the curve must be closed / periodic (e.g. a circle.)
    // If bounds.start == bounds.end, then the curve is closed via one point (e.g. a teardrop.)
    pub curve: CurveIndex,
    pub bounds: Option<EdgeEndpoints>,
}

impl Edge {
    pub fn remap(&self, vertex_remap: &[VertexIndex], curve_remap: &[(CurveIndex, Direction)]) -> (Edge, Direction) {
        let (curve, direction) = curve_remap[self.curve];
        (Edge {
            curve,
            bounds: self.bounds.as_ref().map(|endpoints| {
                EdgeEndpoints::new_with_direction(
                    vertex_remap[endpoints.start],
                    vertex_remap[endpoints.end],
                    direction,
                )
            }),
        }, direction)
    }
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

//#[derive(Debug,Clone)]
//pub struct TopoSelection<'a> {
//    // A subset of the entites within a Topo
//
//    pub topo: &'a Topo,
//
//    pub vertices: BTreeSet<VertexIndex>,
//    pub edges: BTreeSet<EdgeIndex>,
//    pub faces: BTreeSet<FaceIndex>,
//    pub solids: BTreeSet<SolidIndex>,
//}
//
//impl TopoSelection<'_> {
//    fn empty(topo: &Topo) -> TopoSelection {
//        TopoSelection {
//            topo,
//            vertices: Default::default(),
//            edges: Default::default(),
//            faces: Default::default(),
//            solids: Default::default(),
//        }
//    }
//
//    pub fn add_vertex(&mut self, ix: VertexIndex) {
//        self.topo.vertices[ix]; // Perform bounds check
//        self.vertices.insert(ix);
//    }
//
//    pub fn add_edge(&mut self, ix: EdgeIndex) {
//        let edge = &self.topo.edges[ix];
//        self.edges.insert(ix);
//        match &edge.bounds {
//            Some(bounds) => {
//                self.add_vertex(bounds.start);
//                self.add_vertex(bounds.end);
//            },
//            None => {},
//        }
//    }
//
//    pub fn add_face(&mut self, ix: FaceIndex) {
//        let face = &self.topo.faces[ix];
//        self.faces.insert(ix);
//        for bound_loop in &face.bounds {
//            for element in &bound_loop.elements {
//                self.add_edge(element.edge);
//            }
//        }
//
//        for &ridge in &face.ridges {
//            self.add_edge(ridge);
//        }
//
//        for &peak in &face.peaks {
//            self.add_vertex(peak);
//        }
//    }
//}

mod op;
pub use op::*;
