use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpistemicKind {
    Exact,
    Approximate,
    Range,
    Qualitative,
    PossibleSet,
    Unknown,
    Unassessed,
    Contradicted,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpistemicValue {
    pub kind: EpistemicKind,
    pub value: Option<Value>,
}
