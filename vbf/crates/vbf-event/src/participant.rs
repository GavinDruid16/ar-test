use serde::{Deserialize, Serialize};
use vbf_types::{EntityUid, Key};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventParticipant {
    pub role: Key,
    pub entity: EntityUid,
}
impl EventParticipant {
    pub fn new(role: Key, entity: EntityUid) -> Self {
        Self { role, entity }
    }
}