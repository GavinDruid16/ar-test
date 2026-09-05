//! Integer simulation time and strict event ordering.
//!
//! Simulation time is not wall-clock time. VBF stores time relative to a
//! scenario epoch using integer milliseconds. Multiple events may share one
//! simulation time; `EventSequence` supplies strict historical order.

use serde::{Deserialize, Deserializer, Serialize, de};
use std::error::Error;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeError {
    NegativeDuration,
    Overflow,
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeDuration => write!(f, "simulation duration cannot be negative"),
            Self::Overflow => write!(f, "simulation time arithmetic overflowed"),
        }
    }
}

impl Error for TimeError {}

/// Milliseconds relative to the scenario epoch.
///
/// Negative values are permitted so scenario setup or imported historical
/// records can refer to times before the declared epoch when required.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct SimTime(i64);

impl SimTime {
    pub const ZERO: Self = Self(0);

    pub const fn from_millis(milliseconds: i64) -> Self {
        Self(milliseconds)
    }

    pub const fn as_millis(self) -> i64 {
        self.0
    }

    pub fn checked_add(self, duration: SimDuration) -> Result<Self, TimeError> {
        self.0
            .checked_add(duration.0)
            .map(Self)
            .ok_or(TimeError::Overflow)
    }

    pub fn checked_sub(self, duration: SimDuration) -> Result<Self, TimeError> {
        self.0
            .checked_sub(duration.0)
            .map(Self)
            .ok_or(TimeError::Overflow)
    }
}

/// Non-negative elapsed simulation time in milliseconds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SimDuration(i64);

impl SimDuration {
    pub const ZERO: Self = Self(0);

    pub fn from_millis(milliseconds: i64) -> Result<Self, TimeError> {
        if milliseconds < 0 {
            Err(TimeError::NegativeDuration)
        } else {
            Ok(Self(milliseconds))
        }
    }

    pub fn from_seconds(seconds: i64) -> Result<Self, TimeError> {
        let milliseconds = seconds.checked_mul(1_000).ok_or(TimeError::Overflow)?;
        Self::from_millis(milliseconds)
    }

    pub const fn as_millis(self) -> i64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for SimDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let milliseconds = i64::deserialize(deserializer)?;
        Self::from_millis(milliseconds).map_err(de::Error::custom)
    }
}

/// Strict append order for Events.
///
/// This is deliberately separate from `SimTime`: two events can happen at the
/// same battlefield time and still have a deterministic history order.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct EventSequence(u64);

impl EventSequence {
    pub const ZERO: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}

impl Add<SimDuration> for SimTime {
    type Output = Result<SimTime, TimeError>;

    fn add(self, rhs: SimDuration) -> Self::Output {
        self.checked_add(rhs)
    }
}

impl Sub<SimDuration> for SimTime {
    type Output = Result<SimTime, TimeError>;

    fn sub(self, rhs: SimDuration) -> Self::Output {
        self.checked_sub(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_rejects_negative_values() {
        assert_eq!(
            SimDuration::from_millis(-1),
            Err(TimeError::NegativeDuration)
        );
        assert!(serde_json::from_str::<SimDuration>("-1").is_err());
    }

    #[test]
    fn time_addition_uses_integer_milliseconds() {
        let start = SimTime::from_millis(10_000);
        let duration = SimDuration::from_seconds(8).expect("valid duration");
        assert_eq!((start + duration).expect("no overflow").as_millis(), 18_000);
    }

    #[test]
    fn equal_sim_times_can_have_distinct_sequence() {
        let time_a = SimTime::from_millis(858_000);
        let time_b = SimTime::from_millis(858_000);
        let seq_a = EventSequence::new(431);
        let seq_b = seq_a.next().expect("sequence should advance");

        assert_eq!(time_a, time_b);
        assert!(seq_a < seq_b);
    }

    #[test]
    fn time_round_trips_through_json() {
        let original = SimTime::from_millis(858_000);
        let json = serde_json::to_string(&original).expect("time should serialize");
        let restored: SimTime = serde_json::from_str(&json).expect("time should deserialize");
        assert_eq!(original, restored);
    }
}
