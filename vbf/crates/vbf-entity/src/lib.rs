//! Canonical Entity and Definition records for VBF Layer 0.
//!
//! This crate is the first consumer of `vbf-schema`. Component payloads are
//! never stored as anonymous JSON: every payload carries an exact
//! `ComponentSchemaRef`, and validation occurs in Definition, InitialState, or
//! RuntimeState context as appropriate.
//!
//! Actor, Asset, Process, Condition, and Objective/Task remain classes within
//! one Entity identity universe. This crate does not own relationships,
//! spatial geometry, actions, or game-rule effects.

pub mod catalog;
pub mod class;
pub mod component;
pub mod definition;
pub mod entity;
pub mod validation;

pub use catalog::{
    DefinitionCatalog, DefinitionCatalogError, DefinitionResolutionError, EntityCatalog,
    EntityCatalogError, ResolvedDefinition,
};
pub use class::EntityClass;
pub use component::{ComponentData, ComponentDataError};
pub use definition::Definition;
pub use entity::Entity;
pub use validation::{EntityValidationIssue, EntityValidationReport};
