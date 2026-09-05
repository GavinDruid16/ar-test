use crate::Relationship;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use vbf_types::{EntityUid, Key, RelationshipUid, SimTime};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelationshipCatalogError {
    DuplicateUid(RelationshipUid),
    DuplicateKey(Key),
}

impl fmt::Display for RelationshipCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateUid(uid) => write!(f, "relationship UID already registered: {uid}"),
            Self::DuplicateKey(key) => {
                write!(f, "relationship key already registered: {key}")
            }
        }
    }
}

impl Error for RelationshipCatalogError {}

#[derive(Clone, Debug, Default)]
pub struct RelationshipCatalog {
    by_uid: BTreeMap<RelationshipUid, Relationship>,
    by_key: BTreeMap<Key, RelationshipUid>,
}

impl RelationshipCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, relationship: Relationship) -> Result<(), RelationshipCatalogError> {
        if self.by_uid.contains_key(&relationship.uid) {
            return Err(RelationshipCatalogError::DuplicateUid(relationship.uid));
        }
        if self.by_key.contains_key(&relationship.key) {
            return Err(RelationshipCatalogError::DuplicateKey(
                relationship.key.clone(),
            ));
        }

        self.by_key
            .insert(relationship.key.clone(), relationship.uid);
        self.by_uid.insert(relationship.uid, relationship);
        Ok(())
    }

    pub fn get(&self, uid: RelationshipUid) -> Option<&Relationship> {
        self.by_uid.get(&uid)
    }

    pub fn get_by_key(&self, key: &str) -> Option<&Relationship> {
        let uid = self.by_key.get(key)?;
        self.by_uid.get(uid)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Relationship> {
        self.by_uid.values()
    }

    pub fn active_at(&self, time: SimTime) -> impl Iterator<Item = &Relationship> {
        self.by_uid
            .values()
            .filter(move |relationship| relationship.active_at(time))
    }

    pub fn involving(&self, entity: EntityUid) -> impl Iterator<Item = &Relationship> {
        self.by_uid
            .values()
            .filter(move |relationship| relationship.involves(entity))
    }

    pub fn len(&self) -> usize {
        self.by_uid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_uid.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RelationshipSchemaRef;

    fn relationship(key: &str) -> Relationship {
        Relationship::new(
            RelationshipUid::new(),
            Key::new(key).unwrap(),
            RelationshipSchemaRef::new(Key::new("relationship.test").unwrap(), 1).unwrap(),
            vec![],
            serde_json::json!({}),
        )
    }

    #[test]
    fn catalog_rejects_duplicate_keys() {
        let mut catalog = RelationshipCatalog::new();
        catalog
            .register(relationship("relationship.instance.one"))
            .unwrap();

        assert!(matches!(
            catalog.register(relationship("relationship.instance.one")),
            Err(RelationshipCatalogError::DuplicateKey(_))
        ));
    }
}
