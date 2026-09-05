use crate::{FieldConstraint, FieldSchema, FieldType, PersistenceClass, SchemaContext};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use vbf_types::{
    Angle, BranchUid, CorrelationUid, DefinitionUid, DisplayName, Distance, EntityUid,
    EventSequence, EventUid, Key, PackageUid, RelationshipUid, RotationalSpeed, SimDuration,
    SimTime, SnapshotUid, SourceUid, Speed, StateRevision,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JsonValueKind {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}
impl JsonValueKind {
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(_) => Self::Bool,
            Value::Number(_) => Self::Number,
            Value::String(_) => Self::String,
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComponentValidationIssueKind {
    PayloadMustBeObject {
        actual: JsonValueKind,
    },
    MissingRequiredField,
    UnknownField,
    PersistenceMismatch {
        field_persistence: PersistenceClass,
        context: SchemaContext,
    },
    TypeMismatch {
        expected: FieldType,
        actual: JsonValueKind,
    },
    InvalidDomainValue {
        expected: FieldType,
        message: String,
    },
    ConstraintViolation {
        constraint: FieldConstraint,
        message: String,
    },
    InvalidArrayItem {
        index: usize,
        expected: FieldType,
        message: String,
    },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentValidationIssue {
    pub field: Option<String>,
    pub kind: ComponentValidationIssueKind,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ComponentValidationReport {
    pub issues: Vec<ComponentValidationIssue>,
}
impl ComponentValidationReport {
    pub fn push(&mut self, issue: ComponentValidationIssue) {
        self.issues.push(issue);
    }
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }
}
pub(crate) fn validate_field_value(
    field: &FieldSchema,
    value: &Value,
) -> Vec<ComponentValidationIssueKind> {
    let mut issues = Vec::new();
    if let Err(issue) = validate_type(&field.field_type, value) {
        issues.push(issue);
        return issues;
    }
    for constraint in &field.constraints {
        if let Some(issue) = validate_constraint(constraint, value) {
            issues.push(issue);
        }
    }
    issues
}
fn validate_type(expected: &FieldType, value: &Value) -> Result<(), ComponentValidationIssueKind> {
    let mismatch = || ComponentValidationIssueKind::TypeMismatch {
        expected: expected.clone(),
        actual: JsonValueKind::of(value),
    };
    match expected {
        FieldType::Bool if value.is_boolean() => Ok(()),
        FieldType::Integer if value.as_i64().is_some() => Ok(()),
        FieldType::UnsignedInteger if value.as_u64().is_some() => Ok(()),
        FieldType::Decimal if value.as_f64().is_some() => Ok(()),
        FieldType::String if value.is_string() => Ok(()),
        FieldType::DisplayName => validate_deserializable::<DisplayName>(expected, value),
        FieldType::Object if value.is_object() => Ok(()),
        FieldType::Key => validate_deserializable::<Key>(expected, value),
        FieldType::EntityUid => validate_deserializable::<EntityUid>(expected, value),
        FieldType::DefinitionUid => validate_deserializable::<DefinitionUid>(expected, value),
        FieldType::RelationshipUid => validate_deserializable::<RelationshipUid>(expected, value),
        FieldType::EventUid => validate_deserializable::<EventUid>(expected, value),
        FieldType::CorrelationUid => validate_deserializable::<CorrelationUid>(expected, value),
        FieldType::PackageUid => validate_deserializable::<PackageUid>(expected, value),
        FieldType::SourceUid => validate_deserializable::<SourceUid>(expected, value),
        FieldType::SnapshotUid => validate_deserializable::<SnapshotUid>(expected, value),
        FieldType::BranchUid => validate_deserializable::<BranchUid>(expected, value),
        FieldType::SimTime => validate_deserializable::<SimTime>(expected, value),
        FieldType::SimDuration => validate_deserializable::<SimDuration>(expected, value),
        FieldType::EventSequence => validate_deserializable::<EventSequence>(expected, value),
        FieldType::StateRevision => validate_deserializable::<StateRevision>(expected, value),
        FieldType::Distance => validate_deserializable::<Distance>(expected, value),
        FieldType::Speed => validate_deserializable::<Speed>(expected, value),
        FieldType::RotationalSpeed => validate_deserializable::<RotationalSpeed>(expected, value),
        FieldType::Angle => validate_deserializable::<Angle>(expected, value),
        FieldType::Array { items } => {
            let array = value.as_array().ok_or_else(mismatch)?;
            for (index, item) in array.iter().enumerate() {
                if let Err(issue) = validate_type(items, item) {
                    return Err(ComponentValidationIssueKind::InvalidArrayItem {
                        index,
                        expected: items.as_ref().clone(),
                        message: issue.to_string(),
                    });
                }
            }
            Ok(())
        }
        _ => Err(mismatch()),
    }
}
fn validate_deserializable<T>(
    expected: &FieldType,
    value: &Value,
) -> Result<(), ComponentValidationIssueKind>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value::<T>(value.clone())
        .map(|_| ())
        .map_err(|error| ComponentValidationIssueKind::InvalidDomainValue {
            expected: expected.clone(),
            message: error.to_string(),
        })
}
fn validate_constraint(
    constraint: &FieldConstraint,
    value: &Value,
) -> Option<ComponentValidationIssueKind> {
    let fail = |message: String| ComponentValidationIssueKind::ConstraintViolation {
        constraint: constraint.clone(),
        message,
    };
    match constraint {
        FieldConstraint::IntegerRange { min, max } => {
            let number = value.as_i64()?;
            if min.is_some_and(|minimum| number < minimum) {
                return Some(fail(format!("{number} is below minimum {min:?}")));
            }
            if max.is_some_and(|maximum| number > maximum) {
                return Some(fail(format!("{number} is above maximum {max:?}")));
            }
        }
        FieldConstraint::UnsignedRange { min, max } => {
            let number = value.as_u64()?;
            if min.is_some_and(|minimum| number < minimum) {
                return Some(fail(format!("{number} is below minimum {min:?}")));
            }
            if max.is_some_and(|maximum| number > maximum) {
                return Some(fail(format!("{number} is above maximum {max:?}")));
            }
        }
        FieldConstraint::DecimalRange { min, max } => {
            let number = value.as_f64()?;
            if min.is_some_and(|minimum| number < minimum) {
                return Some(fail(format!("{number} is below minimum {min:?}")));
            }
            if max.is_some_and(|maximum| number > maximum) {
                return Some(fail(format!("{number} is above maximum {max:?}")));
            }
        }
        FieldConstraint::StringLength { min, max } => {
            let length = value.as_str()?.chars().count();
            if min.is_some_and(|minimum| length < minimum) {
                return Some(fail(format!(
                    "string length {length} is below minimum {min:?}"
                )));
            }
            if max.is_some_and(|maximum| length > maximum) {
                return Some(fail(format!(
                    "string length {length} is above maximum {max:?}"
                )));
            }
        }
        FieldConstraint::ArrayLength { min, max } => {
            let length = value.as_array()?.len();
            if min.is_some_and(|minimum| length < minimum) {
                return Some(fail(format!(
                    "array length {length} is below minimum {min:?}"
                )));
            }
            if max.is_some_and(|maximum| length > maximum) {
                return Some(fail(format!(
                    "array length {length} is above maximum {max:?}"
                )));
            }
        }
        FieldConstraint::AllowedStrings { values } => {
            let string = value.as_str()?;
            if !values.iter().any(|allowed| allowed == string) {
                return Some(fail(format!("{string:?} is not an allowed value")));
            }
        }
    }
    None
}
impl fmt::Display for ComponentValidationIssueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadMustBeObject { actual } => {
                write!(f, "component payload must be an object, got {actual:?}")
            }
            Self::MissingRequiredField => write!(f, "required field is missing"),
            Self::UnknownField => write!(f, "field is not declared by this schema"),
            Self::PersistenceMismatch {
                field_persistence,
                context,
            } => write!(
                f,
                "field persistence {field_persistence:?} is not allowed in {context:?} context"
            ),
            Self::TypeMismatch { expected, actual } => {
                write!(f, "expected {expected:?}, got {actual:?}")
            }
            Self::InvalidDomainValue { expected, message } => {
                write!(f, "invalid {expected:?} value: {message}")
            }
            Self::ConstraintViolation { message, .. } => f.write_str(message),
            Self::InvalidArrayItem {
                index,
                expected,
                message,
            } => write!(f, "array item {index} is not valid {expected:?}: {message}"),
        }
    }
}
