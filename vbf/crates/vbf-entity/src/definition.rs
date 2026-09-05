use crate::{ComponentData, EntityClass, EntityValidationIssue, EntityValidationReport};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vbf_schema::{SchemaContext, SchemaRegistry};
use vbf_types::{DefinitionUid, DisplayName, Key};

/// Reusable content/template data.
///
/// Definitions may extend one parent Definition. A child component with the
/// same component key replaces the parent's component as a complete payload;
/// Layer 0 does not perform implicit field-level patching.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Definition {
    pub uid: DefinitionUid,
    pub key: Key,
    pub name: DisplayName,
    pub class: EntityClass,
    pub extends: Option<DefinitionUid>,
    pub components: BTreeMap<Key, ComponentData>,
}

impl Definition {
    pub fn new(uid: DefinitionUid, key: Key, name: DisplayName, class: EntityClass) -> Self {
        Self {
            uid,
            key,
            name,
            class,
            extends: None,
            components: BTreeMap::new(),
        }
    }

    pub fn with_parent(mut self, parent: DefinitionUid) -> Self {
        self.extends = Some(parent);
        self
    }

    pub fn set_component(&mut self, component: ComponentData) -> Option<ComponentData> {
        self.components
            .insert(component.schema.key.clone(), component)
    }

    pub fn component(&self, key: &str) -> Option<&ComponentData> {
        self.components.get(key)
    }

    pub fn validate_components(&self, registry: &SchemaRegistry) -> EntityValidationReport {
        let mut report = EntityValidationReport::default();
        for (map_key, component) in &self.components {
            if map_key != &component.schema.key {
                report.push(EntityValidationIssue::ComponentKeyMismatch {
                    map_key: map_key.clone(),
                    schema_key: component.schema.key.clone(),
                });
                continue;
            }

            if let Err(error) = component.validate(registry, SchemaContext::Definition) {
                report.push(EntityValidationIssue::InvalidComponent {
                    component: map_key.clone(),
                    error,
                });
            }
        }
        report
    }
}
