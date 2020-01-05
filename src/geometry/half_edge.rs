use super::constants::{UNSET_VALUE};
use super::{VertexIndex, FaceIndex, MeshPartCollection, UnsetValue};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Copy, Clone)]
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

impl MeshPartCollection<HalfEdge, HalfEdgeIndex> for HalfEdgeCollection {
    fn new() -> HalfEdgeCollection {
        HalfEdgeCollection {
            edges: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.edges.len()
    }

    fn add(&mut self, e: HalfEdge) -> HalfEdgeIndex {
        self.edges.push(e);
        HalfEdgeIndex::new((self.len() - 1) as u32)
    }
}

impl HalfEdgeCollection {
    pub fn edge_pair_index(index: HalfEdgeIndex) -> HalfEdgeIndex {
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

    pub fn edge_pair(&self, index: HalfEdgeIndex) -> Option<&HalfEdge> {
        match index.index >= self.len() as u32 {
            true => Option::None,
            false => {
                let pair_index = HalfEdgeCollection::edge_pair_index(index);
                Option::Some(&self[pair_index])
            }
        }
    }

    pub fn end_vertex_index(&self, index: HalfEdgeIndex) -> Option<VertexIndex> {
        let pair = self.edge_pair(index);
        match pair {
            None => None,
            Some(edge) => Option::Some(edge.start_vertex)
        }
    }

    pub fn vertex_circulator(&self, index: HalfEdgeIndex) -> Option<Vec<HalfEdgeIndex>> {
        if index.index >= self.len() as u32 {
            return Option::None;
        }

        let mut early_exit = false;
        let mut cur_edge_index = index;
        let mut edges: Vec<HalfEdgeIndex> = vec![cur_edge_index];
        for i in 0..100 {
            cur_edge_index = self[HalfEdgeCollection::edge_pair_index(cur_edge_index)].next_edge;
            if cur_edge_index == index {
                early_exit = true;
                break;
            }
            if cur_edge_index.is_unset() {
                panic!("Edge index is unset! Cant continue on circulator!");
            }

            edges.push(cur_edge_index);
        }

        if !early_exit {
            panic!("Vertex circulator ran out of iterations!");
        }

        Option::Some(edges)
    }

    pub fn face_circulator(&self, index: HalfEdgeIndex) -> Option<Vec<HalfEdgeIndex>> {
        if index.index >= self.len() as u32 {
            return Option::None;
        }

        let mut early_exit = false;
        let mut cur_edge_index = index;
        let mut edges: Vec<HalfEdgeIndex> = vec![cur_edge_index];
        for i in 0..100 {
            cur_edge_index = self[cur_edge_index].next_edge;
            if cur_edge_index == index {
                early_exit = true;
                break;
            }
            if cur_edge_index.is_unset() {
                panic!("Edge index is unset! Cant continue on circulator!");
            }

            edges.push(cur_edge_index);
        }

        if !early_exit {
            panic!("Vertex circulator ran out of iterations!");
        }

        Option::Some(edges)
    }

    pub fn is_boundary_index(&self, index: HalfEdgeIndex) -> Option<bool> {
        match self.edge_pair(index) {
            None => None,
            Some(pair) => {
                Option::Some((self[index].adjacent_face.is_unset()) || pair.adjacent_face.is_unset())
            }
        }
    }
}
