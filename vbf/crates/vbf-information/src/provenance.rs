use serde::{Deserialize, Serialize};
use vbf_types::{EntityUid, EventUid, Key, SourceUid};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InformationSource {
    DirectObservation,
    Entity { entity: EntityUid },
    Event { event: EventUid },
    SourceRecord { source: SourceUid },
    Imported { reference: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceRef {
    pub field: Key,
    pub sources: Vec<InformationSource>,
}
