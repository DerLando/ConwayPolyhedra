use super::constants::{UNSET_VALUE};
use super::{HalfEdgeIndex, Point};

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