use super::{*};

pub struct Mesh {
    vertices: VertexCollection,
    edges: HalfEdgeCollection,
    faces: FaceCollection,
}

// general methods
impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: VertexCollection::new(),
            edges: HalfEdgeCollection::new(),
            faces: FaceCollection::new()
        }
    }
}

// All things related to vertices
impl Mesh {
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_vertex(&mut self, v: Vertex) -> usize {
        self.vertices.add(v)
    }

    fn vertex_compact(&self) {

    }
}