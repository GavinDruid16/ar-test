use crate::Geometry2;
use serde::{Deserialize, Serialize};
use vbf_types::{DisplayName, Key};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialRegion {
    pub key: Key,
    pub name: DisplayName,
    pub frame: Key,
    pub geometry: Geometry2,
}
