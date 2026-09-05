use crate::{ComponentDataError, DefinitionResolutionError, EntityClass};
use vbf_schema::SchemaContext;
use vbf_types::Key;

#[derive(Clone, Debug, PartialEq)]
pub enum EntityValidationIssue {
    ComponentKeyMismatch {
        map_key: Key,
        schema_key: Key,
    },
    InvalidComponent {
        component: Key,
        error: ComponentDataError,
    },
    InvalidEntityContext(SchemaContext),
    TemplateResolutionFailed(DefinitionResolutionError),
    TemplateClassMismatch {
        entity_class: EntityClass,
        template_class: EntityClass,
    },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EntityValidationReport {
    pub issues: Vec<EntityValidationIssue>,
}

impl EntityValidationReport {
    pub fn push(&mut self, issue: EntityValidationIssue) {
        self.issues.push(issue);
    }

    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }
}
