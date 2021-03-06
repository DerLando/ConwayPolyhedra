use super::constants::{UNSET_VALUE};
use super::{MeshPartCollection, UnsetValue, HalfEdgeIndex};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Copy, Clone, PartialOrd, Debug)]
pub struct FaceIndex {
    pub index: u32
}

impl UnsetValue for FaceIndex {
    fn unset() -> FaceIndex {
        FaceIndex { index: UNSET_VALUE}
    }

    fn is_unset(&self) -> bool {
        *self == FaceIndex::unset()
    }
}

impl FaceIndex {
    pub fn new(index: u32) -> FaceIndex {
        FaceIndex {index: index}
    }

    pub fn increment(&mut self) {
        self.index += 1;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Face {
    pub first_half_edge: HalfEdgeIndex
}

impl Face {
    pub fn new(first_edge: HalfEdgeIndex) -> Face {
        Face {
            first_half_edge: first_edge
        }
    }

    pub fn unset() -> Face {
        Face {
            first_half_edge: HalfEdgeIndex::unset()
        }
    }

    pub fn is_unused(&self) -> bool {
        self.first_half_edge.is_unset()
    }
}

#[derive(Debug)]
pub struct FaceCollection {
    faces: Vec<Face>
}

impl Index<FaceIndex> for FaceCollection {
    type Output = Face;

    fn index(&self, index: FaceIndex) -> &Self::Output {
        &self.faces[index.index as usize]
    }
}

impl IndexMut<FaceIndex> for FaceCollection {
    fn index_mut(&mut self, index: FaceIndex) -> &mut Self::Output {
        &mut self.faces[index.index as usize]
    }
}

impl MeshPartCollection<Face, FaceIndex> for FaceCollection {
    fn new() -> FaceCollection {
        FaceCollection {
            faces: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.faces.len()
    }

    fn add(&mut self, face: Face) -> FaceIndex {
        self.faces.push(face);
        FaceIndex::new((self.len() - 1) as u32)
    }
}

impl FaceCollection {
    pub fn remove_range(&mut self, start: FaceIndex, count: usize) {
        self.faces.drain(start.index as usize..start.index as usize + count);
    }
}
