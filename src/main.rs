mod lib;
mod tests;
mod structs;

use structs::{Point, Polygon, Line, LineSegment, Intersection, intersection::LineSegmentLineSegmentIntersectionResult};

fn main() {

    let pt0 = Point::new().set_values(-1.0, -1.0);
    let pt1 = Point::new().set_values(1.0, 1.0);
    let pt2 = Point::new().set_values(2.0, 0.0);
    let line0 = LineSegment::new_from_points(&pt0, &pt1);
    let line1 = LineSegment::new_from_points(&pt1, &pt2);

    // Act
    let result = Intersection::line_segment_line_segment(&line0, &line1);

    // Assert
    match result {
        LineSegmentLineSegmentIntersectionResult::None => panic!("None"),
        LineSegmentLineSegmentIntersectionResult::Point(pt) => assert_eq!(pt, pt1),
        LineSegmentLineSegmentIntersectionResult::Overlap(_) => panic!("Overlap"),

    }

    println!("my line segment is {:?}", line0);
}
