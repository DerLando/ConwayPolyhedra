

pub trait MeshPartCollection<T, U> {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn add(&mut self, element: T) -> U;
}

pub trait UnsetValue {
    fn unset() -> Self;
    fn is_unset(&self) -> bool;
}