use super::{Point, Line, LineSegment, Vector};
use std::iter::FromIterator;

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

    // public helper to add a point to the end of the points list
    pub fn add_point(mut self, pt: Point) {
        self.points.push(pt)
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

    // public static square from side length
    pub fn square(len: f64) -> Polygon {
        let pt0 = Point::new().set_values(-len / 2.0, -len / 2.0);
        let pt1 = Point::new().set_values(len / 2.0, -len / 2.0);
        let pt2 = Point::new().set_values(len / 2.0, len / 2.0);
        let pt3 = Point::new().set_values(-len / 2.0, len / 2.0);

        Polygon::from_points(&vec![pt0, pt1, pt2, pt3])
    }
}
