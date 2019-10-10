mod geometry;
mod tests;

#[no_mangle]
pub extern fn test_link() {
    println!("Hello world, I am running from a rust library")
}

// #[no_mangle]
// pub extern fn mass_intersect_line_segments_from_points(pts: Vec<Point>) -> Vec<Point> {

// }