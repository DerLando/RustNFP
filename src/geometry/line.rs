use super::{Point, constants};
use std::f64::{INFINITY, NEG_INFINITY};

#[derive(Debug, Clone, PartialEq)]
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

    pub const YAXIS: Line = Line{m: INFINITY, b: 0.0};

    // constructor from two given points
    pub fn new_from_points(pt0: &Point, pt1: &Point) -> Line {

        // axes checks
        if(pt0.x - pt1.x).abs() < constants::ZERO_TOLERANCE {
            // parallel to y axis
            return Line::YAXIS
        }
        else{
            if (pt0.y - pt1.y).abs() < constants::ZERO_TOLERANCE {
                // parallel to x axis
                return Line::new_from_values(0.0, pt0.y)
            }
        }

        // no parallelity to axes, business as usual
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

    // public point-at function, returns a point on the line for a given x value
    pub fn point_at(&self, x: f64) -> Point {
        let y = self._evaluate_x(x);
        Point{x, y}
    }

    // parallelity check
    pub fn is_parallel_to(&self, other: &Line, tol: f64) -> bool {
        (self.m - other.m).abs() < tol
    }

    // point-on-line check
    pub fn is_point_on(&self, pt: &Point, tol: f64) -> bool {
        (self._evaluate_x(pt.x) - pt.y).abs() < tol
    }

    // private x-evaluator
    fn _evaluate_x(&self, x: f64) -> f64 {
        // y = mx + b
        self.m * x + self.b
    }

}
