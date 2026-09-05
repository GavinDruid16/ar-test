use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point2 {
    pub x_mm: i64,
    pub y_mm: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Geometry2 {
    Point { point: Point2 },
    LineString { points: Vec<Point2> },
    Polygon { exterior: Vec<Point2> },
}
