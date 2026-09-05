use crate::{FieldConstraint, PersistenceClass};
use serde::{Deserialize, Serialize};
use vbf_types::Key;
/// Serialized value shape expected for one field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Bool,
    Integer,
    UnsignedInteger,
    Decimal,
    String,
    DisplayName,
    Key,
    EntityUid,
    DefinitionUid,
    RelationshipUid,
    EventUid,
    CorrelationUid,
    PackageUid,
    SourceUid,
    SnapshotUid,
    BranchUid,
    SimTime,
    SimDuration,
    EventSequence,
    StateRevision,
    Distance,
    Speed,
    RotationalSpeed,
    Angle,
    Object,
    Array { items: Box<FieldType> },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldRequirement {
    Required,
    Optional,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldSchema {
    pub key: Key,
    pub field_type: FieldType,
    pub requirement: FieldRequirement,
    pub persistence: PersistenceClass,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<FieldConstraint>,
}
impl FieldSchema {
    pub fn new(
        key: Key,
        field_type: FieldType,
        requirement: FieldRequirement,
        persistence: PersistenceClass,
    ) -> Self {
        Self {
            key,
            field_type,
            requirement,
            persistence,
            constraints: Vec::new(),
        }
    }
    pub fn with_constraint(mut self, constraint: FieldConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }
    pub fn is_required(&self) -> bool {
        self.requirement == FieldRequirement::Required
    }
}
