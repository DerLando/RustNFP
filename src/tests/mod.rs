#[cfg(test)]
pub mod polygon_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::geometry::{Polygon, Point, LineSegment, constants::ZERO_TOLERANCE, PolygonMergeResult};
    use std::f64::consts::PI;

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
        let poly = Polygon::from_points(&vec);

        // Act
        let area = poly.calculate_area();

        // Assert
        assert_eq!(area, 72.5);
    }

    #[test]
    fn test_point_on() {
        // Arrange
        let poly = Polygon::square(2.0);
        let pt_test = Point::new().set_values(1.0, 1.0);

        // Assert
        assert!(poly.is_point_on(&pt_test, ZERO_TOLERANCE))
    }

    #[test]
    fn test_edges_consistent() {
        //Arrange
        let pt0 = Point::new();
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(-1.0, 1.0);
        let e1 = LineSegment::new_from_points(&pt0, &pt1);
        let e2 = LineSegment::new_from_points(&pt1, &pt2);
        let e3 = LineSegment::new_from_points(&pt2, &pt0);
        let poly = Polygon::from_points(&vec![pt0, pt1, pt2]);

        //Act
        let edges = poly.calculate_edges();

        // Assert
        assert_eq!(edges, vec![e1, e2, e3]);

    }

    #[test]
    fn test_angles_consistent() {
        // Arrange
        let poly = Polygon::square(2.0);

        // Act
        let angles = poly.calculate_angles();

        // Assert
        for angle in angles.iter() {
            assert_eq!(*angle, PI / 2.0);
        }
    }

    #[test]
    fn test_convex() {
        // Arrange
        let poly = Polygon::square(2.0);

        // Assert
        assert!(poly.is_convex());
    }

    #[test]
    fn test_concave() {
        // Arrange
        let pt0 = Point::new();
        let pt1 = Point::new().set_values(2.0, 0.0);
        let pt2 = Point::new().set_values(1.0, 2.0);
        let pt3 = Point::new().set_values(2.0, 4.0);
        let pt4 = Point::new().set_values(0.0, 4.0);
        let poly = Polygon::from_points(&vec![pt0, pt1, pt2, pt3, pt4]);

        // Assert
        assert!(poly.is_concave());
    }

    #[test]
    fn test_triangulation() {
        // Arrange
        let pt0 = Point::new();
        let pt1 = Point::new().set_values(2.0, 0.0);
        let pt2 = Point::new().set_values(1.5, 1.0);
        let pt3 = Point::new().set_values(2.0, 2.0);
        let pt4 = Point::new().set_values(0.0, 2.0);
        let poly = Polygon::from_points(&vec![pt0, pt1, pt2, pt3, pt4]);

        // Act
        let triangulated = poly.triangulate(ZERO_TOLERANCE);

        // Assert
        println!("Triangulated: {:?}", triangulated);
        assert_eq!(triangulated.len(), 3);
    }

    #[test]
    fn test_merge() {
        // Arrange
        let poly = Polygon::square(2.0);
        let tris = poly.triangulate(ZERO_TOLERANCE);

        match Polygon::merge_convex_polygon(&tris[0], &tris[1], ZERO_TOLERANCE) {
            PolygonMergeResult::None => panic!("Not merged!"),
            PolygonMergeResult::Merged(m) => assert_eq!(m.points.len(), 4)
        }
    }

    #[test]
    fn test_subdivide() {
        // Arrange
        let poly = Polygon::square(2.0);

        // Act
        let subdivided = poly.subdivide_concave_polygon_in_convex_pieces(ZERO_TOLERANCE);
        
        // Assert
        assert!(subdivided.len() != 0);
        assert_eq!(subdivided.len(), 1);
        assert_eq!(subdivided[0].points.len(), 4);
    }
}

#[cfg(test)]
pub mod vector_tests {
    use super::super::geometry::{Vector, Polygon, Point, LineSegment, constants::ZERO_TOLERANCE};
    use std::f64::consts::PI;

    #[test]
    fn test_vector_angle_anti_parallel() {
        // Arrange
        let v1 = Vector::new().set_values(1.0, 0.0);
        let v2 = Vector::new().set_values(-1.0, 0.0);

        // Act
        let angle = v1.angle_to(&v2);

        // Assert
        assert_eq!(angle, PI);
    }

    #[test]
    fn test_vector_angle_same() {
        // Arrange
        let v1 = Vector::new().set_values(1.0, 1.0);

        // Act
        let angle = v1.angle_to(&v1);

        // Assert
        assert!(angle.abs() < ZERO_TOLERANCE)
    }

    #[test]
    fn test_vector_angle_order_matters() {
        // Arrange
        let v1 = Vector::new().set_values(1.0, 0.0);
        let v2 = Vector::new().set_values(1.0, 0.5);

        // Act
        let angle_between_v1_and_v2 = v1.angle_to(&v2);
        let angle_between_v2_and_v1 = v2.angle_to(&v1);

        // Assert
        assert_ne!(angle_between_v1_and_v2, angle_between_v2_and_v1);
    }

    #[test]
    fn test_vector_angle_225_degrees_edge_case() {
        // Arrange
        let x_axis = Vector::new().set_values(1.0, 0.0);
        let v_bottom_left = Vector::new().set_values(-2.0, -2.0);
        let v_bottom_right = Vector::new().set_values(2.0, -2.0);

        // Act
        let angle_bottom_left = x_axis.angle_to(&v_bottom_left);
        let angle_bottom_right = x_axis.angle_to(&v_bottom_right);

        // Assert
        assert_eq!(angle_bottom_left, 1.25 * PI);
        assert_eq!(angle_bottom_right, 1.75 * PI);
    }
}

#[cfg(test)]
pub mod line_tests {
    use super::super::geometry::{Line, Point, constants::ZERO_TOLERANCE};

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
    fn test_point_at() {
        // Arrange
        let pt0 = Point::new().set_values(1.0, 0.0);
        let pt1 = Point::new().set_values(2.0, 1.0);
        let line = Line::new_from_points(&pt0, &pt1);

        // Act
        let pt_at = line.point_at(3.0);

        // Assert
        assert_eq!(pt_at, Point::new().set_values(3.0, 2.0))

    }

    #[test]
    fn test_point_on_line() {
        // Arrange
        let pt0 = Point::new();
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(3.0, 3.0);
        let line = Line::new_from_points(&pt0, &pt1);

        // Assert
        assert!(line.is_point_on(&pt2, ZERO_TOLERANCE));
    }

    #[test]
    fn test_point_not_on_line() {
        // Arrange
        let pt0 = Point::new();
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(2.0, 3.0);
        let line = Line::new_from_points(&pt0, &pt1);

        // Assert
        assert!(!line.is_point_on(&pt2, ZERO_TOLERANCE));
    }

    #[test]
    fn test_point_on_line_parallel_to_x_axis() {
        // Arrange
        let pt0 = Point::new().set_values(0.0, 5.0);
        let pt1 = Point::new().set_values(1.0, 5.0);
        let pt_test = Point::new().set_values(8.0, 5.0);
        let line = Line::new_from_points(&pt0, &pt1);

        // Assert
        assert!(line.is_point_on(&pt_test, ZERO_TOLERANCE))
    }
}

#[cfg(test)]
pub mod point_tests {
    use super::super::geometry::{Point};

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
    use super::super::geometry::{LineSegment, Point, constants::ZERO_TOLERANCE};

    #[test]
    fn test_line_segment_is_point_on() {
        // Arrange
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt_test = Point::new().set_values(1.0, 1.0);
        let line = LineSegment::new_from_points(&pt0, &pt1);

        // Assert
        assert!(line.is_point_on(&pt_test, ZERO_TOLERANCE));
    }

    #[test]
    fn test_denominator_zero_on_parallel() {
        //Arrange
        let pt0 = Point::new().set_values(0.0, 1.0);
        let pt1 = Point::new().set_values(0.0, 5.0);
        let pt2 = Point::new().set_values(4.0, 2.0);
        let pt3 = Point::new().set_values(4.0, 8.0);
        let line1 = LineSegment::new_from_points(&pt0, &pt1);
        let line2 = LineSegment::new_from_points(&pt2, &pt3);

        // Assert
        assert!(line1.denominator_with_other(&line2).abs() < ZERO_TOLERANCE);
    }
}

#[cfg(test)]
pub mod intersection_tests {
    use super::super::geometry::{LineSegment, Line, Point, Intersection, Polygon, constants::ZERO_TOLERANCE};
    use super::super::geometry::intersection::{LineLineIntersectionResult, LineSegmentLineSegmentIntersectionResult, PolygonPolygonIntersectionResult};

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
        let result = Intersection::line_line(&line0, &line1, ZERO_TOLERANCE);

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
        let result = Intersection::line_line(&line0, &line1, ZERO_TOLERANCE);

        // Assert
        match result {
            LineLineIntersectionResult::None => panic!("None"),
            LineLineIntersectionResult::Point(_) => panic!("Point"),
            LineLineIntersectionResult::Equal => assert!(true),
        }
    }

    #[test]
    fn test_line_line_intersection_none() {
        // Arrange 
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(0.0, -1.0);
        let pt3 = Point::new().set_values(2.0, 1.0);
        let line0 = Line::new_from_points(&pt0, &pt1);
        let line1 = Line::new_from_points(&pt2, &pt3);

        // Act
        let result = Intersection::line_line(&line0, &line1, ZERO_TOLERANCE);

        // Assert
        match result {
            LineLineIntersectionResult::None => assert!(true),
            LineLineIntersectionResult::Point(_) => panic!("Point found"),
            LineLineIntersectionResult::Equal => panic!("Equal"),
        }

    }

    #[test]
    fn test_line_segment_line_segment_intersection_point() {
        // Arrange
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(2.0, 0.0);
        let line0 = LineSegment::new_from_points(&pt0, &pt1);
        let line1 = LineSegment::new_from_points(&pt1, &pt2);

        // Act
        let result = Intersection::line_segment_line_segment(&line0, &line1, ZERO_TOLERANCE);

        // Assert
        match result {
            LineSegmentLineSegmentIntersectionResult::None => panic!("None"),
            LineSegmentLineSegmentIntersectionResult::Point(pt) => assert_eq!(pt, pt1),
            LineSegmentLineSegmentIntersectionResult::Overlap(_) => panic!("Overlap"),

        }
    }

    #[test]
    fn test_line_segment_line_segment_vertical_horizontal_point() {
        // Arrange
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(-1.0, 1.0);
        let pt2 = Point::new().set_values(-2.0, 1.0);
        let pt3 = Point::new().set_values(2.0, 1.0);
        let line0 = LineSegment::new_from_points(&pt0, &pt1);
        let line1 = LineSegment::new_from_points(&pt2, &pt3);

        // Act
        let result = Intersection::line_segment_line_segment(&line0, &line1, ZERO_TOLERANCE);

        // Assert
        match result {
            LineSegmentLineSegmentIntersectionResult::None => panic!("None"),
            LineSegmentLineSegmentIntersectionResult::Point(pt) => assert_eq!(pt, pt1),
            LineSegmentLineSegmentIntersectionResult::Overlap(_) => panic!("Overlap"),

        }
    }

    #[test]
    fn test_line_segment_line_segment_vertical_horizontal_none() {
        // Arrange
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(-1.0, 1.0);
        let pt2 = Point::new().set_values(2.0, 1.0);
        let pt3 = Point::new().set_values(4.0, 1.0);
        let line0 = LineSegment::new_from_points(&pt0, &pt1);
        let line1 = LineSegment::new_from_points(&pt2, &pt3);

        // Act
        let result = Intersection::line_segment_line_segment(&line0, &line1, ZERO_TOLERANCE);

        // Assert
        match result {
            LineSegmentLineSegmentIntersectionResult::None => assert!(true),
            LineSegmentLineSegmentIntersectionResult::Point(pt) => panic!(format!("Found a point, {:?}", pt)),
            LineSegmentLineSegmentIntersectionResult::Overlap(_) => panic!("Overlap"),

        }
    }

    #[test]
    fn test_line_segment_line_segment_intersection_overlap() {
        // Arrange
        let pt0 = Point::new().set_values(-1.0, -1.0);
        let pt1 = Point::new().set_values(1.0, 1.0);
        let pt2 = Point::new().set_values(-0.5, -0.5);
        let pt3 = Point::new().set_values(2.0, 2.0);
        let line0 = LineSegment::new_from_points(&pt0, &pt1);
        let line1 = LineSegment::new_from_points(&pt2, &pt3);

        // Act
        let result = Intersection::line_segment_line_segment(&line0, &line1, ZERO_TOLERANCE);

        // Assert
        match result {
            LineSegmentLineSegmentIntersectionResult::None => panic!("None"),
            LineSegmentLineSegmentIntersectionResult::Point(_) => panic!("Point found"),
            LineSegmentLineSegmentIntersectionResult::Overlap(line) => assert_eq!(line, LineSegment::new_from_points(&pt0, &pt3)),
        }
    }

    #[test]
    fn test_polygon_edges_intersection_yields_corners() {
        // Arrange
        let poly = Polygon::square(2.0);
        let original_corners = &poly.points;
        let edges = poly.calculate_edges();

        // Act
        for n in 0..edges.len() {
            let next_index = (n + 1) % edges.len();
            let prev_index = (n + edges.len() - 1) % edges.len();
            let l1 = LineSegment::new_from_points(&original_corners[prev_index], &original_corners[n]);
            let l2 = LineSegment::new_from_points(&original_corners[n], &original_corners[next_index]);

            match Intersection::line_segment_line_segment(&l1, &l2, ZERO_TOLERANCE) {
                LineSegmentLineSegmentIntersectionResult::None => panic!("None"),
                LineSegmentLineSegmentIntersectionResult::Overlap(_) => panic!("Overlap"),
                LineSegmentLineSegmentIntersectionResult::Point(pt) => {
                    if !pt.epsilon_equals(&original_corners[n], ZERO_TOLERANCE){
                        panic!(format!("Points were not equal under epsilon on index {}!, {:?}, {:?}", n, pt, original_corners[n]))
                    }
                }
            }
        }

        assert!(true)
    }

    #[test]
    fn test_polygon_polygon_intersection_none() {
        // Arrange
        let poly1 = Polygon::square(3.0);
        let poly2 = Polygon::square(1.0);

        // Act
        let result = Intersection::polygon_polygon(&poly1, &poly2, ZERO_TOLERANCE);

        // Assert
        match result {
            PolygonPolygonIntersectionResult::None => assert!(true),
            PolygonPolygonIntersectionResult::Point(pt_int) => {
                println!("Found single intersection point: {:?}", pt_int);
                panic!("Point");
            },
            PolygonPolygonIntersectionResult::Multiple(pts) => {
                println!("Multiple int points are: {:?}", pts);
                panic!("Multiple");
            }
        }
    }

    #[test]
    fn test_polygon_polygon_intersection_point() {
        // Arrange
        let pt0 = Point::new().set_values(1.0, 1.0);
        let pt1 = Point::new().set_values(3.0, 3.0);
        let pt2 = Point::new().set_values(-3.0, 3.0);

        let tri = Polygon::from_points(&vec![pt0, pt1, pt2]);
        let square = Polygon::square(2.0);

        // Act
        let result = Intersection::polygon_polygon(&tri, &square, ZERO_TOLERANCE);

        // Assert
        match result {
            PolygonPolygonIntersectionResult::None => panic!("No intersection"),
            PolygonPolygonIntersectionResult::Point(pt_int) => assert_eq!(pt_int, pt0),
            PolygonPolygonIntersectionResult::Multiple(pts) => {
                println!("Multiple int points are: {:?}", pts);
                panic!("Multiple");
            }
        }
        
    }
}