use super::{Point};
use crate::lib::constants;

#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    
    // public empty constructor
    pub fn new() -> Vector {
        Vector{x: 0.0, y: 0.0}
    }

    // public value setter
    pub fn set_values(mut self, x: f64, y: f64) -> Vector {
        self.x = x;
        self.y = y;
        return self;
    }

    // public constructor from two points
    pub fn new_from_points(from: &Point, to: &Point) -> Vector {
        // subtract the points
        Vector{
            x: to.x - from.x,
            y: to.y - from.y
        }
    }

    // public static cross-product
    pub fn cross_product(v0: &Vector, v1: &Vector) -> f64 {
        v0.x * v1.y - v0.y * v1.x
    }
}