use crate::{EventOrigin, EventParticipant, EventTypeRef, StateMutation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vbf_types::{CorrelationUid, EntityUid, EventSequence, EventUid, SimTime};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub uid: EventUid,
    pub sequence: EventSequence,
    pub sim_time: SimTime,
    pub event_type: EventTypeRef,
    pub origin: EventOrigin,
    pub cause: Option<EventUid>,
    pub correlation: Option<CorrelationUid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participants: Vec<EventParticipant>,
    pub payload: Value,
    pub mutations: Vec<StateMutation>,
}
impl EventEnvelope {
    pub fn participants_with_role<'a>(
        &'a self,
        role: &'a str,
    ) -> impl Iterator<Item = &'a EventParticipant> + 'a {
        self.participants
            .iter()
            .filter(move |participant| participant.role.as_str() == role)
    }
    pub fn participant(&self, role: &str) -> Option<EntityUid> {
        self.participants
            .iter()
            .find(|participant| participant.role.as_str() == role)
            .map(|participant| participant.entity)
    }
}