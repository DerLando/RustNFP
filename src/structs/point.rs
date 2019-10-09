#[derive(Debug)]
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
    
}
