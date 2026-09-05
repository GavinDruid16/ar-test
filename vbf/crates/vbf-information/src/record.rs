use crate::{EpistemicValue, InformationSource};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vbf_types::{EntityUid, Key, SimTime};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InformationSubject {
    Entity { entity: EntityUid },
    KeyedRecord { key: Key },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationRecord {
    pub key: Key,
    pub holder: EntityUid,
    pub subject: InformationSubject,
    pub acquired_at: SimTime,
    pub source: InformationSource,
    pub claims: BTreeMap<Key, EpistemicValue>,
}
