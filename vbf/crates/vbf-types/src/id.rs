//! Stable identity and human-readable naming.
//!
//! VBF deliberately separates:
//!
//! - UID: permanent machine identity;
//! - Key: stable human-readable authored reference;
//! - DisplayName: presentation text.
//!
//! All five Core token classes use `EntityUid`; Actor/Asset/etc. are entity
//! classes, not separate identity universes.
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
#[derive(Debug)]
pub enum IdParseError {
    InvalidUuid(uuid::Error),
    NilUuid,
}
impl fmt::Display for IdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUuid(err) => write!(f, "invalid UUID: {err}"),
            Self::NilUuid => write!(f, "nil UUID is not permitted as a VBF identifier"),
        }
    }
}
impl Error for IdParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidUuid(err) => Some(err),
            Self::NilUuid => None,
        }
    }
}
fn validate_uuid(uuid: Uuid) -> Result<Uuid, IdParseError> {
    if uuid.is_nil() {
        Err(IdParseError::NilUuid)
    } else {
        Ok(uuid)
    }
}
macro_rules! define_uid_type {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(Uuid);
        impl $name {
            /// Generate a fresh UUIDv7-backed identity.
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }
            /// Construct from an externally supplied UUID after VBF validation.
            pub fn from_uuid(uuid: Uuid) -> Result<Self, IdParseError> {
                validate_uuid(uuid).map(Self)
            }
            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
            pub fn into_uuid(self) -> Uuid {
                self.0
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl FromStr for $name {
            type Err = IdParseError;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let uuid = Uuid::parse_str(value).map_err(IdParseError::InvalidUuid)?;
                Self::from_uuid(uuid)
            }
        }
        impl TryFrom<Uuid> for $name {
            type Error = IdParseError;
            fn try_from(value: Uuid) -> Result<Self, Self::Error> {
                Self::from_uuid(value)
            }
        }
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_str(self)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let raw = String::deserialize(deserializer)?;
                raw.parse::<Self>().map_err(de::Error::custom)
            }
        }
    };
}
define_uid_type!(
    EntityUid,
    "Permanent identity of one instantiated VBF entity."
);
define_uid_type!(
    DefinitionUid,
    "Permanent identity of one reusable definition or template."
);
define_uid_type!(
    RelationshipUid,
    "Permanent identity of one relationship record."
);
define_uid_type!(EventUid, "Permanent identity of one event record.");
define_uid_type!(
    CorrelationUid,
    "Permanent identity of one cross-event occurrence/correlation group."
);
define_uid_type!(PackageUid, "Permanent identity of one VBF package.");
define_uid_type!(
    SourceUid,
    "Permanent identity of one registered source/provenance record."
);
define_uid_type!(
    SnapshotUid,
    "Permanent identity of one saved state snapshot."
);
define_uid_type!(BranchUid, "Permanent identity of one event-history branch.");
const MAX_KEY_BYTES: usize = 255;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyError {
    Empty,
    TooLong {
        actual: usize,
        maximum: usize,
    },
    EmptySegment {
        segment: usize,
    },
    InvalidSegmentStart {
        segment: usize,
        character: char,
    },
    InvalidCharacter {
        segment: usize,
        character_index: usize,
        character: char,
    },
}
impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "key cannot be empty"),
            Self::TooLong { actual, maximum } => {
                write!(f, "key is {actual} bytes long; maximum is {maximum}")
            }
            Self::EmptySegment { segment } => {
                write!(f, "key segment {segment} is empty")
            }
            Self::InvalidSegmentStart { segment, character } => write!(
                f,
                "key segment {segment} begins with invalid character '{character}'"
            ),
            Self::InvalidCharacter {
                segment,
                character_index,
                character,
            } => write!(
                f,
                "key segment {segment} contains invalid character '{character}' at position {character_index}"
            ),
        }
    }
}
impl Error for KeyError {}
impl Key {
    /// Create a canonical dot-separated VBF key.
    ///
    /// Each segment begins with lowercase ASCII or a digit. Remaining segment
    /// characters may also include `_` and `-`.
    pub fn new(value: impl Into<String>) -> Result<Self, KeyError> {
        let value = value.into();
        if value.is_empty() {
            return Err(KeyError::Empty);
        }
        if value.len() > MAX_KEY_BYTES {
            return Err(KeyError::TooLong {
                actual: value.len(),
                maximum: MAX_KEY_BYTES,
            });
        }
        for (segment_index, segment) in value.split('.').enumerate() {
            if segment.is_empty() {
                return Err(KeyError::EmptySegment {
                    segment: segment_index,
                });
            }
            let mut chars = segment.chars();
            let first = chars
                .next()
                .expect("non-empty segment has a first character");
            if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
                return Err(KeyError::InvalidSegmentStart {
                    segment: segment_index,
                    character: first,
                });
            }
            for (offset, character) in chars.enumerate() {
                let valid = character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || character == '_'
                    || character == '-';
                if !valid {
                    return Err(KeyError::InvalidCharacter {
                        segment: segment_index,
                        character_index: offset + 1,
                        character,
                    });
                }
            }
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn into_string(self) -> String {
        self.0
    }
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.0.split('.')
    }
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl FromStr for Key {
    type Err = KeyError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}
impl TryFrom<String> for Key {
    type Error = KeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl TryFrom<&str> for Key {
    type Error = KeyError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl Borrow<str> for Key {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Key::new(raw).map_err(de::Error::custom)
    }
}
const MAX_DISPLAY_NAME_CHARS: usize = 200;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayName(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayNameError {
    Empty,
    TooLong { actual: usize, maximum: usize },
    LeadingOrTrailingWhitespace,
    ControlCharacter { index: usize, character: char },
}
impl fmt::Display for DisplayNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "display name cannot be empty"),
            Self::TooLong { actual, maximum } => write!(
                f,
                "display name is {actual} characters long; maximum is {maximum}"
            ),
            Self::LeadingOrTrailingWhitespace => {
                write!(f, "display name cannot begin or end with whitespace")
            }
            Self::ControlCharacter { index, character } => write!(
                f,
                "display name contains control character {character:?} at position {index}"
            ),
        }
    }
}
impl Error for DisplayNameError {}
impl DisplayName {
    pub fn new(value: impl Into<String>) -> Result<Self, DisplayNameError> {
        let value = value.into();
        if value.is_empty() || value.trim().is_empty() {
            return Err(DisplayNameError::Empty);
        }
        if value.trim() != value {
            return Err(DisplayNameError::LeadingOrTrailingWhitespace);
        }
        let character_count = value.chars().count();
        if character_count > MAX_DISPLAY_NAME_CHARS {
            return Err(DisplayNameError::TooLong {
                actual: character_count,
                maximum: MAX_DISPLAY_NAME_CHARS,
            });
        }
        for (index, character) in value.chars().enumerate() {
            if character.is_control() {
                return Err(DisplayNameError::ControlCharacter { index, character });
            }
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn into_string(self) -> String {
        self.0
    }
}
impl fmt::Display for DisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl FromStr for DisplayName {
    type Err = DisplayNameError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}
impl TryFrom<String> for DisplayName {
    type Error = DisplayNameError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl TryFrom<&str> for DisplayName {
    type Error = DisplayNameError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl AsRef<str> for DisplayName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl Serialize for DisplayName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for DisplayName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        DisplayName::new(raw).map_err(de::Error::custom)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn generated_entity_uid_is_non_nil_v7() {
        let uid = EntityUid::new();
        assert!(!uid.as_uuid().is_nil());
        assert_eq!(uid.as_uuid().get_version_num(), 7);
    }
    #[test]
    fn generated_entity_uids_are_distinct_and_ordered() {
        let first = EntityUid::new();
        let second = EntityUid::new();
        assert_ne!(first, second);
        assert!(first < second);
    }
    #[test]
    fn uid_round_trips_through_text() {
        let original = EntityUid::new();
        let restored: EntityUid = original
            .to_string()
            .parse()
            .expect("generated UID should parse");
        assert_eq!(original, restored);
    }
    #[test]
    fn uid_round_trips_through_json() {
        let original = EntityUid::new();
        let json = serde_json::to_string(&original).expect("UID should serialize");
        let restored: EntityUid = serde_json::from_str(&json).expect("UID should deserialize");
        assert_eq!(original, restored);
    }
    #[test]
    fn nil_uid_is_rejected_from_uuid_and_json() {
        assert!(matches!(
            EntityUid::try_from(Uuid::nil()),
            Err(IdParseError::NilUuid)
        ));
        let json = "\"00000000-0000-0000-0000-000000000000\"";
        assert!(serde_json::from_str::<EntityUid>(json).is_err());
    }
    #[test]
    fn valid_vbf_key_is_accepted() {
        let key = Key::new("entity.us.42crs.baker.m8_b12").expect("key should be valid");
        assert_eq!(key.as_str(), "entity.us.42crs.baker.m8_b12");
    }
    #[test]
    fn malformed_keys_are_rejected() {
        for candidate in [
            "",
            ".entity",
            "entity.",
            "entity..vehicle",
            "Entity.us",
            "entity.US",
            "_entity",
            "-entity",
            "entity us",
            "entity/us",
            "entity:us",
            "véhicule.us",
        ] {
            assert!(Key::new(candidate).is_err(), "{candidate:?} should fail");
        }
    }
    #[test]
    fn key_supports_borrowed_hashmap_lookup() {
        let key = Key::new("entity.us.m8_b12").expect("valid key");
        let mut map = HashMap::new();
        map.insert(key, 42);
        assert_eq!(map.get("entity.us.m8_b12"), Some(&42));
    }
    #[test]
    fn key_round_trips_through_json() {
        let original = Key::new("vehicle.us.m8-armored-car").expect("valid key");
        let json = serde_json::to_string(&original).expect("key should serialize");
        let restored: Key = serde_json::from_str(&json).expect("key should deserialize");
        assert_eq!(original, restored);
    }
    #[test]
    fn display_name_accepts_unicode_and_internal_spaces() {
        let name = DisplayName::new("HMS Småland (J19)").expect("valid display name");
        assert_eq!(name.as_str(), "HMS Småland (J19)");
    }
    #[test]
    fn malformed_display_names_are_rejected() {
        assert!(matches!(
            DisplayName::new("   "),
            Err(DisplayNameError::Empty)
        ));
        assert!(matches!(
            DisplayName::new(" M8 B-12 "),
            Err(DisplayNameError::LeadingOrTrailingWhitespace)
        ));
        assert!(DisplayName::new("M8\nB-12").is_err());
        assert!(DisplayName::new("M8\tB-12").is_err());
    }
    #[test]
    fn display_name_round_trips_through_json() {
        let original = DisplayName::new("Baker Troop M8 B-12").expect("valid name");
        let json = serde_json::to_string(&original).expect("name should serialize");
        let restored: DisplayName = serde_json::from_str(&json).expect("name should deserialize");
        assert_eq!(original, restored);
    }
}
