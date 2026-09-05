pub mod epistemic;
pub mod provenance;
pub mod record;

pub use epistemic::{EpistemicKind, EpistemicValue};
pub use provenance::{InformationSource, ProvenanceRef};
pub use record::{InformationRecord, InformationSubject};
