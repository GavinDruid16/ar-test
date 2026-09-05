use crate::{RelationshipSchema, RelationshipSchemaDefinitionError};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use vbf_types::Key;

#[derive(Clone, Debug, PartialEq)]
pub enum RelationshipRegistryError {
    InvalidSchema(RelationshipSchemaDefinitionError),
    DuplicateSchema { key: Key, version: u32 },
}

impl fmt::Display for RelationshipRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(error) => write!(f, "invalid relationship schema: {error}"),
            Self::DuplicateSchema { key, version } => {
                write!(
                    f,
                    "relationship schema already registered: {key} v{version}"
                )
            }
        }
    }
}

impl Error for RelationshipRegistryError {}

#[derive(Clone, Debug, Default)]
pub struct RelationshipSchemaRegistry {
    schemas: BTreeMap<Key, BTreeMap<u32, RelationshipSchema>>,
}

impl RelationshipSchemaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        schema: RelationshipSchema,
    ) -> Result<(), RelationshipRegistryError> {
        schema
            .validate_definition()
            .map_err(RelationshipRegistryError::InvalidSchema)?;

        let versions = self.schemas.entry(schema.key.clone()).or_default();
        if versions.contains_key(&schema.version) {
            return Err(RelationshipRegistryError::DuplicateSchema {
                key: schema.key,
                version: schema.version,
            });
        }

        versions.insert(schema.version, schema);
        Ok(())
    }

    pub fn get(&self, key: &str, version: u32) -> Option<&RelationshipSchema> {
        self.schemas.get(key)?.get(&version)
    }

    pub fn latest(&self, key: &str) -> Option<&RelationshipSchema> {
        self.schemas
            .get(key)?
            .last_key_value()
            .map(|(_, schema)| schema)
    }

    pub fn versions(&self, key: &str) -> impl Iterator<Item = u32> + '_ {
        self.schemas
            .get(key)
            .into_iter()
            .flat_map(|versions| versions.keys().copied())
    }

    pub fn len(&self) -> usize {
        self.schemas.values().map(|versions| versions.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ParticipantRoleSchema, SlotRequirement};
    use vbf_entity::EntityClass;

    fn schema(version: u32) -> RelationshipSchema {
        RelationshipSchema::new(
            Key::new("relationship.test").unwrap(),
            version,
            Key::new("module.test").unwrap(),
            vec![ParticipantRoleSchema::new(
                Key::new("actor").unwrap(),
                1,
                Some(1),
                vec![EntityClass::Actor],
                SlotRequirement::Forbidden,
            )],
            vec![],
            vec![],
        )
        .unwrap()
    }

    #[test]
    fn registry_allows_multiple_versions() {
        let mut registry = RelationshipSchemaRegistry::new();
        registry.register(schema(1)).unwrap();
        registry.register(schema(2)).unwrap();

        assert_eq!(registry.len(), 2);
        assert_eq!(registry.latest("relationship.test").unwrap().version, 2);
        assert_eq!(
            registry.versions("relationship.test").collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn registry_rejects_duplicate_key_version_pair() {
        let mut registry = RelationshipSchemaRegistry::new();
        registry.register(schema(1)).unwrap();

        assert!(matches!(
            registry.register(schema(1)),
            Err(RelationshipRegistryError::DuplicateSchema { .. })
        ));
    }
}
