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
    
}
