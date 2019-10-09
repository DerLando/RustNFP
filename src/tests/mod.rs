#[cfg(test)]
pub mod polygon_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::structs::{Polygon, Point};

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

#[cfg(test)]
pub mod line_tests {
    use super::super::structs::{Line, Point};

    #[test]
    fn test_two_point_constructor() {
        // Arrange
        let pt0 = Point::new().set_values(1.0, 0.0);
        let pt1 = Point::new().set_values(2.0, 1.0);
        let line = Line::new_from_points(&pt0, &pt1);

        // Assert
        assert_eq!(line.m, 1.0);
        assert_eq!(line.b, -1.0);
    }

    #[test]
    fn test_point_on_line() {
        // Arrange
        let pt0 = Point::new();
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(2.0, 2.0);
        let line = Line::new_from_points(&pt0, &pt1);

        // Assert
        assert!(line.is_point_on(&pt2));
    }
}

#[cfg(test)]
pub mod point_tests {
    use super::super::structs::{Point};

    #[test]
    fn test_point_distance() {
        // Arrange
        let pt0 = Point::new().set_values(0.0, 0.0);
        let pt1 = Point::new().set_values(2.0, 0.0);

        // Act
        let dist = pt0.distance_to(&pt1);

        // Assert
        assert_eq!(dist, 2.0);
    }
}

#[cfg(test)]
pub mod line_segment_tests {
    use super::super::structs::{LineSegment, Point};

}

#[cfg(test)]
pub mod intersection_tests {
    use super::super::structs::{LineSegment, Line, Point, Intersection};
    use super::super::structs::intersection::{LineLineIntersectionResult};

    #[test]
    fn test_line_line_intersection_point() {
        // Arrange
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(-1.0, 1.0);
        let pt3 = Point::new().set_values(1.0, -1.0);
        let line0 = Line::new_from_points(&pt0, &pt1);
        let line1 = Line::new_from_points(&pt2, &pt3);

        // Act
        let result = Intersection::line_line(&line0, &line1);

        // Assert
        match result {
            LineLineIntersectionResult::None => panic!("None"),
            LineLineIntersectionResult::Point(pt) => assert_eq!(pt, Point::new()),
            LineLineIntersectionResult::Equal => panic!("Equal"),
        }
    }

        #[test]
    fn test_line_line_intersection_equal() {
        // Arrange
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(1.0, 1.0);
        let line0 = Line::new_from_points(&pt0, &pt1);
        let line1 = Line::new_from_points(&pt1, &pt0);

        // Act
        let result = Intersection::line_line(&line0, &line1);

        // Assert
        match result {
            LineLineIntersectionResult::None => panic!("None"),
            LineLineIntersectionResult::Point(_) => panic!("Point"),
            LineLineIntersectionResult::Equal => assert!(true),
        }
    }
}