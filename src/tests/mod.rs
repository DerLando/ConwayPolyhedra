#[cfg(test)]
pub mod vertex_tests {
    use super::super::geometry::{Mesh, Vertex, Point};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn vertex_adding() {
        // Arrange
        let mut mesh = Mesh::new();
        let pt1 = Point::new();
        let mut pt2 = Point::new();
        pt2.x = 10.0;
        let v1 = Vertex::new(pt1);
        let v2 = Vertex::new(pt2);

        // Assert
        assert_eq!(mesh.vertex_count(), 0);
        mesh.add_vertex(v1);
        assert_eq!(mesh.vertex_count(), 1);
        mesh.add_vertex(v2);
        assert_eq!(mesh.vertex_count(), 2);
    }

    #[test]
    fn vertex_compacting() {
        // Arrange
        let mut mesh = Mesh::new();
        let pt1 = Point::new();
        let mut pt2 = Point::new();
        pt2.x = 10.0;
        let v1 = Vertex::new(pt1);
        let v2 = Vertex::new(pt2);

        // Act
        mesh.add_vertex(v1);
        mesh.add_vertex(v2);
        mesh.compact();

        // Assert
        assert_eq!(mesh.vertex_count(), 0);
    }
}

#[cfg(test)]
pub mod edge_tests {
    use super::super::geometry::{Mesh, Vertex, Face, Point, VertexIndex, FaceIndex, HalfEdge, HalfEdgeIndex};

    #[test]
    fn can_find_edges() {
        // Arrange
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex_position(Point::new());
        let v1 = mesh.add_vertex_position(Point::from_values(2.0, 0.0, 0.0));
        let v2 = mesh.add_vertex_position(Point::from_values(2.0, 2.0, 0.0));
        mesh.add_face_by_indices(vec![v0, v1, v2]);
        
        // Assert
        println!("can_find_edges mesh is: {:?}", mesh);
        assert_eq!(HalfEdgeIndex::new(0), mesh.find_half_edge_index(v0, v1).unwrap());
        assert_eq!(HalfEdgeIndex::new(2), mesh.find_half_edge_index(v1, v2).unwrap());
        assert_eq!(HalfEdgeIndex::new(4), mesh.find_half_edge_index(v2, v0).unwrap());

    }

    #[test]
    fn can_find_end_vertices() {
        // Arrange
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex_position(Point::new());
        let v1 = mesh.add_vertex_position(Point::from_values(2.0, 0.0, 0.0));
        let v2 = mesh.add_vertex_position(Point::from_values(2.0, 2.0, 0.0));
        mesh.add_face_by_indices(vec![v0, v1, v2]);

        // Assert
        assert_eq!(mesh.find_end_vertex_index(HalfEdgeIndex::new(0)), v1);
        assert_eq!(mesh.find_end_vertex_index(HalfEdgeIndex::new(1)), v0);

        // this fails currently
        assert_eq!(mesh.find_end_vertex_index(HalfEdgeIndex::new(2)), v2);

        assert_eq!(mesh.find_end_vertex_index(HalfEdgeIndex::new(3)), v1);

        // this fails currently
        assert_eq!(mesh.find_end_vertex_index(HalfEdgeIndex::new(4)), v0);

        assert_eq!(mesh.find_end_vertex_index(HalfEdgeIndex::new(5)), v2);
    }
}

#[cfg(test)]
pub mod face_tests {
    use super::super::geometry::{Mesh, Vertex, Face, Point, VertexIndex, FaceIndex};

    #[test]
    fn face_adding() {
        // Arrange
        let mut mesh = Mesh::new();
        mesh.add_vertex_position(Point::new());
        mesh.add_vertex_position(Point::from_values(2.0, 0.0, 0.0));
        mesh.add_vertex_position(Point::from_values(2.0, 2.0, 0.0));
        mesh.add_vertex_position(Point::from_values(0.0, 2.0, 0.0));

        // Act
        let f_index = mesh.add_face_by_indices(vec![VertexIndex::new(0), VertexIndex::new(1), VertexIndex::new(2), VertexIndex::new(3)]);

        // Assert
        assert_eq!(FaceIndex::new(0), f_index);
        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.half_edge_count(), 8);
        assert_eq!(mesh.face_count(), 1);

        // Act
        println!("NOT compact mesh is: {:?}", mesh);
        mesh.remove_face(FaceIndex::new(0));
        mesh.compact();

        // Assert
        println!("Compact mesh is: {:?}", mesh);
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.half_edge_count(), 0);
        assert_eq!(mesh.face_count(), 0);
    }

    #[test]
    fn add_multiple_faces() {
        // Arrange
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex_position(Point::new());
        let v1 = mesh.add_vertex_position(Point::from_values(2.0, 0.0, 0.0));
        let v2 = mesh.add_vertex_position(Point::from_values(2.0, 2.0, 0.0));
        let v3 = mesh.add_vertex_position(Point::from_values(0.0, 2.0, 0.0));

        // Act
        let f0 = mesh.add_face_by_indices(vec![v0, v1, v2]);
        let f1 = mesh.add_face_by_indices(vec![v2, v0, v3]);

        // Assert
        println!("f0 and f1: {:?}, {:?}", f0, f1);
        assert_eq!(mesh.face_count(), 2);
        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.half_edge_count(), 10);
    }
}
