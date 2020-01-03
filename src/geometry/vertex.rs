use super::constants::{UNSET_VALUE};

pub struct VertexIndex {
    pub index: u32,
}

impl VertexIndex {
    pub fn unset() -> VertexIndex {
        VertexIndex { index: UNSET_VALUE}
    }

    pub fn new(index: u32) -> VertexIndex {
        VertexIndex {index: index}
    }
}

pub struct Vertex {

}