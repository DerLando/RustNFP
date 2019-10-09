use super::{Point};
use crate::lib::constants;

// possible result cases for line intersections
pub enum LineIntersection {
    None,
    Point(Point),
    Line(Line),
}

#[derive(Debug, Clone)]
pub struct Line {
    pub m: f64, // rate of change
    pub b: f64 // y offset
}

impl Line {
    // public generic constructor returning x-axis
    pub fn new() -> Line {
        Line{m: 0.0, b: 0.0}
    }

    // constructor from values for rate-of-change and y-offset
    pub fn new_from_values(m: f64, b: f64) -> Line {
        Line{m: m, b: b}
    }

    // constructor from two given points
    pub fn new_from_points(pt0: &Point, pt1: &Point) -> Line {
        let m = Line::calculate_slope(pt0, pt1);
        let b = Line::calculate_y_offset(m, pt0);

        Line{m: m, b: b}
    }

    // private helper to calculate rate of change of two points
    fn calculate_slope(pt0: &Point, pt1: &Point) -> f64 {
        // rate of change is y-change divided through x change
        let x_change = pt1.x - pt0.x;
        let y_change = pt1.y - pt0.y;
        return y_change / x_change;
    }

    // private helper to calculate y offset from a slope and a given point
    fn calculate_y_offset(m: f64, pt: &Point) -> f64 {
        // y = m * x + b ==> -b = m * x - y ==> b = y - m * x
        return pt.y - m * pt.x;
    }

    // public point-at function, returns a point on the line for a givne x value
    pub fn point_at(&self, x: f64) -> Point {
        // y = mx + b
        let y = self.m * x + self.b;
        Point{x, y}
    }

    // Intersector function
    pub fn intersect_line(&self, other: &Line) -> LineIntersection {
        // test parallel
        if self.is_parallel_to(other){
            return LineIntersection::Line(self.clone())
        }
        else {
            //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
            let x = (other.b - self.b) / (self.m - other.m);
            return LineIntersection::Point(self.point_at(x))
        }
    }

    // parallelity check
    pub fn is_parallel_to(&self, other: &Line) -> bool {
        (self.m - other.m).abs() < constants::ZERO_TOLERANCE
    }

}
