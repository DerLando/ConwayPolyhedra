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

// All things related to halfedges
impl Mesh {
    pub fn half_edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn add_half_edge(&mut self, e: HalfEdge) -> usize {
        self.edges.add(e)
    }

    pub fn add_edge_pair(&mut self, start: VertexIndex, end: VertexIndex, face: FaceIndex) {
        let edge_count = self.half_edge_count() as u32;
        let edge_index = HalfEdgeIndex::new(edge_count);
        let next_edge_index = HalfEdgeIndex::new(edge_count + 1);
        let e1 = HalfEdge::new(start, face, next_edge_index);
        let e2 = HalfEdge::new(end, FaceIndex::unset(), edge_index);

        self.add_half_edge(e1);
        self.add_half_edge(e2);
    }
}