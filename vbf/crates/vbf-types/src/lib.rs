//! Foundational value types shared by VBF.
//!
//! This crate deliberately contains no battlefield rules. It defines small,
//! validated types that higher layers can trust.
pub mod id;
pub mod quantity;
pub mod revision;
pub mod time;
pub use id::{
    BranchUid, CorrelationUid, DefinitionUid, DisplayName, DisplayNameError, EntityUid, EventUid,
    IdParseError, Key, KeyError, PackageUid, RelationshipUid, SnapshotUid, SourceUid,
};
pub use quantity::{Angle, Distance, QuantityError, RotationalSpeed, Speed};
pub use revision::{RevisionError, StateRevision};
pub use time::{EventSequence, SimDuration, SimTime, TimeError};
