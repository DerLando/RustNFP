mod lib;
mod tests;

use lib::structs::{Point, Polygon};

fn main() {
    let pt1 = Point::new();
    let pt3 = Point::new().set_values(1.0, 0.0);
    let pt2 = Point::new().set_values(1.0, 1.0);

    let vec = vec![pt1, pt2, pt3];
    let poly = Polygon::from_points(vec);

    println!("My poly is: {:?}", poly);

    println!("A square with len 3 is: {:?}", Polygon::square(2.0));
}
