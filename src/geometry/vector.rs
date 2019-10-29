use super::{Point, constants};

#[derive(Debug, PartialEq)]
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

    pub fn epsilon_equals(&self, other: &Vector, tol: f64) -> bool {
        (self.x - other.x).abs() < tol && (self.y - other.y).abs() < tol
    }

    // public length function
    pub fn calculate_length(&self) -> f64 {
        if self.x == self.y && self.x == 0.0 {
            return 0.0
        }

        (self.x * self.x + self.y * self.y).sqrt()
    }

    // public normalizer
    pub fn normalize(mut self) {
        let len = self.calculate_length();

        if len == 0.0 {
            return
        }

        self.x /= len;
        self.y /= len;
    }

    // normalized copy
    pub fn as_normalized(&self) -> Vector {
        let len = self.calculate_length();

        if len == 0.0 {
            return Vector::new()
        }

        Vector{
            x: self.x / len,
            y: self.y / len
        }
    }

    /// Calculates signed angle between two vectors
    /// angles are parametrized between 0 and 2PI going counter-clockwise
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_nfp::geometry::{Vector};
    /// use std::f64::consts::PI;
    /// 
    /// let v0 = Vector::new().set_values(1.0, 1.0);
    /// let v1 = Vector::new().set_values(-1.0, -1.0);
    /// 
    /// assert!((v0.angle_to(&v1) - PI).abs() < 0.000001)
    pub fn angle_to(&self, other: &Vector) -> f64 {
        // dot product gives shortest angle
        let angle = self.as_normalized().dot_product(&other.as_normalized()).acos();

        // compute sign -> positive sign is angle less then pi
        let sign = Vector::cross_product(self, &other);

        // compute mirror factor, set to zero if positive sign
        let mut fac = 0.0;
        if sign < 0.0 {
            fac = 2.0 * (std::f64::consts::PI - angle);
        }

        (angle + fac) % (2.0 * std::f64::consts::PI)
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