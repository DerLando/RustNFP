
pub use self::point::Point;
mod point;

pub use self::line::Line;
mod line;

pub use self::line::LinePointRelation;

pub use self::polygon::Polygon;
mod polygon;

pub use self::polygon::PolygonEdgeRelation;

pub use self::line_segment::LineSegment;
mod line_segment;

pub use self::intersection::Intersection;
pub mod intersection;

pub use self::intersection::{LineLineIntersectionResult, PolygonPolygonIntersectionResult, LineSegmentLineSegmentIntersectionResult};

pub use self::vector::Vector;
mod vector;

pub mod constants{
    // unset value for undefined geometry
    pub const UNSET_VALUE: f64 = -123456.789;
    pub const ZERO_TOLERANCE: f64 = 0.000001;
}