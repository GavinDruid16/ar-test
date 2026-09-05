use crate::{
    ComponentData, DefinitionCatalog, EntityClass, EntityValidationIssue, EntityValidationReport,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vbf_schema::{SchemaContext, SchemaRegistry};
use vbf_types::{DefinitionUid, DisplayName, EntityUid, Key};

/// One instantiated VBF Entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub uid: EntityUid,
    pub key: Key,
    pub name: DisplayName,
    pub class: EntityClass,
    pub template: Option<DefinitionUid>,
    pub components: BTreeMap<Key, ComponentData>,
}

impl Entity {
    pub fn new(uid: EntityUid, key: Key, name: DisplayName, class: EntityClass) -> Self {
        Self {
            uid,
            key,
            name,
            class,
            template: None,
            components: BTreeMap::new(),
        }
    }

    pub fn with_template(mut self, template: DefinitionUid) -> Self {
        self.template = Some(template);
        self
    }

    pub fn set_component(&mut self, component: ComponentData) -> Option<ComponentData> {
        self.components
            .insert(component.schema.key.clone(), component)
    }

    pub fn component(&self, key: &str) -> Option<&ComponentData> {
        self.components.get(key)
    }

    pub fn validate(
        &self,
        schemas: &SchemaRegistry,
        definitions: &DefinitionCatalog,
        context: SchemaContext,
    ) -> EntityValidationReport {
        let mut report = EntityValidationReport::default();

        if !matches!(
            context,
            SchemaContext::InitialState | SchemaContext::RuntimeState
        ) {
            report.push(EntityValidationIssue::InvalidEntityContext(context));
            return report;
        }

        if let Some(template) = self.template {
            match definitions.resolve(template) {
                Ok(resolved) => {
                    if resolved.class != self.class {
                        report.push(EntityValidationIssue::TemplateClassMismatch {
                            entity_class: self.class,
                            template_class: resolved.class,
                        });
                    }
                }
                Err(error) => {
                    report.push(EntityValidationIssue::TemplateResolutionFailed(error));
                }
            }
        }

        for (map_key, component) in &self.components {
            if map_key != &component.schema.key {
                report.push(EntityValidationIssue::ComponentKeyMismatch {
                    map_key: map_key.clone(),
                    schema_key: component.schema.key.clone(),
                });
                continue;
            }

            if let Err(error) = component.validate(schemas, context) {
                report.push(EntityValidationIssue::InvalidComponent {
                    component: map_key.clone(),
                    error,
                });
            }
        }

        report
    }
}
