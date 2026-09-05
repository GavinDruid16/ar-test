//! Schema metadata and structural component validation for Layer 0.
//!
//! `vbf-schema` answers four questions:
//!
//! - Which component schemas exist, and at which versions?
//! - Which fields are legal in each component?
//! - Which persistence context may contain each field?
//! - Does a serialized component payload structurally match its schema?
//!
//! This crate does not resolve battlefield rules and does not resolve entity
//! references. Those responsibilities belong to higher Layer 0/domain code.

pub mod component;
pub mod constraint;
pub mod context;
pub mod field;
pub mod persistence;
pub mod registry;
pub mod validation;

pub use component::{ComponentSchema, ComponentSchemaRef, SchemaDefinitionError};
pub use constraint::FieldConstraint;
pub use context::SchemaContext;
pub use field::{FieldRequirement, FieldSchema, FieldType};
pub use persistence::PersistenceClass;
pub use registry::{RegistryError, SchemaRegistry};
pub use validation::{
    ComponentValidationIssue, ComponentValidationIssueKind, ComponentValidationReport,
    JsonValueKind,
};
