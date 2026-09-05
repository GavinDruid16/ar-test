use crate::RelationshipSchemaRef;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vbf_types::{EntityUid, Key, RelationshipUid, SimTime};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipParticipant {
    pub entity: EntityUid,
    pub role: Key,
    pub slot: Option<Key>,
}

impl RelationshipParticipant {
    pub fn new(entity: EntityUid, role: Key) -> Self {
        Self {
            entity,
            role,
            slot: None,
        }
    }

    pub fn in_slot(mut self, slot: Key) -> Self {
        self.slot = Some(slot);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub uid: RelationshipUid,
    pub key: Key,
    pub schema: RelationshipSchemaRef,
    pub participants: Vec<RelationshipParticipant>,
    pub properties: Value,
    pub valid_from: Option<SimTime>,
    pub valid_to: Option<SimTime>,
}

impl Relationship {
    pub fn new(
        uid: RelationshipUid,
        key: Key,
        schema: RelationshipSchemaRef,
        participants: Vec<RelationshipParticipant>,
        properties: Value,
    ) -> Self {
        Self {
            uid,
            key,
            schema,
            participants,
            properties,
            valid_from: None,
            valid_to: None,
        }
    }

    pub fn with_validity(mut self, valid_from: Option<SimTime>, valid_to: Option<SimTime>) -> Self {
        self.valid_from = valid_from;
        self.valid_to = valid_to;
        self
    }

    pub fn involves(&self, entity: EntityUid) -> bool {
        self.participants
            .iter()
            .any(|participant| participant.entity == entity)
    }

    pub fn participants_in_role(
        &self,
        role: &str,
    ) -> impl Iterator<Item = &RelationshipParticipant> {
        self.participants
            .iter()
            .filter(move |participant| participant.role.as_str() == role)
    }

    /// Relationship intervals are start-inclusive and end-exclusive.
    ///
    /// This permits an old assignment to end at the same simulation instant
    /// that a replacement assignment begins without creating an overlap.
    pub fn active_at(&self, time: SimTime) -> bool {
        let has_started = self.valid_from.is_none_or(|valid_from| valid_from <= time);
        let has_not_ended = self.valid_to.is_none_or(|valid_to| time < valid_to);
        has_started && has_not_ended
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_relationship() -> Relationship {
        Relationship::new(
            RelationshipUid::new(),
            Key::new("relationship.instance.test").unwrap(),
            RelationshipSchemaRef::new(Key::new("relationship.test").unwrap(), 1).unwrap(),
            vec![],
            serde_json::json!({}),
        )
    }

    #[test]
    fn validity_is_start_inclusive_and_end_exclusive() {
        let relationship = test_relationship().with_validity(
            Some(SimTime::from_millis(1_000)),
            Some(SimTime::from_millis(2_000)),
        );

        assert!(!relationship.active_at(SimTime::from_millis(999)));
        assert!(relationship.active_at(SimTime::from_millis(1_000)));
        assert!(relationship.active_at(SimTime::from_millis(1_999)));
        assert!(!relationship.active_at(SimTime::from_millis(2_000)));
    }
}
