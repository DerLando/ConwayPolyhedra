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
