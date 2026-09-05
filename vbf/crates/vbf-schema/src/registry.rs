use crate::{ComponentSchema, SchemaDefinitionError};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use vbf_types::Key;

#[derive(Debug, Clone, PartialEq)]
pub enum RegistryError {
    InvalidSchema(SchemaDefinitionError),
    DuplicateSchema { key: Key, version: u32 },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(error) => write!(f, "invalid component schema: {error}"),
            Self::DuplicateSchema { key, version } => {
                write!(f, "component schema already registered: {key} v{version}")
            }
        }
    }
}

impl Error for RegistryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSchema(error) => Some(error),
            Self::DuplicateSchema { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SchemaRegistry {
    components: BTreeMap<Key, BTreeMap<u32, ComponentSchema>>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, schema: ComponentSchema) -> Result<(), RegistryError> {
        schema
            .validate_definition()
            .map_err(RegistryError::InvalidSchema)?;

        let key = schema.key.clone();
        let version = schema.version;
        let versions = self.components.entry(key.clone()).or_default();
        if versions.contains_key(&version) {
            return Err(RegistryError::DuplicateSchema { key, version });
        }
        versions.insert(version, schema);
        Ok(())
    }

    pub fn get(&self, key: &str, version: u32) -> Option<&ComponentSchema> {
        self.components.get(key)?.get(&version)
    }

    pub fn latest(&self, key: &str) -> Option<&ComponentSchema> {
        self.components
            .get(key)?
            .last_key_value()
            .map(|(_, schema)| schema)
    }

    pub fn versions(&self, key: &str) -> impl Iterator<Item = u32> + '_ {
        self.components
            .get(key)
            .into_iter()
            .flat_map(|versions| versions.keys().copied())
    }

    /// Number of registered component schema versions.
    pub fn len(&self) -> usize {
        self.components
            .values()
            .map(|versions| versions.len())
            .sum()
    }

    /// Number of distinct component keys, regardless of version count.
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn owned_by<'a>(&'a self, owner: &'a str) -> impl Iterator<Item = &'a ComponentSchema> {
        self.components
            .values()
            .flat_map(|versions| versions.values())
            .filter(move |schema| schema.owner.as_str() == owner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema(version: u32) -> ComponentSchema {
        ComponentSchema::new(
            Key::new("component.spatial").unwrap(),
            version,
            Key::new("module.layer0").unwrap(),
            vec![],
        )
        .unwrap()
    }

    #[test]
    fn registry_allows_multiple_versions_of_same_component() {
        let mut registry = SchemaRegistry::new();
        registry.register(schema(1)).unwrap();
        registry.register(schema(2)).unwrap();

        assert_eq!(registry.len(), 2);
        assert_eq!(registry.component_count(), 1);
        assert_eq!(registry.latest("component.spatial").unwrap().version, 2);
        assert_eq!(
            registry.versions("component.spatial").collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn registry_rejects_duplicate_key_and_version() {
        let mut registry = SchemaRegistry::new();
        registry.register(schema(1)).unwrap();
        assert!(matches!(
            registry.register(schema(1)),
            Err(RegistryError::DuplicateSchema { version: 1, .. })
        ));
    }

    #[test]
    fn owner_query_returns_registered_schemas() {
        let mut registry = SchemaRegistry::new();
        registry.register(schema(1)).unwrap();
        assert_eq!(registry.owned_by("module.layer0").count(), 1);
        assert_eq!(registry.owned_by("module.combat").count(), 0);
    }
}
