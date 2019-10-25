use super::{Line, Point};

#[derive(Debug, PartialEq)]
pub struct LineSegment{
    pub line: Line,
    pub from: Point,
    pub to: Point
}

impl LineSegment{
    // public constructor from points
    pub fn new_from_points(pt0: &Point, pt1: &Point) -> LineSegment {
        LineSegment{
            line: Line::new_from_points(pt0, pt1),
            from: pt0.clone(),
            to: pt1.clone(),
        }
    }

    pub fn calculate_length(&self) -> f64 {
        self.from.distance_to(&self.to)
    }

    // public line on point check
    pub fn is_point_on(&self, pt: &Point, tol: f64) -> bool {
        // https://stackoverflow.com/a/328110

        // co_linear check first
        if Point::are_colinear(&self.from, &self.to, pt, tol){
            if self.from.x == self.to.x{
                (self.from.x <= pt.x && pt.x <= self.to.x) | (self.from.x >= pt.x && pt.x >= self.to.x)
            }
            else{
                (self.from.y <= pt.x && pt.y <= self.to.y) | (self.from.y >= pt.y && pt.y >= self.to.y)
            }
        }
        else{
            false
        }
    }

    // public bezier parameter evaluation
    pub fn point_at_normalized_parameter(&self, t: f64) -> Point {
        if (t < 0.0) | (t > 1.0) {panic!("point_at_normalized_parameter ERROR: Parameter was not normalized!");}
        Point {
            x: self.from.x + t * (self.to.x - self.from.x),
            y: self.from.y + t * (self.to.y - self.from.y)}
    }

}