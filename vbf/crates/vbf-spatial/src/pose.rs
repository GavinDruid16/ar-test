use crate::WorldPosition;
use serde::{Deserialize, Serialize};
use vbf_types::Angle;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vector3Mm {
    pub x_mm: i64,
    pub y_mm: i64,
    pub z_mm: i64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Orientation3 {
    pub yaw: Angle,
    pub pitch: Angle,
    pub roll: Angle,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalPose {
    pub offset: Vector3Mm,
    pub orientation: Orientation3,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldPose {
    pub position: WorldPosition,
    pub orientation: Orientation3,
}