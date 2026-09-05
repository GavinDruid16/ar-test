use serde::{Deserialize, Serialize};

/// Optional structural restrictions applied after a field's base type matches.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "constraint", rename_all = "snake_case")]
pub enum FieldConstraint {
    IntegerRange {
        min: Option<i64>,
        max: Option<i64>,
    },
    UnsignedRange {
        min: Option<u64>,
        max: Option<u64>,
    },
    DecimalRange {
        min: Option<f64>,
        max: Option<f64>,
    },
    StringLength {
        min: Option<usize>,
        max: Option<usize>,
    },
    ArrayLength {
        min: Option<usize>,
        max: Option<usize>,
    },
    AllowedStrings {
        values: Vec<String>,
    },
}
