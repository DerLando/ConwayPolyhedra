use super::{VertexData, EdgeData, PolygonData};

pub struct HalfData{
    pub next: Box<HalfData>,
    pub previous: Box<HalfData>,
    pub pair: Box<HalfData>,

    pub origin: Box<VertexData>,
    pub left: Box<PolygonData>,
    pub edge: Box<EdgeData>
    
}