use super::{Point, constants};

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

    // public length function
    pub fn calculate_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // public normalizer
    pub fn normalize(mut self) {
        let len = self.calculate_length();
        self.x /= len;
        self.y /= len;
    }

    // normalized copy
    pub fn as_normalized(&self) -> Vector {
        let len = self.calculate_length();
        Vector{
            x: self.x / len,
            y: self.y / len
        }
    }

    // angle function
    pub fn angle_to(&self, other: &Vector) -> f64 {
        (self.as_normalized().dot_product(&other.as_normalized()).acos())
    }

    // public static cross-product
    pub fn cross_product(v0: &Vector, v1: &Vector) -> f64 {
        v0.x * v1.y - v0.y * v1.x
    }

    // public dot product function
    pub fn dot_product(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }
}