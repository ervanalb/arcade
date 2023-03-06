/// This module deals with how various geometrc primitives
/// are assembled together, topologically.

use crate::pga::*;
use crate::vertex::*;
use crate::curve::*;
use crate::surface::*;
use crate::construct::*;
use crate::global::*;

pub type VertexIndex = usize;
pub type EdgeIndex = usize;
pub type FaceIndex = usize;
pub type SolidIndex = usize;

pub type CurveIndex = usize;
pub type SurfaceIndex = usize;

pub type TopoResult<T> = Result<T, TopoError>;

/// This is a memory arena representing objects with up to 3 dimensions.
/// It includes:
/// * A set of vertices
/// * A set of edges, which may connect vertices via subsets of curves
/// * Faces, which may connect edges via subsets of surfaces
/// * Solids, which are built up from faces that may share edges
///
/// It is meant to be lightweight and immutable.
/// One example use is to construct two Topo objects
/// each containing just a single edge,
/// and combine them together.
#[derive(Debug,Clone,Default)]
pub struct Topo {
    // Underlying reference geometry
    vertices: Vec<Trivector>,
    curves: Vec<Curve>,
    surfaces: Vec<Surface>,

    // Connectivity information
    edges: Vec<Edge>,
    faces: Vec<Face>,
    solids: Vec<Solid>,
}

#[derive(Debug,Clone)]
pub enum TopoError {
}

impl Topo {
    pub fn vertices(&self) -> &[Trivector] {
        &self.vertices
    }

    pub fn curves(&self) -> &[Curve] {
        &self.curves
    }

    pub fn surfaces(&self) -> &[Surface] {
        &self.surfaces
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    pub fn solids(&self) -> &[Solid] {
        &self.solids
    }

    /// Empty topology, containing no geometry
    pub fn empty() -> Self {
        Default::default()
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

    fn push_other_vertex(&mut self, other: &Topo, vertex: VertexIndex) -> TopoResult<VertexIndex> {
        self.push_vertex(other.vertices[vertex])
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

    fn push_other_curve(&mut self, other: &Topo, curve: CurveIndex) -> TopoResult<(CurveIndex, Direction)> {
        self.push_curve(other.curves[curve])
    }

    // Returns the CurveIndex and a direction
    // indicating whether the returned curve has a reversed "sense"
    // (i.e. either U or V was flipped, but not both, since that would amount to a 180 degree rotation)
    fn push_surface(&mut self, surface: Surface) -> TopoResult<(SurfaceIndex, Direction)> {
        for (i, existing_surface) in self.curves.iter().enumerate() {
            if let Some(direction) = surfaces_coincident(&surface, existing_surface) {
                return (i, direction);
            }
        };

        let ix = self.surfaces.len();
        self.surfaces.push(surface);
        (ix, Direction::Forward)
    }

    fn push_other_surface(&mut self, other: &Topo, surface: SurfaceIndex) -> TopoResult<(SurfaceIndex, Direction)> {
        self.push_surface(other.surfaces[surface])
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

    // Push an edge from another topo to this topo, along with any dependent geometry like curves and vertices
    fn push_other_edge(&mut self, other: &Topo, edge: EdgeIndex) -> TopoResult<(EdgeIndex, Direction)> {
        let Edge { curve, bounds } = other.edges[edge];
        let (curve, direction) = self.push_other_curve(curve)?;
        let bounds = bounds.map(|EdgeEndpoints { start, end }| EdgeEndpoints::new_with_direction(
            self.push_other_vertex(start),
            self.push_other_vertex(end),
            direction
        ));
        let edge = self.push_edge(Edge { curve, bounds });
        Ok((edge, direction))
    }

    fn push_face(&mut self, face: Face) -> FaceIndex {
        // This function probably doesn't need to return a Direction
        // because surface consolidation will have already happened,
        // (which means there won't be a coincident but reversed surface)
        // so it's not possible for a coincident Face to be pushed
        // with the opposite direction of an existing Face.

        let existing_face_index = self.faces.iter().enumerate().find_map(|(i, existing_face)| {
            if &face == existing_face {
                Some(i)
            } else {
                None
            }
        });

        let face_index = existing_face_index.unwrap_or_else(|| {
            let ix = self.faces.len();
            self.faces.push(face);
            ix
        });

        face_index
    }

    // Push a face from another topo to this topo, along with any dependent geometry like edges, curves, surfaces, and vertices
    fn push_other_face(&mut self, face: Face) -> FaceIndex {
        todo!();
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
        Self::edge(Curve::line_from_two_points(start, end), Some((start, end)))
    }

    /// Convenience function for making edges that are circular arcs defined by 3 points
    pub fn circular_arc_from_three_points(start: Trivector, middle: Trivector, end: Trivector) -> TopoResult<Self> {
        Self::edge(Curve::circle_from_three_points(start, middle, end), Some((start, end)))
    }

    /// Find and return all possible loops
    fn possible_loops(&self) -> Vec<Loop> {
        // Build up a map of edge connectivity
        let mut directed_edges = Vec::<DirectedEdge>::new();
        // First, push all edges to this map, in both directions
        for edge in 0..self.edges.len() {
            for direction in [Direction::Forward, Direction::Reverse] {
                directed_edges.push(DirectedEdge {
                    edge,
                    direction,
                });
            }
        }
        // Second, for each directed edge, see what other directed edges are possible "next steps"
        // Store this adjacency information in a parallel array
        let connectivity: Vec<Vec<usize>> = directed_edges.iter().map(|directed_edge| {
            if let Some(end_vertex_index) = self.edges[directed_edge.edge].bounds.as_ref().map(|endpoints| endpoints.end_with_direction(directed_edge.direction)) {
                let next_edges: Vec<usize> = directed_edges.iter().enumerate().filter_map(|(i, next_directed_edge)| {
                    let next_start_vertex_index = self.edges[next_directed_edge.edge].bounds.as_ref().map(|endpoints| endpoints.start_with_direction(next_directed_edge.direction))?;
                    if directed_edge.edge != next_directed_edge.edge && end_vertex_index == next_start_vertex_index {
                        Some(i)
                    } else {
                        None
                    }
                }).collect();
                next_edges
            } else {
                // Closed periodic edges don't connect to other edges
                Vec::<usize>::new()
            }
        }).collect();

        // Now that we have this map, find cycles in the graph using DFS
        // and a white-grey-black node coloring approach.
        // https://stackoverflow.com/a/62971341

        #[derive(Debug,Clone,Copy)]
        enum Color {
            White, // Node is not visited
            Grey,  // Node is on the path that is being explored
            Black, // Node is visited
        }

        // Push all nodes onto the stack for DFS
        let mut stack: Vec<usize> = (0..directed_edges.len()).collect();
        let mut color: Vec<Color> = vec![Color::White; directed_edges.len()];

        let mut cycles = Vec::<Vec<usize>>::new();

        while let Some(&n) = stack.last() {
            match color[n] {
                Color::White => {
                    color[n] = Color::Grey;
                    for &m in connectivity[n].iter() {
                        match color[m] {
                            Color::White => {
                                stack.push(m);
                            }
                            Color::Grey => {
                                // This edge creates a cycle.

                                // We will find every cycle twice (forwards and backwards.)
                                // We can safely discard the redundant half of the cycles
                                // by only keeping ones that start with an edge
                                // in the forward direction.
                                match directed_edges[m].direction {
                                    Direction::Forward => {
                                        let cycle_start = stack.len() - 1 - stack.iter().rev().position(|&i| i == m).unwrap();
                                        let cycle: Vec<usize> = stack[cycle_start..].to_vec();
                                        cycles.push(cycle);
                                    }
                                    Direction::Reverse => {}
                                }
                            }
                            Color::Black => {
                                // Already visited; no action necessary
                            }
                        }
                    }
                }
                Color::Grey => {
                    color[n] = Color::Black;
                    stack.pop();
                }
                Color::Black => {
                    // Some of the original nodes that were pushed will become colored black,
                    // so we can ignore them as they have been explored already
                    stack.pop();
                }
            }
        }

        cycles.iter().map(|cycle| Loop {
            elements: cycle.iter().map(|&i| directed_edges[i].clone()).collect()
        }).collect()
    }
}

/// Inner struct for Edge.
/// Contains the indices of the start & end points
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct EdgeEndpoints {
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

    pub fn start_with_direction(&self, direction: Direction) -> VertexIndex {
        match direction {
            Direction::Forward => self.start,
            Direction::Reverse => self.end,
        }
    }

    pub fn end_with_direction(&self, direction: Direction) -> VertexIndex {
        match direction {
            Direction::Forward => self.end,
            Direction::Reverse => self.start,
        }
    }
}

/// An edge is a section of a curve.
/// If bounds is None, then the curve must be closed / periodic (e.g. a circle.)
/// If bounds.start == bounds.end, then the curve is closed via one point (e.g. a teardrop.)
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Edge {
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

/// Inner struct for Loop.
/// Includes the edge index, and the direction it is being used in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectedEdge {
    pub edge: EdgeIndex,
    pub direction: Direction,
}

/// Inner struct for Face.
/// A loop is a closed set of edges, to be used as a boundary for a face.
/// The edges are listed in-order and with a consistent winding direction
/// such that the face lies to the right of the loop.
#[derive(Debug, Clone, Eq)]
pub struct Loop {
    pub elements: Vec<DirectedEdge>,
}

impl PartialEq for Loop {
    fn eq(&self, other: &Self) -> bool {
        false // XXX TODO
    }
}

/// A face is a section of a surface.
/// If the bounds are empty, then the surface must be closed / periodic (e.g. a sphere.)
/// Otherwise, these loops should bound the surface
/// (e.g. trace a perimeter around the outside, along with any holes.)
/// "Ridges" contains a list of edges that trace any 1D discontinuities in the surface normals.
/// "Peaks" contains a list of points that mark any 0D discontinuities in the surface normals.
#[derive(Debug, Clone, Eq)]
pub struct Face {
    pub surface: SurfaceIndex,
    pub bounds: Vec<Loop>,
    //pub ridges: Vec<EdgeIndex>,
    //pub peaks: Vec<VertexIndex>,
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        false // XXX TODO
    }
}

/// Inner struct for Shell.
/// Includes the face index, and the direction it is being used in.
#[derive(Debug,Clone)]
pub struct DirectedFace {
    pub face: FaceIndex,
    pub direction: Direction,
}

/// Inner struct for Solid
/// A shell is a closed (manifold) set of faces, to be used as the boundary for a solid.
/// The faces must be oriented with a consistent winding direction
/// such that the solid always lies on the positive side of the surface.
#[derive(Debug,Clone)]
pub struct Shell {
    pub elements: Vec<DirectedFace>,
}

/// A solid is a region of space bounded by shells.
/// A non-hollow solid will contain just a single shell.
/// A solid with one or more internal voids will have two or more shells.
#[derive(Debug,Clone)]
pub struct Solid {
    pub bounds: Vec<Shell>,
}

/// Given an Edge, returns the t_min and t_max of its underlying curve
fn curve_bounds_for_edge(topo: &Topo, edge: EdgeIndex) -> (Float, Float) {
    let Edge { curve, bounds } = &topo.edges[edge];
    let c = &topo.curves[*curve];
    match bounds {
        Some(endpoints) => {
            let start_pt = topo.vertices()[endpoints.start];
            let end_pt = topo.vertices()[endpoints.end];
            let start_t = c.t_first(start_pt);
            let end_t = c.t_last(end_pt);
            (start_t, end_t)
        },
        None => {
            assert!(c.closed(), "The curve must be closed");
            let start_t = c.t_min().unwrap();
            let end_t = c.t_max().unwrap();
            (start_t, end_t)
        }
    }
}

// High-level operations
mod op;
pub use op::*;
