#[cfg(test)]
pub mod vertex_tests {
    use super::super::geometry::{Mesh, Vertex, Point};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

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
        mesh.add_face_by_indices(vec![VertexIndex::new(0), VertexIndex::new(1), VertexIndex::new(2), VertexIndex::new(3)]);

        // Assert
        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.half_edge_count(), 8);
        assert_eq!(mesh.face_count(), 1);

        // Act
        mesh.remove_face(FaceIndex::new(0));
        mesh.compact();
        println!("Compact mesh is: {:?}", mesh);
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.half_edge_count(), 0);
        assert_eq!(mesh.face_count(), 0);
    }
}
