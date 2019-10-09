use super::{Line, Point, LineSegment, Polygon};

pub enum LineSegmentLineSegmentIntersectionResult {
    None,
    Point(Point),
    Overlap(LineSegment)
}

// possible result cases for line intersections
#[derive(PartialEq)]
pub enum LineLineIntersectionResult {
    None, // No intersection, lines are parallel
    Point(Point), // intersection in a single point
    Equal, // input lines are parallel (or anti-parallel) and have the same y offset
}

pub struct Intersection {

}

impl Intersection {
    pub fn line_line(first: &Line, other: &Line) -> LineLineIntersectionResult {
        // test parallel
        if first.is_parallel_to(other){
            println!("Parallel lines, {:?} and {:?}", first, other);
            // pick any point on first line and see if it lies on other line
            let pt_test = first.point_at(0.0);
            if other.is_point_on(&pt_test){
                return LineLineIntersectionResult::Equal
            }
            else{
                return LineLineIntersectionResult::None
            }
        }
        else{
            //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
            let x = (other.b - first.b) / (first.m - other.m);
            return LineLineIntersectionResult::Point(first.point_at(x))
        }
    }
}