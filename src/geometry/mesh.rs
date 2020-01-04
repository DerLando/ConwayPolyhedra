use super::{*};
use super::constants::{UNSET_VALUE};

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

    pub fn add_vertex_position(&mut self, position: Point) -> usize {
        self.vertices.add(Vertex::new(position))
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

    pub fn find_half_edge_index(&self, start: VertexIndex, end: VertexIndex) -> Option<HalfEdgeIndex> {
        let halfedge_index = self.vertices[start].outgoing_half_edge;

        let result = self.edges.vertex_circulator(halfedge_index);
        match result {
            None => return Option::None,
            Some(circulator) => {
                for index in circulator {
                    if end == self.edges.edge_pair(&index).unwrap().start_vertex {
                        return Some(index);
                    }
                }
                return Option::None
            }
        };
    }
}

// All things related to faces
impl Mesh {
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn add_face(&mut self, face: Face) -> usize {
        self.faces.add(face)
    }

    pub fn add_face_by_indices(&mut self, indices:Vec<VertexIndex>) -> usize {
        let n = indices.len();
        let unset = UNSET_VALUE as usize;

        // Check for degenerate
        if n < 3 {
            return unset;
        }

        // test if vertices are valid
        let v_count = self.vertex_count();
        for index in indices {
            if index.index >= v_count as u32 {
                panic!("Vertex index out of range!");
            }
            let outgoing_halfedge_index = self.vertices[index].outgoing_half_edge;
            if (!outgoing_halfedge_index.is_unset()) && (!self.edges[outgoing_halfedge_index].adjacent_face.is_unset()){
                return unset;
            }
        }



        return unset;
    }
}