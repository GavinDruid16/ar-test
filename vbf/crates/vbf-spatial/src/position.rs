use serde::{Deserialize, Serialize};
use vbf_types::{EntityUid, Key};
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldPosition {
    pub frame: Key,
    pub x_mm: i64,
    pub y_mm: i64,
    pub z_mm: i64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostedPosition {
    pub host: EntityUid,
    pub station: Option<Key>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Key>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum SpatialPosition {
    World(WorldPosition),
    Hosted(HostedPosition),
}
