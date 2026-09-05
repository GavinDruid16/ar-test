use serde::{Deserialize, Serialize};
use vbf_types::{DisplayName, Key};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisDirection {
    East,
    West,
    North,
    South,
    Up,
    Down,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinateFrame {
    pub key: Key,
    pub name: DisplayName,
    pub x_axis: AxisDirection,
    pub y_axis: AxisDirection,
    pub z_axis: AxisDirection,
}
