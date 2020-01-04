use super::constants::{UNSET_VALUE};
use super::{VertexIndex, FaceIndex, MeshPartCollection, UnsetValue};
use std::ops::{Index, IndexMut};

#[derive(PartialEq)]
pub struct HalfEdgeIndex {
    pub index: u32,
}

impl UnsetValue for HalfEdgeIndex {
    fn unset() -> HalfEdgeIndex {
        HalfEdgeIndex { index: UNSET_VALUE}
    }

    fn is_unset(&self) -> bool {
        *self == HalfEdgeIndex::unset()
    }
}

impl HalfEdgeIndex {
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

impl MeshPartCollection<HalfEdge> for HalfEdgeCollection {
    fn new() -> HalfEdgeCollection {
        HalfEdgeCollection {
            edges: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.edges.len()
    }

    fn add(&mut self, e: HalfEdge) -> usize {
        self.edges.push(e);
        self.len() - 1
    }
}

impl HalfEdgeCollection {
    pub fn edge_pair_index(index: &HalfEdgeIndex) -> HalfEdgeIndex {
        if index.index == UNSET_VALUE {
            HalfEdgeIndex::unset()
        }
        else {
            match index.index % 2 {
                0 => HalfEdgeIndex::new(index.index + 1),
                1 => HalfEdgeIndex::new(index.index - 1),
                _ => HalfEdgeIndex::unset()
            }
        }
    }

    pub fn edge_pair(&self, index: &HalfEdgeIndex) -> Option<&HalfEdge> {
        match index.index >= self.len() as u32 {
            true => Option::None,
            false => {
                let pair_index = HalfEdgeCollection::edge_pair_index(index);
                Option::Some(&self[pair_index])
            }
        }
    }
}
