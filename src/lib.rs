pub mod geometry;
mod tests;

use geometry::{Point, Polygon, LineSegment, Line, Vector};

#[no_mangle]
pub extern fn test_link() {
    println!("Hello world, I am running from a rust library")
}
