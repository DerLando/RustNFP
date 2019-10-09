#[cfg(test)]
pub mod polygon_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use no_fit_polygon::structs::{Polygon, Point};

    #[test]
    fn test_area_square() {
        // Arrange
        let poly = Polygon::square(4.0);

        // Act
        let area = poly.calculate_area();

        // Assert
        assert_eq!(area, 16.0);
    }

    #[test]
    fn test_area_any() {
        // Arrange
        let pt0 = Point::new().set_values(2.0, 2.0);
        let pt1 = Point::new().set_values(11.0, 2.0);
        let pt2 = Point::new().set_values(14.0, 9.0);
        let pt3 = Point::new().set_values(4.0, 10.0);
        let vec = vec![pt0, pt1, pt2, pt3];
        let poly = Polygon::from_points(vec);

        // Act
        let area = poly.calculate_area();

        // Assert
        assert_eq!(area, 72.5);
    }
}