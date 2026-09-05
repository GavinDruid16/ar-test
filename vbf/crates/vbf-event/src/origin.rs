use serde::{Deserialize, Serialize};
use vbf_types::{EntityUid, Key};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventOrigin {
    System,
    Referee,
    Action { action: Key, actor: EntityUid },
    Process { process: EntityUid },
    Import,
}
