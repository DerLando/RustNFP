use super::{Point, Line, LineSegment, Vector, Intersection, LineSegmentLineSegmentIntersectionResult};
use std::iter::FromIterator;
use std::f64::consts::PI;
use std::collections::HashSet;

pub enum PolygonEdgeRelation {
    None,
    Shared(usize, usize)
}

pub enum PolygonMergeResult {
    None,
    Merged(Polygon)
}

#[derive(Debug)]
pub struct Polygon{
    pub points: Vec<Point>,
}

impl Polygon {
    // public new helper
    pub fn new() -> Polygon {
        let pts: Vec<Point> = Vec::new();
        Polygon{points: pts}
    }

    // public helper to construct from points
    pub fn from_points(pts: &Vec<Point>) -> Polygon {
        // let cloned_points = pts.iter().map(|p| p.clone()).collect::<Vec<_>>();
        let cloned_points = pts.clone();
        Polygon{points: cloned_points}
    }

    /// Public constructor from a list of line-segments
    /// WARNING: There are no error checks, this will just push start-points of the line segment in the points vector!
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_nfp::geometry::{Polygon}; // geometric structs
    /// 
    /// let tol = 0.001; // geometric tolerance
    /// 
    /// let poly = Polygon::square(2.0); // square with side length 2.0
    /// let edges = poly.calculate_edges();
    /// let reconstructed = Polygon::from_edges(&edges);
    /// assert!(reconstructed.epsilon_equals(&poly, tol));
    /// ```
    pub fn from_edges(edges: &Vec<LineSegment>) -> Polygon {
        let edge_count = edges.len();
        let mut pts: Vec<Point> = Vec::with_capacity(edge_count);

        for n in 0..edge_count {
            pts.push(edges[n].from);
        }

        Polygon{
            points: pts
        }
    }

    // public helper to add a point to the end of the points list
    pub fn add_point(mut self, pt: Point) {
        self.points.push(pt)
    }

    /// Public equality check under tolerance
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_nfp::geometry::{Polygon}; // geometric structs
    /// 
    /// let tol = 0.001; // geometric tolerance
    /// 
    /// let poly = Polygon::square(2.0); // square with side length 2.0
    /// let other = Polygon::square(2.00001); // pretty close side length ;)
    /// 
    /// assert!(poly.epsilon_equals(&other, tol));
    pub fn epsilon_equals(&self, other: &Polygon, tol: f64) -> bool {
        self.points.iter().zip(&other.points).all(|(x, y)| x.epsilon_equals(&y, tol))
    }

    // public area calculation
    pub fn calculate_area(&self) -> f64 {
        let mut area: f64 = 0.0;
        let pt_count = self.points.len();

        // iterate over points
        for (i, pt) in self.points.iter().enumerate() {
            let next_index = (i + 1) % pt_count;
            let next_point = &self.points[next_index];
            let int_value = pt.x * next_point.y - pt.y * next_point.x;
            area += int_value;
        }
            
        // keep area positive
        if area < 0.0 {
            area *= -1.0
            };

        area / 2.0
    }

    // public edge getter
    pub fn calculate_edges(&self) -> Vec<LineSegment> {

        // empty edge array
        let edge_count = self.points.len();
        let mut edges: Vec<LineSegment> = Vec::with_capacity(edge_count);

        // iterate over points
        for i in 0..edge_count {
            let next_index = (i + 1) % (edge_count);
            edges.push(LineSegment::new_from_points(&self.points[i], &self.points[next_index]));
        };

        return edges;
    }

    // public single edge getter
    pub fn calculate_single_edge(&self, index: usize) -> LineSegment {
        let next_index = (index + 1) % self.points.len();

        LineSegment::new_from_points(&self.points[index], &self.points[next_index])
    }

    // public concavity test
    pub fn is_concave(&self) -> bool {
        self.calculate_angles().iter().any(|&x| x > std::f64::consts::PI / 2.0)
    }

    // public convex test
    pub fn is_convex(&self) -> bool {
        !self.is_concave()
    }

    // public interior angle calculator
    pub fn calculate_angles(&self) -> Vec<f64> {
        let corner_count = self.points.len();
        let mut angles: Vec<f64> = Vec::with_capacity(corner_count);

        for i in 0..corner_count {
            let prev_index = (i + corner_count - 1) % corner_count;
            let next_index = (i + 1) % corner_count;

            let v_from_last_corner = Vector::new_from_points(&self.points[prev_index], &self.points[i]);
            let v_to_next_corner = Vector::new_from_points(&self.points[i], &self.points[next_index]);

            angles.push(v_from_last_corner.angle_to(&v_to_next_corner));
        }

        return angles;
    }

    // public point on
    pub fn is_point_on(&self, pt_test: &Point, tol: f64) -> bool {
        let edges = self.calculate_edges();
        for edge in edges{
            if edge.is_point_on(pt_test, tol) {
                return true
            }
        }
        false
    }

    // public reverse orientation
    pub fn reverse_orientation(&mut self) {
        self.points.reverse()
    }

    /// helper do determine polygons who share an edge
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_nfp::geometry::{Polygon, Point}; // geometric structs
    /// use rust_nfp::geometry::PolygonEdgeRelation; // Edge relation enum
    /// 
    /// let tol = 0.001; // geometric tolerance
    /// 
    /// let square = Polygon::square(2.0); // square with side length 2.0
    /// let pt0 = Point::new().set_values(1.0, -1.0);
    /// let pt1 = Point::new().set_values(2.0, 0.0);
    /// let pt2 = Point::new().set_values(1.0, 1.0);
    /// let tri = Polygon::from_points(&vec![pt0, pt1, pt2]); // Triangle coinciding with right edge of square
    /// 
    /// match square.shares_an_edge(&tri, tol){
    ///     PolygonEdgeRelation::None => panic!("No edge relation found!"),
    ///     PolygonEdgeRelation::Shared(s, o) => {
    ///         assert_eq!(s, 1); // shared edge index on square is 1
    ///         assert_eq!(o, 2); // shared edge index on tri is 2
    ///     }    
    /// };
    /// ```
    pub fn shares_an_edge(&self, other: &Polygon, tol: f64) -> PolygonEdgeRelation {
        let self_edges = self.calculate_edges();
        let other_edges = other.calculate_edges();

        for s in 0..self_edges.len() {
            for o in 0..other_edges.len() {
                if !self_edges[s].is_from_to_coincident(&other_edges[o], tol){
                    continue
                }

                return PolygonEdgeRelation::Shared(s, o);
            }
        }

        PolygonEdgeRelation::None
    }

    pub fn corner_index(&self, corner: &Point, tol: f64) -> Result<usize, ()> {
        match self.points.iter().position(|x| x.epsilon_equals(&corner, tol)) {
            Option::None => Result::Err(()),
            Option::Some(index) => Result::Ok(index)
        }
    }

    pub fn prev_corner_before_edge(&self, edge_index: usize) -> usize {
        (edge_index - 1 + self.points.len()) % self.points.len()
    }

    pub fn next_corner_after_edge(&self, edge_index: usize) -> usize {
        (edge_index + 2) % self.points.len()
    }

    pub fn edge_from_to(&self, edge_index: usize) -> (usize, usize) {
        (edge_index, ((edge_index + 1) % self.points.len()))
    }

    pub fn corners_without_edge(&self, edge_index: usize) -> Vec<Point> {
        let start_index = self.prev_corner_before_edge(edge_index);
        let end_index = self.next_corner_after_edge(edge_index);
        if start_index == end_index {
            return vec![self.points[start_index]];
        }
        else {
            println!("WithoutEdge: trying to take slice from {} to {} on {:?}", start_index, end_index, &self.points);
            self.points[start_index..end_index].into_iter().cloned().collect::<Vec<Point>>()
        }
    }

    // public static square from side length
    pub fn square(len: f64) -> Polygon {
        let pt0 = Point::new().set_values(-len / 2.0, -len / 2.0);
        let pt1 = Point::new().set_values(len / 2.0, -len / 2.0);
        let pt2 = Point::new().set_values(len / 2.0, len / 2.0);
        let pt3 = Point::new().set_values(-len / 2.0, len / 2.0);

        Polygon::from_points(&vec![pt0, pt1, pt2, pt3])
    }

    pub fn circle(radius: f64, corner_count: usize) -> Polygon {
        if corner_count == 0 {
            return Polygon::new();
        }

        let rot_step = 2.0 * PI / corner_count as f64;
        let mut corners: Vec<Point> = Vec::with_capacity(corner_count);
        for n in 0..corner_count {
            corners.push(Point::new_from_polar(radius, n as f64 * rot_step));
        }

        Polygon::from_points(&corners)
    }

    pub fn triangulate(&self, tol: f64) -> Vec<Polygon> {
        let mut convex_parts: Vec<Polygon> = Vec::new();

        // helper struct
        #[derive(Debug)]
        struct PartitionLine {
            pub from: usize,
            pub to: usize,
            pub middle: usize,
            pub length: f64
        }

        impl PartitionLine {
            pub fn all_indices(&self) -> HashSet<usize> {
                let mut all: HashSet<usize> = [self.from, self.middle, self.to].iter().cloned().collect();
                all
            }

            pub fn is_index_intersecting(&self, other: &PartitionLine) -> bool {
                self.all_indices().is_disjoint(&other.all_indices())
            }

            pub fn contains_other_middle(&self, other: &PartitionLine) -> bool {
                self.all_indices().contains(&other.middle)
            }
        }

        // copy points in array that can be manipulated
        let mut corners = Vec::from_iter(self.points[..].iter().cloned());

        while corners.len() > 3 {
            let poly = Polygon::from_points(&corners);
            let corner_count = poly.points.len();
            let angles = poly.calculate_angles();
            let mut possible_partition_lines: Vec<PartitionLine> = Vec::new();
            let edges = poly.calculate_edges();

            println!("Current poly is: {:?}", poly);

            // iterate over all corners
            for n in 0..corner_count {
                // inner angle > 180° -> cannot be a valid partition line
                if angles[n] > PI {
                    continue;
                }

                let prev_index = (n + corner_count - 1) % corner_count;
                let next_index = (n + 1) % corner_count;
                let mut is_intersecting = false;

                let line = LineSegment::new_from_points(&poly.points[prev_index], &poly.points[next_index]);
                for i in 0..corner_count {
                    if (i == prev_index) | (i == n) {
                        continue
                    }
                    let edge = &edges[i];
                    match Intersection::line_segment_line_segment(&line, edge, tol) {
                        LineSegmentLineSegmentIntersectionResult::None => continue,
                        LineSegmentLineSegmentIntersectionResult::Overlap(_) => {
                            is_intersecting = true;
                            break;
                        },
                        LineSegmentLineSegmentIntersectionResult::Point(pt) => {
                            if pt.epsilon_equals(&edge.from, tol) | pt.epsilon_equals(&edge.to, tol) {
                                // println!("Line for corner {}, intersecting edge {}, Point was fine {:?}", n, i, pt);
                                continue;
                            }
                            else {
                                // println!("Line for corner {}, intersecting edge {}, Point was NOT fine {:?} \nLines in question are {:?} and \n{:?}", n, i, pt, line, edge);
                                is_intersecting = true;
                                break;
                            }
                        }
                    }
                }

                if is_intersecting{
                    continue;
                }
                // TODO: We can calculate the angles here explicitly, so we don't have to iterate the corners twice

                // push partition line
                possible_partition_lines.push(PartitionLine{
                    from: prev_index, 
                    to: next_index,
                    middle: n,
                    length: poly.points[prev_index].distance_to_squared(&poly.points[next_index])});
            }

            // sort partition lines by their length
            possible_partition_lines.sort_by(|a, b| b.length.partial_cmp(&a.length).unwrap());

            // println!("Partition lines sorted are: {:?}", possible_partition_lines);

            // shortest partition line is what we want
            let found_partition = possible_partition_lines.pop().unwrap();
            convex_parts.push(Polygon::from_points(&vec![poly.points[found_partition.from], poly.points[found_partition.middle], poly.points[found_partition.to]]));

            // remove used vertex and start over
            corners.remove(found_partition.middle);
        }

        // corners now has only 3 points left -> convex poly
        convex_parts.push(Polygon::from_points(&corners));

        return convex_parts;
    }

    pub fn merge_convex_polygon(first: &Polygon, other: &Polygon, tol: f64) -> PolygonMergeResult {
        // test for coincident edge first
        let mut first_edge_index: usize = 0;
        let mut other_edge_index: usize = 0;
        match first.shares_an_edge(&other, tol) {
            PolygonEdgeRelation::None => return PolygonMergeResult::None,
            PolygonEdgeRelation::Shared(s, o) => {
                first_edge_index = s;
                other_edge_index = o;
            }
        };

        println!("Passed shared edge test...");

        let first_shared_vertex_indices = first.edge_from_to(first_edge_index);
        let other_shared_vertex_indices = other.edge_from_to(other_edge_index);
        let prev = first.prev_corner_before_edge(first_edge_index);
        let next = other.next_corner_after_edge(other_edge_index);

        // first shared vertex
        use crate::geometry::LinePointRelation;
        match Line::line_point_relation_fast(
            &first.points[prev],
            &other.points[next],
            &first.points[first_shared_vertex_indices.1],
            tol) {
                LinePointRelation::Right =>{
                    println!("Got right for first: {:?}, other: {:?} and test: {:?} ", &first.points[prev], &other.points[next], &first.points[first_shared_vertex_indices.1]);
                    return PolygonMergeResult::None},
                LinePointRelation::On => return PolygonMergeResult::None,
                LinePointRelation::Left => (),//do nothing
            };
        
        println!("Passed first point relation test...");
        
        // second shared vertex
        match Line::line_point_relation_fast(
            &other.points[next],
            &first.points[prev],
            &first.points[first_shared_vertex_indices.0],
            tol) {
                LinePointRelation::Right => return PolygonMergeResult::None,
                LinePointRelation::On => return PolygonMergeResult::None,
                LinePointRelation::Left => (), // do nothing
            };
        
        println!("Passed second vertex relation test...");
        
        // if we get here we can finally merge
        let mut other_corners = other.corners_without_edge(other_edge_index);
        let mut corners: Vec<Point> = first.points.iter().cloned().collect();

        println!("other_corners are: {:?}", other_corners);

        println!("Attempting to take slice from {}, to {} on vec: {:?}", first_shared_vertex_indices.0, first_shared_vertex_indices.1, corners);
        
        if first_shared_vertex_indices.1 == 0 {
            corners.append(&mut other_corners);
            println!("Appending... corners are now: {:?}", &corners);
        }
        else {
            corners.splice(first_shared_vertex_indices.0..first_shared_vertex_indices.1, other_corners.into_iter());
            println!("Splicing... corners are now: {:?}", &corners);
        }

        return PolygonMergeResult::Merged(Polygon::from_points(&corners));


    }
}
