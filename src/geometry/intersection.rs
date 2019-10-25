use super::{Line, Point, LineSegment, Polygon};

pub enum LineSegmentLineSegmentIntersectionResult {
    None,
    Point(Point),
    Overlap(LineSegment)
}

// possible result cases for line intersections
#[derive(PartialEq)]
pub enum LineLineIntersectionResult {
    None, // No intersection, lines are parallel
    Point(Point), // intersection in a single point
    Equal, // input lines are parallel (or anti-parallel) and have the same y offset
}

pub enum PolygonPolygonIntersectionResult {
    None, // No intersection
    Point(Point), // intersection in a single point
    Multiple(Vec<Point>) // multiple intersections
}

pub struct Intersection {

}

impl Intersection {
    // public line - line intersection test
    pub fn line_line(first: &Line, other: &Line, tol: f64) -> LineLineIntersectionResult {
        // test parallel
        if first.is_parallel_to(other, tol){
            println!("Parallel lines, {:?} and {:?}", first, other);
            // pick any point on first line and see if it lies on other line
            let pt_test = first.point_at(0.0);
            if other.is_point_on(&pt_test, tol){
                return LineLineIntersectionResult::Equal
            }
            else{
                return LineLineIntersectionResult::None
            }
        }
        else{
            //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
            let x = (other.b - first.b) / (first.m - other.m);
            return LineLineIntersectionResult::Point(first.point_at(x))
        }
    }

    // public lineSegment - lineSegment intersection test
    pub fn line_segment_line_segment(first: &LineSegment, other: &LineSegment, tol: f64) -> LineSegmentLineSegmentIntersectionResult {

        // new algorithm bazed on bezier representation
        // see: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

        // check denominator == 0 in tolerance -> early exit
        let denominator = first.denominator_with_other(other);
        println!("denominator is {} for line segments {:?} and {:?}", denominator, first, other);
        match denominator.abs() < tol{
            // parallel lines!, check for coincident, line segments have to share a point!
            true => match other.is_point_on(&first.from, tol) | first.is_point_on(&other.from, tol) {
                true => {
                    // infinite lines are equal, so there must be overlap
                    // store points in vec
                    let mut pts = vec![&first.from, &first.to, &other.from, &other.to];
                    // check if all x values are equal -> compare y values
                    if pts[0].x == pts[1].x && pts[1].x == pts[2].x && pts[2].x == pts[3].x {
                        pts.sort_by(|a, b| b.y.partial_cmp(&a.x).unwrap())
                    }
                    else{
                        // compare x values instead
                        pts.sort_by(|a, b| b.x.partial_cmp(&a.x).unwrap())
                    }
                    pts.reverse();
                    println!("Points sorted are: {:?}", pts);
                    // line segment from leftmost (or lowest) to rightmost (or highest) point
                    LineSegmentLineSegmentIntersectionResult::Overlap(LineSegment::new_from_points(&pts[0], &pts[3]))
                }
                false => {
                    LineSegmentLineSegmentIntersectionResult::None
                }
            }
            false => {

                // make this readable at the cost of memory :/
                let x1 = first.from.x;
                let x2 = first.to.x;
                let x3 = other.from.x;
                let x4 = other.to.x;
                let y1 = first.from.y;
                let y2 = first.to.y;
                let y3 = other.from.y;
                let y4 = other.to.y;

                // calculate possible intersection point on self and other
                let divisor = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
                let first_param = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / divisor;
                // early exit if param not normalized
                if (0.0 > first_param) || (first_param > 1.0){
                    println!("First param was {}, early exit", first_param);
                    return LineSegmentLineSegmentIntersectionResult::None
                }

                let other_param = ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / divisor;
                // early exit if param not normalized
                if (0.0 > other_param) || (other_param > 1.0){
                    println!("Other param was {}, early exit", other_param);
                    return LineSegmentLineSegmentIntersectionResult::None
                }

                println!("Evaluation parameters I found are: {} and {}", first_param, other_param);

                // now we are sure the intersection point lies on both lines
                let int_pt_x = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / divisor;
                let int_pt_y = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / divisor;

                LineSegmentLineSegmentIntersectionResult::Point(Point::new().set_values(int_pt_x, int_pt_y))

            }
        }
    }

    // public polygon - polygon
    pub fn polygon_polygon(first: &Polygon, other: &Polygon, tol: f64) -> PolygonPolygonIntersectionResult {
        let first_edges = first.calculate_edges();
        let other_edges = other.calculate_edges();
        let mut int_pts: Vec<Point> = Vec::new();
        let mut found_intersection = false;

        println!("first edges are: {:?}", first_edges);
        println!("other edges are: {:?}", other_edges);

        for f_edge in &first_edges {
            for o_edge in &other_edges {
                match Intersection::line_segment_line_segment(&f_edge, &o_edge, tol){
                    LineSegmentLineSegmentIntersectionResult::None => continue,
                    LineSegmentLineSegmentIntersectionResult::Point(int_pt) =>{
                        int_pts.push(int_pt);
                        found_intersection = true;
                },
                    LineSegmentLineSegmentIntersectionResult::Overlap(int_line) =>{
                        int_pts.append(&mut vec![int_line.from, int_line.to]);
                        found_intersection = true;
                    }
            };
        };
        }

        // check results
        if !found_intersection {PolygonPolygonIntersectionResult::None}
        else {
            if int_pts.len() == 1 {PolygonPolygonIntersectionResult::Point(int_pts[0].clone())}
            else{PolygonPolygonIntersectionResult::Multiple(int_pts)}
        }
    }
}