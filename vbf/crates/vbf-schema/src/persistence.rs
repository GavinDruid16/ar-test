use crate::SchemaContext;
use serde::{Deserialize, Serialize};

/// Declares where a field is allowed to exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceClass {
    /// Reusable content/template data. Never authoritative instance state.
    Definition,

    /// Per-instance state fixed at scenario boot and not ordinarily mutated.
    Initial,

    /// Authoritative instance state that may change through validated mutations.
    Mutable,

    /// Data that belongs only inside an Event record.
    Event,

    /// Read-only output calculated from other state.
    Derived,

    /// Temporary UI/cache/session data with no canonical persistence.
    Ephemeral,
}

impl PersistenceClass {
    /// Return whether this field may legally appear in the requested context.
    pub const fn allows(self, context: SchemaContext) -> bool {
        match self {
            Self::Definition => matches!(context, SchemaContext::Definition),
            Self::Initial => matches!(
                context,
                SchemaContext::InitialState | SchemaContext::RuntimeState
            ),
            Self::Mutable => matches!(
                context,
                SchemaContext::InitialState | SchemaContext::RuntimeState
            ),
            Self::Event => matches!(context, SchemaContext::Event),
            Self::Derived => matches!(context, SchemaContext::Derived),
            Self::Ephemeral => matches!(context, SchemaContext::Ephemeral),
        }
    }

    /// Canonical persisted data excludes derived and ephemeral values.
    pub const fn is_canonical(self) -> bool {
        !matches!(self, Self::Derived | Self::Ephemeral)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_fields_are_allowed_in_initial_and_runtime_state() {
        assert!(PersistenceClass::Mutable.allows(SchemaContext::InitialState));
        assert!(PersistenceClass::Mutable.allows(SchemaContext::RuntimeState));
        assert!(!PersistenceClass::Mutable.allows(SchemaContext::Definition));
    }

    #[test]
    fn derived_and_ephemeral_fields_are_not_canonical() {
        assert!(!PersistenceClass::Derived.is_canonical());
        assert!(!PersistenceClass::Ephemeral.is_canonical());
        assert!(PersistenceClass::Mutable.is_canonical());
    }
}
