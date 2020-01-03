use super::constants::{UNSET_VALUE};

pub struct FaceIndex {
    pub index: u32
}

impl FaceIndex {
    pub fn unset() -> FaceIndex {
        FaceIndex { index: UNSET_VALUE}
    }

    pub fn new(index: u32) -> FaceIndex {
        FaceIndex {index: index}
    }
}

pub struct Face{

}