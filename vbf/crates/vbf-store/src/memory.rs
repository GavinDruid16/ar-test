use crate::{CommitError, WorldState, commit::commit_event as apply_event_commit};
use vbf_event::EventEnvelope;
use vbf_types::{EntityUid, RelationshipUid};
#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    state: WorldState,
    events: Vec<EventEnvelope>,
}
impl MemoryStore {
    pub fn new(state: WorldState) -> Self {
        Self {
            state,
            events: Vec::new(),
        }
    }
    pub fn state(&self) -> &WorldState {
        &self.state
    }
    pub fn events(&self) -> &[EventEnvelope] {
        &self.events
    }
    pub fn last_event(&self) -> Option<&EventEnvelope> {
        self.events.last()
    }
    pub fn commit_event(&mut self, event: EventEnvelope) -> Result<(), CommitError> {
        apply_event_commit(&mut self.state, &mut self.events, event)
    }
    pub fn has_entity(&self, uid: EntityUid) -> bool {
        self.state.entities.contains_key(&uid)
    }
    pub fn has_relationship(&self, uid: RelationshipUid) -> bool {
        self.state.relationships.contains_key(&uid)
    }
}
