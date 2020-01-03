pub use self::point::Point;
mod point;

pub use self::half_edge::{HalfEdge, HalfEdgeIndex};
mod half_edge;

pub use self::vertex::{Vertex, VertexIndex};
mod vertex;

pub use self::face::{Face, FaceIndex};
mod face;

pub mod constants {
    // is 4294967295
    pub const UNSET_VALUE: u32 = u32::max_value();
}