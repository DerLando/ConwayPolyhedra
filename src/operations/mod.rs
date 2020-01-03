use super::geometry::Point;

pub trait FaceCenter{
    fn compute(&self) -> Point; 
}

pub trait Dual{
    fn compute(&self) -> Self;
}