use super::constants::{UNSET_VALUE};
use super::{MeshPartCollection, UnsetValue};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Copy, Clone)]
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
}

pub struct Face{

}

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
}
