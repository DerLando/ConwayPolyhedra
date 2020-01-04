

pub trait MeshPartCollection<T> {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn add(&mut self, element: T) -> usize;
}

pub trait UnsetValue {
    fn unset() -> Self;
    fn is_unset(&self) -> bool;
}