use serde::{Deserialize, Serialize};

/// The kind of record currently being validated against a component schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaContext {
    Definition,
    InitialState,
    RuntimeState,
    Event,
    Derived,
    Ephemeral,
}
