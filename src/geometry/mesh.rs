use super::{*};
use super::constants::{UNSET_VALUE};
use super::{MeshPartCollection, UnsetValue};
use std::fmt;

pub struct Mesh {
    vertices: VertexCollection,
    edges: HalfEdgeCollection,
    faces: FaceCollection,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:#?}, {:#?}), {:#?}", self.vertices, self.faces, self.edges)
    }
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

    pub fn compact(&mut self) {
        self.vertex_compact();
        self.face_compact();
        self.half_edge_compact();
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

    fn vertex_compact(&mut self) {
        let mut marker = VertexIndex::new(0);

        for i in 0..self.vertex_count() {
            let iter = VertexIndex::new(i as u32);

            if !self.vertices[iter].is_unused() {
                if marker < iter {
                    self.vertices[marker] = self.vertices[iter];

                    // update all outgoing halfedges
                    let first = self.vertices[marker].outgoing_half_edge;
                    for edge_index in self.edges.vertex_circulator(first).unwrap() {
                        self.edges[edge_index].start_vertex = marker;
                    }
                marker.increment();
                }
            }
        }

        // trim list down to size
        if marker.index < self.vertex_count() as u32 {
            self.vertices.remove_range(marker, self.vertex_count() - marker.index as usize);
        }
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

        let index1 = self.add_half_edge(e1);
        let _index2 = self.add_half_edge(e2);

        println!("start outgoing v: {:?}", self.vertices[start].outgoing_half_edge);
        if self.vertices[start].outgoing_half_edge.is_unset() {
            self.vertices[start].outgoing_half_edge = index1;
        }
        println!("start outgoing v: {:?}", self.vertices[start].outgoing_half_edge);
        index1
    }

    pub fn find_half_edge_index(&self, start: VertexIndex, end: VertexIndex) -> Option<HalfEdgeIndex> {
        let halfedge_index = self.vertices[start].outgoing_half_edge;
        println!("find index: {:?}", halfedge_index);
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

    pub fn remove_half_edge_pair(&mut self, index: HalfEdgeIndex) {
        // if index.is_unset() {
        //     return;
        // }

        let pair = HalfEdgeCollection::edge_pair_index(index);

        // if pair.is_unset(){
        //     self.edges[index] = HalfEdge::unset();
        //     return;
        // }

        // reconnect adjacent halfedges
        self.edges.make_consecutive(self.edges[pair].previous_edge, self.edges[index].next_edge);
        self.edges.make_consecutive(self.edges[index].previous_edge, self.edges[pair].next_edge);

        // update outgoind halfedges, if necessary. If last halfedge then
        // make vertex unused (outgoing.is_unset()), otherwise set to next around vertex
        let v_index = self.edges[index].start_vertex;
        let v_pair = self.edges[pair].start_vertex;

        if self.vertices[v_index].outgoing_half_edge == index {
            if self.edges[pair].next_edge == index {
                self.vertices[v_index].outgoing_half_edge = HalfEdgeIndex::unset();
            }
            else {
                self.vertices[v_index].outgoing_half_edge = self.edges[pair].next_edge;
            }
        }
        if self.vertices[v_pair].outgoing_half_edge == pair {
            if self.edges[index].next_edge == pair {
                self.vertices[v_pair].outgoing_half_edge = HalfEdgeIndex::unset();
            }
            else {
                self.vertices[v_pair].outgoing_half_edge = self.edges[index].next_edge;
            }
        }

        // mark half-edges for deletion
        self.edges[index] = HalfEdge::unset();
        self.edges[pair] = HalfEdge::unset();
    }

    fn half_edge_compact(&mut self){
        let mut marker = HalfEdgeIndex::new(0);

        for i in 0..self.half_edge_count() {

            let iter = HalfEdgeIndex::new(i as u32);

            // check if used
            if !self.edges[marker].is_unused() {
                if marker < iter {
                    // Copy current edge to marker slot
                    self.edges[marker] = self.edges[iter];

                    // update start vertex if necessary
                    let v_index = self.edges[marker].start_vertex;
                    if self.vertices[v_index].outgoing_half_edge == iter {
                        self.vertices[v_index].outgoing_half_edge = marker;
                    }

                    // update adjacent face if necessary
                    if self.edges[marker].adjacent_face.is_unset() {
                        let f_index = self.edges[marker].adjacent_face;
                        if self.faces[f_index].first_half_edge == iter {
                            self.faces[f_index].first_half_edge = marker;
                        }
                    }

                    // update next/prev halfedges
                    let next_index = self.edges[marker].next_edge;
                    self.edges[next_index].previous_edge = marker;
                    let prev_index = self.edges[marker].previous_edge;
                    self.edges[prev_index].next_edge = marker;
                }
                marker.increment(); // spots filled, increment the marker
            }
        }

        // check if even count of edges
        if marker.index % 2 != 0 {
            panic!("Halfedge count was uneven after compact call!");
        }

        // trim list
        if marker.index < self.half_edge_count() as u32 {
            self.edges.remove_range(marker, self.half_edge_count() - marker.index as usize)
        }
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
                    println!("added edge: {:#?}", edges[i])
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
            self.edges[edges[i]].next_edge = edges[(i + 1) % n];
            // TODO: link previous also
        }

        // Add face
        return self.add_face(Face::new(edges[0]));
    }

    pub fn remove_face(&mut self, index: FaceIndex) {
        match self.face_half_edge_indices(index) {
            None => (),
            Some(indices) => {
                println!("edges in face are: {:#?}", indices);
                for edge_index in indices {
                    if self.edges.is_boundary_index(edge_index).unwrap() {
                        self.remove_half_edge_pair(edge_index);
                    }
                    else {
                        self.edges[edge_index].adjacent_face = FaceIndex::unset();
                        self.vertices[self.edges[edge_index].start_vertex].outgoing_half_edge = edge_index;
                    }
                }
                self.faces[index] = Face::unset();
            }
        }
    }

    pub fn face_half_edge_indices(&self, index: FaceIndex) -> Option<Vec<HalfEdgeIndex>> {
        self.edges.face_circulator(self.faces[index].first_half_edge)
    }

    pub fn face_compact(&mut self) {
        let mut marker = FaceIndex::new(0);

        for i in 0..self.face_count() {
            let iter = FaceIndex::new(i as u32);

            // test valid face
            if !self.faces[iter].is_unused() {
                if marker < iter {
                    self.faces[marker] = self.faces[iter];

                    // update all adjacent half-edges
                    for edge_index in self.face_half_edge_indices(marker).unwrap() {
                        self.edges[edge_index].adjacent_face = marker;
                    }
                }
                marker.increment();
            }
        }

        // trim list down to new size
        if marker.index < self.face_count() as u32 {
            self.faces.remove_range(marker, self.face_count() - marker.index as usize)
        }
    }

}