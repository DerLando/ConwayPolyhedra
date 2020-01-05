use super::constants::{UNSET_VALUE};
use super::{HalfEdgeIndex, Point, MeshPartCollection, UnsetValue};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Copy, Clone, PartialOrd, Debug)]
pub struct VertexIndex {
    pub index: u32,
}

impl UnsetValue for VertexIndex {
    fn unset() -> VertexIndex {
        VertexIndex { index: UNSET_VALUE}
    }

    fn is_unset(&self) -> bool {
        *self == VertexIndex::unset()
    }
}

impl VertexIndex {
    pub fn new(index: u32) -> VertexIndex {
        VertexIndex {index: index}
    }

    pub fn increment(&mut self) {
        self.index += 1;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub outgoing_half_edge: HalfEdgeIndex,
    pub location: Point,
}

impl Vertex {
    pub fn unset() -> Vertex {
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
        self.outgoing_half_edge.is_unset()
    }
}

#[derive(Debug)]
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

impl MeshPartCollection<Vertex, VertexIndex> for VertexCollection {
    fn new() -> VertexCollection {
        VertexCollection {
            vertices: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn add(&mut self, v: Vertex) -> VertexIndex {
        self.vertices.push(v);
        VertexIndex::new((self.len() - 1) as u32)
    }
}

impl VertexCollection {
    pub fn remove_range(&mut self, start: VertexIndex, count: usize) {
        self.vertices.drain(start.index as usize..start.index as usize + count);
    }
}