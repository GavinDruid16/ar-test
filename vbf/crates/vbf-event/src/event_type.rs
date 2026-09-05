use serde::{Deserialize, Deserializer, Serialize, de};
use std::error::Error;
use std::fmt;
use vbf_types::Key;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct EventTypeRef {
    pub key: Key,
    pub version: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventTypeRefError {
    ZeroVersion,
}
impl fmt::Display for EventTypeRefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroVersion => write!(f, "event type version must be at least 1"),
        }
    }
}
impl Error for EventTypeRefError {}
impl EventTypeRef {
    pub fn new(key: Key, version: u32) -> Result<Self, EventTypeRefError> {
        if version == 0 {
            Err(EventTypeRefError::ZeroVersion)
        } else {
            Ok(Self { key, version })
        }
    }
}
impl<'de> Deserialize<'de> for EventTypeRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawEventTypeRef {
            key: Key,
            version: u32,
        }
        let raw = RawEventTypeRef::deserialize(deserializer)?;
        Self::new(raw.key, raw.version).map_err(de::Error::custom)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_type_rejects_zero_version() {
        assert!(matches!(
            EventTypeRef::new(Key::new("event.test").unwrap(), 0),
            Err(EventTypeRefError::ZeroVersion)
        ));
    }
}