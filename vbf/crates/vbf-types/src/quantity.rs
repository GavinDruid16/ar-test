//! Small, explicit physical quantity types.
//!
//! VBF should not pass unlabelled `f64` values around for distance, speed, or
//! heading. These wrappers establish canonical integer storage while allowing
//! authoring/import layers to perform unit conversion separately.
use serde::{Deserialize, Deserializer, Serialize, de};
use std::error::Error;
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityError {
    NegativeDistance,
    NegativeSpeed,
    NegativeRotationalSpeed,
    NonFiniteValue,
    OutOfRange,
}
impl fmt::Display for QuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeDistance => write!(f, "distance cannot be negative"),
            Self::NegativeSpeed => write!(f, "speed cannot be negative"),
            Self::NegativeRotationalSpeed => write!(f, "rotational speed cannot be negative"),
            Self::NonFiniteValue => write!(f, "quantity must be a finite number"),
            Self::OutOfRange => write!(f, "quantity is outside the supported range"),
        }
    }
}
impl Error for QuantityError {}
/// Non-negative distance in integer millimetres.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Distance(u64);
impl Distance {
    pub const ZERO: Self = Self(0);
    pub const fn from_millimetres(mm: u64) -> Self {
        Self(mm)
    }
    pub fn from_metres(metres: f64) -> Result<Self, QuantityError> {
        if !metres.is_finite() {
            return Err(QuantityError::NonFiniteValue);
        }
        if metres < 0.0 {
            return Err(QuantityError::NegativeDistance);
        }
        let mm = metres * 1_000.0;
        if mm > u64::MAX as f64 {
            return Err(QuantityError::OutOfRange);
        }
        Ok(Self(mm.round() as u64))
    }
    pub const fn as_millimetres(self) -> u64 {
        self.0
    }
    pub fn as_metres(self) -> f64 {
        self.0 as f64 / 1_000.0
    }
}
/// Non-negative speed in integer millimetres per second.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Speed(u64);
impl Speed {
    pub const ZERO: Self = Self(0);
    pub const fn from_mm_per_second(value: u64) -> Self {
        Self(value)
    }
    pub fn from_metres_per_second(value: f64) -> Result<Self, QuantityError> {
        if !value.is_finite() {
            return Err(QuantityError::NonFiniteValue);
        }
        if value < 0.0 {
            return Err(QuantityError::NegativeSpeed);
        }
        let mm_per_second = value * 1_000.0;
        if mm_per_second > u64::MAX as f64 {
            return Err(QuantityError::OutOfRange);
        }
        Ok(Self(mm_per_second.round() as u64))
    }
    pub fn from_kilometres_per_hour(value: f64) -> Result<Self, QuantityError> {
        Self::from_metres_per_second(value / 3.6)
    }
    pub const fn as_mm_per_second(self) -> u64 {
        self.0
    }
    pub fn as_metres_per_second(self) -> f64 {
        self.0 as f64 / 1_000.0
    }
}
/// Non-negative rotational speed in integer milli-revolutions per minute.
///
/// This is a canonical physical quantity. Rendering or domain adapters may
/// convert it to ordinary RPM without storing an unlabelled decimal as truth.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct RotationalSpeed(u64);
impl RotationalSpeed {
    pub const ZERO: Self = Self(0);
    pub const fn from_milli_rpm(value: u64) -> Self {
        Self(value)
    }
    pub fn from_rpm(value: f64) -> Result<Self, QuantityError> {
        if !value.is_finite() {
            return Err(QuantityError::NonFiniteValue);
        }
        if value < 0.0 {
            return Err(QuantityError::NegativeRotationalSpeed);
        }
        let milli_rpm = value * 1_000.0;
        if milli_rpm > u64::MAX as f64 {
            return Err(QuantityError::OutOfRange);
        }
        Ok(Self(milli_rpm.round() as u64))
    }
    pub const fn as_milli_rpm(self) -> u64 {
        self.0
    }
    pub fn as_rpm(self) -> f64 {
        self.0 as f64 / 1_000.0
    }
}
/// Heading/orientation normalized to [0, 360) degrees.
///
/// Stored as integer millidegrees so authoritative state is deterministic while
/// remaining much more precise than the rules will ordinarily require.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Angle(u32);
impl Angle {
    const FULL_CIRCLE_MILLIDEGREES: i64 = 360_000;
    pub fn from_millidegrees(value: u32) -> Result<Self, QuantityError> {
        if value >= Self::FULL_CIRCLE_MILLIDEGREES as u32 {
            Err(QuantityError::OutOfRange)
        } else {
            Ok(Self(value))
        }
    }
    pub fn from_degrees(degrees: f64) -> Result<Self, QuantityError> {
        if !degrees.is_finite() {
            return Err(QuantityError::NonFiniteValue);
        }
        let milli = (degrees * 1_000.0).round();
        if milli < i64::MIN as f64 || milli > i64::MAX as f64 {
            return Err(QuantityError::OutOfRange);
        }
        let normalized = (milli as i64).rem_euclid(Self::FULL_CIRCLE_MILLIDEGREES);
        Ok(Self(normalized as u32))
    }
    pub const fn as_millidegrees(self) -> u32 {
        self.0
    }
    pub fn as_degrees(self) -> f64 {
        self.0 as f64 / 1_000.0
    }
}
impl<'de> Deserialize<'de> for Angle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millidegrees = u32::deserialize(deserializer)?;
        Self::from_millidegrees(millidegrees).map_err(de::Error::custom)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distance_converts_to_integer_millimetres() {
        let distance = Distance::from_metres(327.4).expect("valid distance");
        assert_eq!(distance.as_millimetres(), 327_400);
        assert_eq!(distance.as_metres(), 327.4);
    }
    #[test]
    fn distance_rejects_negative_values() {
        assert_eq!(
            Distance::from_metres(-0.1),
            Err(QuantityError::NegativeDistance)
        );
    }
    #[test]
    fn speed_converts_from_kilometres_per_hour() {
        let speed = Speed::from_kilometres_per_hour(36.0).expect("valid speed");
        assert_eq!(speed.as_mm_per_second(), 10_000);
    }
    #[test]
    fn rotational_speed_converts_to_integer_milli_rpm() {
        let speed = RotationalSpeed::from_rpm(350.25).expect("valid rotational speed");
        assert_eq!(speed.as_milli_rpm(), 350_250);
        assert_eq!(speed.as_rpm(), 350.25);
    }
    #[test]
    fn rotational_speed_rejects_negative_values() {
        assert_eq!(
            RotationalSpeed::from_rpm(-0.1),
            Err(QuantityError::NegativeRotationalSpeed)
        );
    }
    #[test]
    fn angle_normalizes_positive_and_negative_values() {
        let a = Angle::from_degrees(450.0).expect("valid angle");
        let b = Angle::from_degrees(-90.0).expect("valid angle");
        assert_eq!(a.as_degrees(), 90.0);
        assert_eq!(b.as_degrees(), 270.0);
    }
    #[test]
    fn angle_deserialization_rejects_noncanonical_raw_value() {
        assert!(serde_json::from_str::<Angle>("360000").is_err());
        assert!(serde_json::from_str::<Angle>("359999").is_ok());
    }
    #[test]
    fn quantities_round_trip_through_json() {
        let original = Distance::from_metres(100.0).expect("valid distance");
        let json = serde_json::to_string(&original).expect("distance should serialize");
        let restored: Distance = serde_json::from_str(&json).expect("distance should deserialize");
        assert_eq!(original, restored);
    }
}
