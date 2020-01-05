use super::{*};
use super::constants::{UNSET_VALUE};
use super::{MeshPartCollection, UnsetValue};

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

    pub fn add_vertex(&mut self, v: Vertex) -> VertexIndex {
        self.vertices.add(v)
    }

    pub fn add_vertex_position(&mut self, position: Point) -> VertexIndex {
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

    pub fn add_half_edge(&mut self, e: HalfEdge) -> HalfEdgeIndex {
        self.edges.add(e)
    }

    pub fn add_edge_pair(&mut self, start: VertexIndex, end: VertexIndex, face: FaceIndex) -> HalfEdgeIndex {
        let edge_count = self.half_edge_count() as u32;
        let edge_index = HalfEdgeIndex::new(edge_count);
        let next_edge_index = HalfEdgeIndex::new(edge_count + 1);
        let e1 = HalfEdge::new(start, face, next_edge_index);
        let e2 = HalfEdge::new(end, FaceIndex::unset(), edge_index);

        let index = self.add_half_edge(e1);
        self.add_half_edge(e2);

        index
    }

    pub fn find_half_edge_index(&self, start: VertexIndex, end: VertexIndex) -> Option<HalfEdgeIndex> {
        let halfedge_index = self.vertices[start].outgoing_half_edge;

        let result = self.edges.vertex_circulator(halfedge_index);
        match result {
            None => return Option::None,
            Some(circulator) => {
                for index in circulator {
                    if end == self.edges.edge_pair(index).unwrap().start_vertex {
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

    pub fn add_face(&mut self, face: Face) -> FaceIndex {
        self.faces.add(face)
    }

    pub fn add_face_by_indices(&mut self, indices:Vec<VertexIndex>) -> FaceIndex {
        let n = indices.len();
        let unset = FaceIndex::unset();

        // Check for degenerate
        if n < 3 {
            return unset;
        }

        // test if vertices are valid
        let v_count = self.vertex_count();
        for index in indices.clone() {
            if index.index >= v_count as u32 {
                panic!("Vertex index out of range!");
            }
            let outgoing_halfedge_index = self.vertices[index].outgoing_half_edge;
            if (!outgoing_halfedge_index.is_unset()) && (!self.edges[outgoing_halfedge_index].adjacent_face.is_unset()){
                return unset;
            }
        }

        // test each vertex pair, if they already share an half-edge
        // if so, check if that pair is already linked to a face
        // else, create it
        let mut edges = vec![HalfEdgeIndex::unset(); n];
        let face_index = FaceIndex::new(self.face_count() as u32);
        for i in 0..n {
            let cur_index = indices[i];
            let next_index = indices[(i + 1) % (n - 1)];

            // TODO: do this twice, start to end and end to start.
            match self.find_half_edge_index(cur_index, next_index) {
                None => {
                    edges[i] = self.add_edge_pair(cur_index, next_index, face_index);
                }
                Some(index) => {
                    if !index.is_unset() { // already an adjacent face -> non-manifold
                        return unset;
                    }
                    self.edges[index].adjacent_face = face_index;
                    edges[i] = index;
                }
            }
        }

        // Link half-edges
        for i in 0..n {
            self.edges[edges[i]].next_edge = edges[(i + 1) % (n - 1)];
        }

        // Add face
        return self.add_face(Face::new(edges[0]));
    }

    pub fn remove_face(&mut self, index: FaceIndex) {

    }
}