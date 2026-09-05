//! First-class relationship records for VBF Layer 0.
//!
//! Relationship meaning is data-driven. Core and domain rules packages
//! register versioned relationship schemas; this crate owns the structural
//! substrate that makes those relationships safe to store and validate.

pub mod catalog;
pub mod registry;
pub mod relationship;
pub mod schema;
pub mod validation;

pub use catalog::{RelationshipCatalog, RelationshipCatalogError};
pub use registry::{RelationshipRegistryError, RelationshipSchemaRegistry};
pub use relationship::{Relationship, RelationshipParticipant};
pub use schema::{
    ParticipantRoleSchema, RelationshipRule, RelationshipSchema, RelationshipSchemaDefinitionError,
    RelationshipSchemaRef, SlotRequirement,
};
pub use validation::{
    RelationshipValidationIssue, RelationshipValidationIssueKind, RelationshipValidationReport,
    validate_relationship,
};
