use super::constants::{UNSET_VALUE};
use super::{VertexIndex, FaceIndex};
use std::ops::{Index, IndexMut};

pub struct HalfEdgeIndex {
    pub index: u32,
}

impl HalfEdgeIndex {
    pub const fn unset() -> HalfEdgeIndex {
        HalfEdgeIndex { index: UNSET_VALUE}
    }

    pub fn new(index: u32) -> HalfEdgeIndex {
        HalfEdgeIndex {index: index}
    }
}

pub struct HalfEdge {
    pub start_vertex: VertexIndex,
    pub adjacent_face: FaceIndex,
    pub next_edge: HalfEdgeIndex,
    pub previous_edge: HalfEdgeIndex
}

impl HalfEdge {
    pub fn unset() -> HalfEdge {
        HalfEdge {
            start_vertex: VertexIndex::unset(),
            adjacent_face: FaceIndex::unset(),
            next_edge: HalfEdgeIndex::unset(),
            previous_edge: HalfEdgeIndex::unset(),
        }
    }

    pub fn new(start_vertex: VertexIndex, adjacent_face: FaceIndex, next_edge: HalfEdgeIndex) -> HalfEdge {
        let mut edge = HalfEdge::unset();
        edge.start_vertex = start_vertex;
        edge.adjacent_face = adjacent_face;
        edge.next_edge = next_edge;

        edge
    }

    pub fn is_unused(&self) -> bool {
        self.start_vertex.index == UNSET_VALUE
    }
}

pub struct HalfEdgeCollection {
    edges: Vec<HalfEdge>
}

impl Index<HalfEdgeIndex> for HalfEdgeCollection {
    type Output = HalfEdge;

    fn index(&self, index: HalfEdgeIndex) -> &Self::Output {
        &self.edges[index.index as usize]
    }
}

impl IndexMut<HalfEdgeIndex> for HalfEdgeCollection {
    fn index_mut(&mut self, index: HalfEdgeIndex) -> &mut Self::Output {
        &mut self.edges[index.index as usize]
    }
}
