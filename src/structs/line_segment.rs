use super::{Line, Point};

#[derive(Debug)]
pub struct LineSegment{
    line: Line,
    from: Point,
    to: Point
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
}