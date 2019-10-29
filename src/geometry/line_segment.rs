use super::{Line, Point, Vector};

#[derive(Debug, PartialEq)]
pub struct LineSegment{
    pub line: Line,
    pub from: Point,
    pub to: Point,
    pub direction: Vector
}

impl LineSegment{
    // public constructor from points
    pub fn new_from_points(pt0: &Point, pt1: &Point) -> LineSegment {
        LineSegment{
            line: Line::new_from_points(pt0, pt1),
            from: pt0.clone(),
            to: pt1.clone(),
            direction: Vector::new_from_points(pt0, pt1)
        }
    }

    pub fn calculate_direction(&self) -> Vector {
        Vector::new_from_points(&self.from, &self.to)
    }

    pub fn calculate_length(&self) -> f64 {
        self.from.distance_to(&self.to)
    }

    pub fn angle_to(&self, other: &LineSegment) -> f64 {
        self.direction.angle_to(&other.direction)
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

    // public denominator calculation
    // d = (x1 - x2)(y3 - y4) - (y1 - y2)(x3 - x4)
    pub fn denominator_with_other(&self, other: &LineSegment) -> f64 {
        (self.from.x - self.to.x) * (other.from.y - other.to.y) - (self.from.y - self.to.y) * (other.from.x - other.to.x)
    }

    // public bezier parameter evaluation
    pub fn point_at_normalized_parameter(&self, t: f64) -> Point {
        if (t < 0.0) | (t > 1.0) {panic!("point_at_normalized_parameter ERROR: Parameter was not normalized!");}
        Point {
            x: self.from.x + t * (self.to.x - self.from.x),
            y: self.from.y + t * (self.to.y - self.from.y)}
    }

    pub fn is_from_to_coincident(&self, other: &LineSegment, tol: f64) -> bool {
        (self.from.epsilon_equals(&other.from, tol) | self.from.epsilon_equals(&other.to, tol)) &&
        (self.to.epsilon_equals(&other.from, tol) | self.to.epsilon_equals(&other.to, tol))
    }
    
    /// Translate LineSegment to new start point
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_nfp::geometry::{Point, LineSegment};
    /// 
    /// let pt0 = Point::new();
    /// let pt1 = Point::new().set_values(2.0, 2.0);
    /// let pt_new_start = Point::new().set_values(1.0, 0.0);
    /// let mut line = LineSegment::new_from_points(&pt0, &pt1);
    /// 
    /// line.move_to_point(&pt_new_start);
    /// 
    /// assert!(line.from.epsilon_equals(&pt_new_start, 0.001));
    /// assert!(line.to.epsilon_equals(&Point::new().set_values(3.0, 2.0), 0.001));
    /// ```
    pub fn move_to_point(&mut self, pt: &Point){
        let v_move = Vector::new_from_points(&self.from, &pt);

        self.from = Point::new().set_values(self.from.x + v_move.x, self.from.y + v_move.y);
        self.to = Point::new().set_values(self.to.x + v_move.x, self.to.y + v_move.y);
    }

    pub fn copy_to_point(self, pt: &Point) -> LineSegment {
        let v_move = Vector::new_from_points(&self.from, &pt);

        LineSegment::new_from_points(&self.from.copy_along_vector(&v_move), &self.to.copy_along_vector(&v_move))
    }

}