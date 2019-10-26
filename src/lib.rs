pub mod geometry;
mod tests;


#[no_mangle]
pub extern fn test_link() {
    println!("Hello world, I am running from a rust library")
}

pub mod lib {

    use super::geometry::{Point, Polygon, LineSegment, Line, Vector};
    use std::f64::consts::{PI};

    /// Calculates the nfp of two convex polygon
    /// WARNING: For performance we do NO error checking what so ever, so take care that the
    /// input polygon are valid and both convex!
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_nfp::lib::calculate_convex_nfp;
    /// use rust_nfp::geometry::{Polygon, Point};
    /// 
    /// let square = Polygon::square(2.0);
    /// let tri = Polygon::from_points(&vec![Point::new(), Point::new().set_values(2.0, 2.0), Point::new().set_values(-2.0, 2.0)]);
    /// let square2 = Polygon::square(2.0);
    /// let nfp = calculate_convex_nfp(&square, tri);
    /// 
    /// println!("nfp is: {:?}", nfp);
    /// assert_eq!(nfp.points.len(), 7);
    /// assert!(nfp.is_convex());
    pub fn calculate_convex_nfp(first: &Polygon, mut other: Polygon) -> Polygon {

        // helper line -> x-axis to compare angles to
        let x_axis = LineSegment::new_from_points(&Point::new(), &Point::new().set_values(1.0, 0.0));

        // reverse orientation of other
        other.reverse_orientation();

        // get all edges of both polygons
        let mut all_edges = first.calculate_edges();
        all_edges.extend(other.calculate_edges());

        // sort by angle to x_axis
        all_edges.sort_by(|a, b| (x_axis.angle_to(&b)).partial_cmp(&(x_axis.angle_to(&a))).unwrap());

        // create sorted list, first item is first edge popped
        let mut edges_sorted: Vec<LineSegment> = vec![all_edges.pop().unwrap()];

        // iteratively sort by `flattest` angle between line segments and pop
        while all_edges.len() > 0 {
            let last_edge = edges_sorted.last().unwrap();
            let all_angles_x_to_pt = all_edges.iter().map(|x| x_axis.angle_to(&x)).collect::<Vec<_>>();
            let all_angles_pt_to_x = all_edges.iter().map(|x| x.angle_to(&x_axis)).collect::<Vec<_>>();
            // all_edges.reverse();
            println!("angles from x to pt: {:?}", all_angles_x_to_pt);
            println!("angles from pt to x: {:?}", all_angles_pt_to_x);
            let next_edge = all_edges.pop().unwrap();
            println!("next_edge is: {:?}", next_edge);
            let next_edge_moved = next_edge.copy_to_point(&last_edge.to);
            edges_sorted.push(next_edge_moved);
        }

        Polygon::from_edges(&edges_sorted)
        
    }
}
