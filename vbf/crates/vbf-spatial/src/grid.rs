use serde::{Deserialize, Serialize};
use vbf_types::Key;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GridRole {
    Player,
    Referee,
    Rules,
    Debug,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HexOrientation {
    FlatTop,
    PointyTop,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridOverlay {
    pub key: Key,
    pub frame: Key,
    pub role: GridRole,
    pub orientation: HexOrientation,
    pub nominal_scale_mm: u64,
    pub rows: u32,
    pub columns: u32,
}
