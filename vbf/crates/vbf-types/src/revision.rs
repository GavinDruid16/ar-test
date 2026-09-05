//! Monotonic authoritative-state revision numbers.
//!
//! A revision identifies which committed authoritative state a query used.
//! It is not simulation time and is not an Event UID.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevisionError {
    Overflow,
}

impl fmt::Display for RevisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow => write!(f, "state revision overflowed"),
        }
    }
}

impl Error for RevisionError {}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct StateRevision(u64);

impl StateRevision {
    pub const INITIAL: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn next(self) -> Result<Self, RevisionError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(RevisionError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_revision_is_zero() {
        assert_eq!(StateRevision::INITIAL.get(), 0);
    }

    #[test]
    fn revision_advances_monotonically() {
        let r0 = StateRevision::INITIAL;
        let r1 = r0.next().expect("revision should advance");
        let r2 = r1.next().expect("revision should advance");

        assert!(r0 < r1);
        assert!(r1 < r2);
    }

    #[test]
    fn revision_overflow_is_detected() {
        let maximum = StateRevision::new(u64::MAX);
        assert_eq!(maximum.next(), Err(RevisionError::Overflow));
    }
}
