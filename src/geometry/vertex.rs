use super::constants::{UNSET_VALUE};
use super::{HalfEdgeIndex, Point};
use std::ops::{Index, IndexMut};

pub struct VertexIndex {
    pub index: u32,
}

impl VertexIndex {
    pub const fn unset() -> VertexIndex {
        VertexIndex { index: UNSET_VALUE}
    }

    pub fn new(index: u32) -> VertexIndex {
        VertexIndex {index: index}
    }
}

pub struct Vertex {
    pub outgoing_half_edge: HalfEdgeIndex,
    pub location: Point,
}

impl Vertex {
    pub const fn unset() -> Vertex {
        Vertex {
            outgoing_half_edge: HalfEdgeIndex::unset(),
            location: Point::unset()
        }
    }

    pub fn new(location: Point) -> Vertex {
        let mut v = Vertex::unset();
        v.location = location;
        v
    }

    pub fn is_unused(&self) -> bool {
        self.outgoing_half_edge.index == UNSET_VALUE
    }
}

pub struct VertexCollection {
    vertices: Vec<Vertex>
}

impl Index<VertexIndex> for VertexCollection {
    type Output = Vertex;

    fn index(&self, index: VertexIndex) -> &Self::Output {
        &self.vertices[index.index as usize]
    }
}

impl IndexMut<VertexIndex> for VertexCollection {
    fn index_mut(&mut self, index: VertexIndex) -> &mut Self::Output {
        &mut self.vertices[index.index as usize]
    }
}

impl VertexCollection {
    pub fn new() -> VertexCollection {
        VertexCollection {
            vertices: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn add(&mut self, v: Vertex) -> usize {
        self.vertices.push(v);
        self.len() - 1
    }
}