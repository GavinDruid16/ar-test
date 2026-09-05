use crate::{
    ComponentValidationIssue, ComponentValidationIssueKind, ComponentValidationReport,
    FieldConstraint, FieldSchema, FieldType, JsonValueKind, SchemaContext,
};
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use vbf_types::Key;

/// Stable reference to one exact component schema version.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ComponentSchemaRef {
    pub key: Key,
    pub version: u32,
}

impl ComponentSchemaRef {
    pub fn new(key: Key, version: u32) -> Result<Self, SchemaDefinitionError> {
        if version == 0 {
            return Err(SchemaDefinitionError::ZeroVersion);
        }
        Ok(Self { key, version })
    }
}

impl<'de> Deserialize<'de> for ComponentSchemaRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawSchemaRef {
            key: Key,
            version: u32,
        }

        let raw = RawSchemaRef::deserialize(deserializer)?;
        Self::new(raw.key, raw.version).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentSchema {
    pub key: Key,
    pub version: u32,
    pub owner: Key,
    pub fields: Vec<FieldSchema>,
    #[serde(default)]
    pub allow_unknown_fields: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SchemaDefinitionError {
    ZeroVersion,
    DuplicateField(Key),
    IncompatibleConstraint {
        field: Key,
        field_type: FieldType,
        constraint: FieldConstraint,
    },
    InvalidConstraintBounds {
        field: Key,
        constraint: FieldConstraint,
    },
    NonFiniteDecimalConstraint {
        field: Key,
    },
    EmptyAllowedStrings {
        field: Key,
    },
    InvalidAllowedString {
        field: Key,
        value: String,
    },
}

impl fmt::Display for SchemaDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroVersion => write!(f, "component schema version must be at least 1"),
            Self::DuplicateField(field) => {
                write!(f, "duplicate field in component schema: {field}")
            }
            Self::IncompatibleConstraint {
                field,
                field_type,
                constraint,
            } => write!(
                f,
                "constraint {constraint:?} is incompatible with field {field} type {field_type:?}"
            ),
            Self::InvalidConstraintBounds { field, constraint } => write!(
                f,
                "constraint bounds are invalid for field {field}: {constraint:?}"
            ),
            Self::NonFiniteDecimalConstraint { field } => write!(
                f,
                "decimal constraint for field {field} contains a non-finite bound"
            ),
            Self::EmptyAllowedStrings { field } => {
                write!(f, "allowed-string constraint for field {field} is empty")
            }
            Self::InvalidAllowedString { field, value } => write!(
                f,
                "allowed value {value:?} is invalid for Key field {field}"
            ),
        }
    }
}

impl Error for SchemaDefinitionError {}

impl ComponentSchema {
    pub fn new(
        key: Key,
        version: u32,
        owner: Key,
        fields: Vec<FieldSchema>,
    ) -> Result<Self, SchemaDefinitionError> {
        let schema = Self {
            key,
            version,
            owner,
            fields,
            allow_unknown_fields: false,
        };
        schema.validate_definition()?;
        Ok(schema)
    }

    pub fn schema_ref(&self) -> ComponentSchemaRef {
        ComponentSchemaRef {
            key: self.key.clone(),
            version: self.version,
        }
    }

    pub fn allow_unknown_fields(mut self, allow: bool) -> Self {
        self.allow_unknown_fields = allow;
        self
    }

    pub fn field(&self, key: &str) -> Option<&FieldSchema> {
        self.fields.iter().find(|field| field.key.as_str() == key)
    }

    pub fn validate_definition(&self) -> Result<(), SchemaDefinitionError> {
        if self.version == 0 {
            return Err(SchemaDefinitionError::ZeroVersion);
        }

        let mut keys = BTreeSet::new();
        for field in &self.fields {
            if !keys.insert(field.key.clone()) {
                return Err(SchemaDefinitionError::DuplicateField(field.key.clone()));
            }
            validate_field_constraints(field)?;
        }

        Ok(())
    }

    pub fn validate_payload(
        &self,
        payload: &Value,
        context: SchemaContext,
    ) -> ComponentValidationReport {
        let mut report = ComponentValidationReport::default();
        let Some(object) = payload.as_object() else {
            report.push(ComponentValidationIssue {
                field: None,
                kind: ComponentValidationIssueKind::PayloadMustBeObject {
                    actual: JsonValueKind::of(payload),
                },
            });
            return report;
        };

        for field in &self.fields {
            let present = object.get(field.key.as_str());
            let allowed = field.persistence.allows(context);

            if allowed && field.is_required() && present.is_none() {
                report.push(ComponentValidationIssue {
                    field: Some(field.key.to_string()),
                    kind: ComponentValidationIssueKind::MissingRequiredField,
                });
            }

            if let Some(value) = present {
                if !allowed {
                    report.push(ComponentValidationIssue {
                        field: Some(field.key.to_string()),
                        kind: ComponentValidationIssueKind::PersistenceMismatch {
                            field_persistence: field.persistence,
                            context,
                        },
                    });
                    continue;
                }

                for kind in crate::validation::validate_field_value(field, value) {
                    report.push(ComponentValidationIssue {
                        field: Some(field.key.to_string()),
                        kind,
                    });
                }
            }
        }

        if !self.allow_unknown_fields {
            for key in object.keys() {
                if self.field(key).is_none() {
                    report.push(ComponentValidationIssue {
                        field: Some(key.clone()),
                        kind: ComponentValidationIssueKind::UnknownField,
                    });
                }
            }
        }

        report
    }
}

fn validate_field_constraints(field: &FieldSchema) -> Result<(), SchemaDefinitionError> {
    for constraint in &field.constraints {
        let compatible = match constraint {
            FieldConstraint::IntegerRange { .. } => field.field_type == FieldType::Integer,
            FieldConstraint::UnsignedRange { .. } => field.field_type == FieldType::UnsignedInteger,
            FieldConstraint::DecimalRange { .. } => field.field_type == FieldType::Decimal,
            FieldConstraint::StringLength { .. } | FieldConstraint::AllowedStrings { .. } => {
                matches!(
                    &field.field_type,
                    FieldType::String | FieldType::DisplayName | FieldType::Key
                )
            }
            FieldConstraint::ArrayLength { .. } => {
                matches!(&field.field_type, FieldType::Array { .. })
            }
        };

        if !compatible {
            return Err(SchemaDefinitionError::IncompatibleConstraint {
                field: field.key.clone(),
                field_type: field.field_type.clone(),
                constraint: constraint.clone(),
            });
        }

        match constraint {
            FieldConstraint::IntegerRange { min, max } => {
                if bounds_reversed(*min, *max) {
                    return invalid_bounds(field, constraint);
                }
            }
            FieldConstraint::UnsignedRange { min, max } => {
                if bounds_reversed(*min, *max) {
                    return invalid_bounds(field, constraint);
                }
            }
            FieldConstraint::DecimalRange { min, max } => {
                if min.is_some_and(|value| !value.is_finite())
                    || max.is_some_and(|value| !value.is_finite())
                {
                    return Err(SchemaDefinitionError::NonFiniteDecimalConstraint {
                        field: field.key.clone(),
                    });
                }
                if bounds_reversed(*min, *max) {
                    return invalid_bounds(field, constraint);
                }
            }
            FieldConstraint::StringLength { min, max }
            | FieldConstraint::ArrayLength { min, max } => {
                if bounds_reversed(*min, *max) {
                    return invalid_bounds(field, constraint);
                }
            }
            FieldConstraint::AllowedStrings { values } => {
                if values.is_empty() {
                    return Err(SchemaDefinitionError::EmptyAllowedStrings {
                        field: field.key.clone(),
                    });
                }
                if matches!(&field.field_type, FieldType::Key) {
                    for value in values {
                        if Key::new(value.as_str()).is_err() {
                            return Err(SchemaDefinitionError::InvalidAllowedString {
                                field: field.key.clone(),
                                value: value.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn bounds_reversed<T: PartialOrd>(min: Option<T>, max: Option<T>) -> bool {
    matches!((min, max), (Some(minimum), Some(maximum)) if minimum > maximum)
}

fn invalid_bounds(
    field: &FieldSchema,
    constraint: &FieldConstraint,
) -> Result<(), SchemaDefinitionError> {
    Err(SchemaDefinitionError::InvalidConstraintBounds {
        field: field.key.clone(),
        constraint: constraint.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldRequirement, PersistenceClass};
    use serde_json::json;

    fn field(key: &str, field_type: FieldType, persistence: PersistenceClass) -> FieldSchema {
        FieldSchema::new(
            Key::new(key).unwrap(),
            field_type,
            FieldRequirement::Required,
            persistence,
        )
    }

    fn basic_schema(fields: Vec<FieldSchema>) -> ComponentSchema {
        ComponentSchema::new(
            Key::new("component.test").unwrap(),
            1,
            Key::new("module.layer0").unwrap(),
            fields,
        )
        .unwrap()
    }

    #[test]
    fn schema_rejects_zero_version() {
        let result = ComponentSchema::new(
            Key::new("component.test").unwrap(),
            0,
            Key::new("module.layer0").unwrap(),
            vec![],
        );
        assert!(matches!(result, Err(SchemaDefinitionError::ZeroVersion)));
    }

    #[test]
    fn schema_rejects_duplicate_fields() {
        let first = field("state", FieldType::String, PersistenceClass::Mutable);
        let second = field("state", FieldType::String, PersistenceClass::Mutable);
        let result = ComponentSchema::new(
            Key::new("component.test").unwrap(),
            1,
            Key::new("module.layer0").unwrap(),
            vec![first, second],
        );
        assert!(matches!(
            result,
            Err(SchemaDefinitionError::DuplicateField(_))
        ));
    }

    #[test]
    fn schema_rejects_incompatible_constraint() {
        let constrained = field("state", FieldType::Bool, PersistenceClass::Mutable)
            .with_constraint(FieldConstraint::StringLength {
                min: Some(1),
                max: Some(10),
            });
        assert!(matches!(
            ComponentSchema::new(
                Key::new("component.test").unwrap(),
                1,
                Key::new("module.layer0").unwrap(),
                vec![constrained],
            ),
            Err(SchemaDefinitionError::IncompatibleConstraint { .. })
        ));
    }

    #[test]
    fn schema_rejects_reversed_constraint_bounds() {
        let constrained = field(
            "count",
            FieldType::UnsignedInteger,
            PersistenceClass::Mutable,
        )
        .with_constraint(FieldConstraint::UnsignedRange {
            min: Some(10),
            max: Some(5),
        });
        assert!(matches!(
            ComponentSchema::new(
                Key::new("component.test").unwrap(),
                1,
                Key::new("module.layer0").unwrap(),
                vec![constrained],
            ),
            Err(SchemaDefinitionError::InvalidConstraintBounds { .. })
        ));
    }

    #[test]
    fn required_fields_are_context_sensitive() {
        let schema = basic_schema(vec![
            field(
                "length_mm",
                FieldType::UnsignedInteger,
                PersistenceClass::Definition,
            ),
            field("state", FieldType::String, PersistenceClass::Mutable),
        ]);

        let runtime = schema.validate_payload(
            &json!({"state": "operational"}),
            SchemaContext::RuntimeState,
        );
        assert!(runtime.is_valid());

        let definition =
            schema.validate_payload(&json!({"length_mm": 5000}), SchemaContext::Definition);
        assert!(definition.is_valid());
    }

    #[test]
    fn persistence_mismatch_is_reported() {
        let schema = basic_schema(vec![field(
            "length_mm",
            FieldType::UnsignedInteger,
            PersistenceClass::Definition,
        )]);
        let report =
            schema.validate_payload(&json!({"length_mm": 5000}), SchemaContext::RuntimeState);
        assert_eq!(report.issues.len(), 1);
        assert!(matches!(
            &report.issues[0].kind,
            ComponentValidationIssueKind::PersistenceMismatch { .. }
        ));
    }

    #[test]
    fn unknown_fields_are_rejected_by_default() {
        let schema = basic_schema(vec![]);
        let report =
            schema.validate_payload(&json!({"mystery": true}), SchemaContext::RuntimeState);
        assert!(matches!(
            &report.issues[0].kind,
            ComponentValidationIssueKind::UnknownField
        ));
    }

    #[test]
    fn unknown_fields_can_be_explicitly_allowed() {
        let schema = basic_schema(vec![]).allow_unknown_fields(true);
        let report =
            schema.validate_payload(&json!({"mystery": true}), SchemaContext::RuntimeState);
        assert!(report.is_valid());
    }

    #[test]
    fn typed_key_values_use_domain_validation() {
        let schema = basic_schema(vec![field(
            "frame",
            FieldType::Key,
            PersistenceClass::Mutable,
        )]);
        assert!(
            schema
                .validate_payload(
                    &json!({"frame": "frame.vaux_local"}),
                    SchemaContext::RuntimeState
                )
                .is_valid()
        );
        assert!(
            !schema
                .validate_payload(&json!({"frame": "BAD KEY"}), SchemaContext::RuntimeState)
                .is_valid()
        );
    }

    #[test]
    fn array_item_types_are_checked() {
        let schema = basic_schema(vec![field(
            "crew",
            FieldType::Array {
                items: Box::new(FieldType::EntityUid),
            },
            PersistenceClass::Mutable,
        )]);
        let report = schema.validate_payload(
            &json!({"crew": ["not-a-uuid"]}),
            SchemaContext::RuntimeState,
        );
        assert!(matches!(
            &report.issues[0].kind,
            ComponentValidationIssueKind::InvalidArrayItem { .. }
        ));
    }

    #[test]
    fn string_constraints_are_enforced() {
        let constrained = field("state", FieldType::String, PersistenceClass::Mutable)
            .with_constraint(FieldConstraint::AllowedStrings {
                values: vec!["operational".into(), "disabled".into()],
            });
        let schema = basic_schema(vec![constrained]);
        assert!(
            schema
                .validate_payload(
                    &json!({"state": "operational"}),
                    SchemaContext::RuntimeState
                )
                .is_valid()
        );
        assert!(
            !schema
                .validate_payload(&json!({"state": "flying"}), SchemaContext::RuntimeState)
                .is_valid()
        );
    }
}
