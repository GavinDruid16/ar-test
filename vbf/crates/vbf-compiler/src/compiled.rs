use crate::Layer0Source;
use std::error::Error;
use std::fmt;
use vbf_store::WorldState;
use vbf_types::{EntityUid, Key};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    DuplicateEntityUid(EntityUid),
    DuplicateEntityKey(Key),
    DuplicateInformationKey(Key),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEntityUid(uid) => write!(f, "duplicate entity UID: {uid}"),
            Self::DuplicateEntityKey(key) => write!(f, "duplicate entity key: {key}"),
            Self::DuplicateInformationKey(key) => write!(f, "duplicate information key: {key}"),
        }
    }
}

impl Error for CompileError {}

#[derive(Clone, Debug)]
pub struct CompiledLayer0 {
    pub source: Layer0Source,
    pub initial_state: WorldState,
}

pub fn compile_layer0(source: Layer0Source) -> Result<CompiledLayer0, CompileError> {
    let mut state = WorldState::default();
    let mut entity_keys = std::collections::BTreeSet::new();

    for entity in &source.entities {
        if state.entities.contains_key(&entity.uid) {
            return Err(CompileError::DuplicateEntityUid(entity.uid));
        }
        if !entity_keys.insert(entity.key.clone()) {
            return Err(CompileError::DuplicateEntityKey(entity.key.clone()));
        }
        state.entities.insert(entity.uid, entity.clone());
    }

    for relationship in &source.relationships {
        state
            .relationships
            .insert(relationship.uid, relationship.clone());
    }

    for record in &source.information {
        if state
            .information
            .insert(record.key.clone(), record.clone())
            .is_some()
        {
            return Err(CompileError::DuplicateInformationKey(record.key.clone()));
        }
    }

    Ok(CompiledLayer0 {
        source,
        initial_state: state,
    })
}
