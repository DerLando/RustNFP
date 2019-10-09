use super::{Vector};
use crate::lib::constants;

#[derive(Debug, Clone, PartialEq)]
pub struct Point{
    pub x: f64,
    pub y: f64
}

impl Point {

    pub fn new() -> Point {
        Point{x: 0.0, y: 0.0}
    }

    pub fn set_values(mut self, x: f64, y: f64) -> Point {
        self.x = x;
        self.y = y;
        return self
    }

    // public distance function
    pub fn distance_to(&self, other: &Point) -> f64 {
        // Check for same point
        if self == other {
            return 0.0
        }
        else{
            let x_distance = other.x - self.x;
            let y_distance = other.y - self.y;
            (x_distance * x_distance - y_distance * y_distance).sqrt()
        }
    }

    // public co-linear check for 3 points
    pub fn are_colinear(pt0: &Point, pt1: &Point, pt2: &Point) -> bool {
        let vec0 = Vector::new_from_points(&pt0, &pt1);
        let vec1 = Vector::new_from_points(&pt0, &pt2);
        Vector::cross_product(&vec0, &vec1).abs() < constants::ZERO_TOLERANCE
    }
    
}
