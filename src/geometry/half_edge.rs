use super::constants::{UNSET_VALUE};
use super::{VertexIndex, FaceIndex};

pub struct HalfEdgeIndex {
    pub index: u32,
}

pub struct HalfEdge {
    pub start_vertex: VertexIndex,
    pub adjacent_face: FaceIndex,
    pub next_edge: HalfEdgeIndex,
    pub previous_edge: HalfEdgeIndex
}