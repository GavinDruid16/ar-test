use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fmt;
use vbf_schema::{ComponentSchemaRef, ComponentValidationReport, SchemaContext, SchemaRegistry};

/// One canonical component payload bound to one exact schema version.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentData {
    pub schema: ComponentSchemaRef,
    pub payload: Value,
}

impl ComponentData {
    pub fn new(schema: ComponentSchemaRef, payload: Value) -> Self {
        Self { schema, payload }
    }

    pub fn validate(
        &self,
        registry: &SchemaRegistry,
        context: SchemaContext,
    ) -> Result<(), ComponentDataError> {
        let Some(schema) = registry.get(self.schema.key.as_str(), self.schema.version) else {
            return Err(ComponentDataError::UnknownSchema(self.schema.clone()));
        };

        let report = schema.validate_payload(&self.payload, context);
        if report.is_valid() {
            Ok(())
        } else {
            Err(ComponentDataError::InvalidPayload {
                schema: self.schema.clone(),
                report,
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentDataError {
    UnknownSchema(ComponentSchemaRef),
    InvalidPayload {
        schema: ComponentSchemaRef,
        report: ComponentValidationReport,
    },
}

impl fmt::Display for ComponentDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSchema(schema) => write!(
                f,
                "component schema is not registered: {} v{}",
                schema.key, schema.version
            ),
            Self::InvalidPayload { schema, report } => write!(
                f,
                "component payload does not match {} v{} ({} issue(s))",
                schema.key,
                schema.version,
                report.issues.len()
            ),
        }
    }
}

impl Error for ComponentDataError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use vbf_schema::{ComponentSchema, FieldRequirement, FieldSchema, FieldType, PersistenceClass};
    use vbf_types::Key;

    fn registry() -> SchemaRegistry {
        let schema = ComponentSchema::new(
            Key::new("component.test_state").unwrap(),
            1,
            Key::new("module.test").unwrap(),
            vec![FieldSchema::new(
                Key::new("state").unwrap(),
                FieldType::Key,
                FieldRequirement::Required,
                PersistenceClass::Mutable,
            )],
        )
        .unwrap();

        let mut registry = SchemaRegistry::new();
        registry.register(schema).unwrap();
        registry
    }

    #[test]
    fn component_data_requires_registered_exact_schema_version() {
        let component = ComponentData::new(
            ComponentSchemaRef::new(Key::new("component.test_state").unwrap(), 2).unwrap(),
            json!({"state": "state.ready"}),
        );
        assert!(matches!(
            component.validate(&registry(), SchemaContext::RuntimeState),
            Err(ComponentDataError::UnknownSchema(_))
        ));
    }

    #[test]
    fn component_data_uses_schema_context_validation() {
        let component = ComponentData::new(
            ComponentSchemaRef::new(Key::new("component.test_state").unwrap(), 1).unwrap(),
            json!({"state": "state.ready"}),
        );
        assert!(
            component
                .validate(&registry(), SchemaContext::RuntimeState)
                .is_ok()
        );
        assert!(
            component
                .validate(&registry(), SchemaContext::Definition)
                .is_err()
        );
    }
}
