use crate::WorldPose;
use serde::{Deserialize, Serialize};
use vbf_types::SimTime;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Velocity3 {
    pub x_mm_per_second: i64,
    pub y_mm_per_second: i64,
    pub z_mm_per_second: i64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AngularVelocity3 {
    pub yaw_millidegrees_per_second: i64,
    pub pitch_millidegrees_per_second: i64,
    pub roll_millidegrees_per_second: i64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionSample {
    pub sim_time: SimTime,
    pub pose: WorldPose,
    pub linear_velocity: Velocity3,
    pub angular_velocity: AngularVelocity3,
}