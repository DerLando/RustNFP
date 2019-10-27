use super::{Point, Line, LineSegment, Vector, Intersection, LineSegmentLineSegmentIntersectionResult};
use std::iter::FromIterator;
use std::f64::consts::PI;
use std::collections::HashSet;

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

        // let corner_count = self.points.len();
        // let angles = self.calculate_angles();
        // let mut possible_partition_lines: Vec<PartitionLine> = Vec::new();
        // let edges = self.calculate_edges();

        // // iterate over all corners
        // for n in 0..corner_count {
        //     // inner angle > 180° -> cannot be a valid partition line
        //     if angles[n] > PI {
        //         continue;
        //     }

        //     let prev_index = (n + corner_count - 1) % corner_count;
        //     let next_index = (n + 1) % corner_count;
        //     let mut is_intersecting = false;

        //     let line = LineSegment::new_from_points(&self.points[prev_index], &self.points[next_index]);
        //     for i in 0..corner_count {
        //         if (i == prev_index) | (i == n) {
        //             continue
        //         }
        //         let edge = &edges[i];
        //         match Intersection::line_segment_line_segment(&line, edge, tol) {
        //             LineSegmentLineSegmentIntersectionResult::None => continue,
        //             LineSegmentLineSegmentIntersectionResult::Overlap(_) => {
        //                 is_intersecting = true;
        //                 break;
        //             },
        //             LineSegmentLineSegmentIntersectionResult::Point(pt) => {
        //                 if pt.epsilon_equals(&edge.from, tol) | pt.epsilon_equals(&edge.to, tol) {
        //                     println!("Line for corner {}, intersecting edge {}, Point was fine {:?}", n, i, pt);
        //                     continue;
        //                 }
        //                 else {
        //                     println!("Line for corner {}, intersecting edge {}, Point was NOT fine {:?} \nLines in question are {:?} and \n{:?}", n, i, pt, line, edge);
        //                     is_intersecting = true;
        //                     break;
        //                 }
        //             }
        //         }
        //     }

        //     if is_intersecting{
        //         continue;
        //     }
        //     // TODO: We can calculate the angles here explicitly, so we don't have to iterate the corners twice

        //     // push partition line
        //     possible_partition_lines.push(PartitionLine{
        //         from: prev_index, 
        //         to: next_index,
        //         middle: n,
        //         length: self.points[prev_index].distance_to_squared(&self.points[next_index])});
        // }

        // // sort partition lines by their length
        // possible_partition_lines.sort_by(|a, b| b.length.partial_cmp(&a.length).unwrap());

        // println!("Partition lines sorted are: {:?}", possible_partition_lines);

        // // TODO: THis has to be done iteratively!
        // // After selecting our partition, we remove the vertex and do it all over on a new polygon
        // // which is the original polygon minus the removed vertex
        // // this feels awfully inefficient, but lets go with this for now

        // // filter out all valid partitions
        // let mut valid_partitions: Vec<PartitionLine> = Vec::new();
        // while possible_partition_lines.len() > 0 {
        //     // pop shortest line and add to valid partitions
        //     valid_partitions.push(possible_partition_lines.pop().unwrap());

        //     // remove all now invalid lines
        //     let last_line = valid_partitions.last().unwrap();
        //     possible_partition_lines.retain(|x| !x.contains_other_middle(&last_line));
        // }

        // // valid partitions is now filled, build new polys from it
        // for partition in valid_partitions {
        //     convex_parts.push(Polygon::from_points(&vec![self.points[partition.from], self.points[partition.middle], self.points[partition.to]]));
        // }

        // return convex_parts;
    }
}
