use serde::{Deserialize, Serialize};

/// The five Core token classes share one Entity identity universe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityClass {
    Actor,
    Asset,
    Process,
    Condition,
    ObjectiveTask,
}
