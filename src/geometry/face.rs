use super::constants::{UNSET_VALUE};
use std::ops::{Index, IndexMut};

pub struct FaceIndex {
    pub index: u32
}

impl FaceIndex {
    pub const fn unset() -> FaceIndex {
        FaceIndex { index: UNSET_VALUE}
    }

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

impl FaceCollection {
    pub fn new() -> FaceCollection {
        FaceCollection {
            faces: Vec::new()
        }
    }
}
