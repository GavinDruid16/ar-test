use crate::{ComponentData, Definition, Entity, EntityClass};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use vbf_types::{DefinitionUid, EntityUid, Key};

#[derive(Clone, Debug, Default)]
pub struct DefinitionCatalog {
    by_uid: BTreeMap<DefinitionUid, Definition>,
    by_key: BTreeMap<Key, DefinitionUid>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefinitionCatalogError {
    DuplicateUid(DefinitionUid),
    DuplicateKey(Key),
}

impl fmt::Display for DefinitionCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateUid(uid) => write!(f, "definition UID already registered: {uid}"),
            Self::DuplicateKey(key) => write!(f, "definition key already registered: {key}"),
        }
    }
}

impl Error for DefinitionCatalogError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefinitionResolutionError {
    MissingDefinition(DefinitionUid),
    MissingParent {
        definition: DefinitionUid,
        parent: DefinitionUid,
    },
    InheritanceCycle(DefinitionUid),
    ParentClassMismatch {
        definition: DefinitionUid,
        definition_class: EntityClass,
        parent: DefinitionUid,
        parent_class: EntityClass,
    },
}

impl fmt::Display for DefinitionResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDefinition(uid) => write!(f, "definition does not exist: {uid}"),
            Self::MissingParent { definition, parent } => {
                write!(f, "definition {definition} extends missing parent {parent}")
            }
            Self::InheritanceCycle(uid) => {
                write!(f, "definition inheritance cycle contains {uid}")
            }
            Self::ParentClassMismatch {
                definition,
                definition_class,
                parent,
                parent_class,
            } => write!(
                f,
                "definition {definition} ({definition_class:?}) cannot extend {parent} ({parent_class:?})"
            ),
        }
    }
}

impl Error for DefinitionResolutionError {}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedDefinition {
    pub uid: DefinitionUid,
    pub key: Key,
    pub class: EntityClass,
    pub lineage: Vec<DefinitionUid>,
    pub components: BTreeMap<Key, ComponentData>,
}

impl DefinitionCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, definition: Definition) -> Result<(), DefinitionCatalogError> {
        if self.by_uid.contains_key(&definition.uid) {
            return Err(DefinitionCatalogError::DuplicateUid(definition.uid));
        }
        if self.by_key.contains_key(&definition.key) {
            return Err(DefinitionCatalogError::DuplicateKey(definition.key));
        }

        self.by_key.insert(definition.key.clone(), definition.uid);
        self.by_uid.insert(definition.uid, definition);
        Ok(())
    }

    pub fn get(&self, uid: DefinitionUid) -> Option<&Definition> {
        self.by_uid.get(&uid)
    }

    pub fn get_by_key(&self, key: &str) -> Option<&Definition> {
        let uid = self.by_key.get(key)?;
        self.by_uid.get(uid)
    }

    pub fn len(&self) -> usize {
        self.by_uid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_uid.is_empty()
    }

    pub fn resolve(
        &self,
        uid: DefinitionUid,
    ) -> Result<ResolvedDefinition, DefinitionResolutionError> {
        let mut visiting = BTreeSet::new();
        self.resolve_inner(uid, &mut visiting)
    }

    fn resolve_inner(
        &self,
        uid: DefinitionUid,
        visiting: &mut BTreeSet<DefinitionUid>,
    ) -> Result<ResolvedDefinition, DefinitionResolutionError> {
        if !visiting.insert(uid) {
            return Err(DefinitionResolutionError::InheritanceCycle(uid));
        }

        let definition = self
            .by_uid
            .get(&uid)
            .ok_or(DefinitionResolutionError::MissingDefinition(uid))?;

        let mut lineage = Vec::new();
        let mut components = BTreeMap::new();

        if let Some(parent_uid) = definition.extends {
            let parent =
                self.by_uid
                    .get(&parent_uid)
                    .ok_or(DefinitionResolutionError::MissingParent {
                        definition: uid,
                        parent: parent_uid,
                    })?;

            if parent.class != definition.class {
                return Err(DefinitionResolutionError::ParentClassMismatch {
                    definition: uid,
                    definition_class: definition.class,
                    parent: parent_uid,
                    parent_class: parent.class,
                });
            }

            let resolved_parent = self.resolve_inner(parent_uid, visiting)?;
            lineage.extend(resolved_parent.lineage);
            components.extend(resolved_parent.components);
        }

        lineage.push(uid);
        components.extend(definition.components.clone());
        visiting.remove(&uid);

        Ok(ResolvedDefinition {
            uid,
            key: definition.key.clone(),
            class: definition.class,
            lineage,
            components,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct EntityCatalog {
    by_uid: BTreeMap<EntityUid, Entity>,
    by_key: BTreeMap<Key, EntityUid>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityCatalogError {
    DuplicateUid(EntityUid),
    DuplicateKey(Key),
}

impl fmt::Display for EntityCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateUid(uid) => write!(f, "entity UID already registered: {uid}"),
            Self::DuplicateKey(key) => write!(f, "entity key already registered: {key}"),
        }
    }
}

impl Error for EntityCatalogError {}

impl EntityCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entity: Entity) -> Result<(), EntityCatalogError> {
        if self.by_uid.contains_key(&entity.uid) {
            return Err(EntityCatalogError::DuplicateUid(entity.uid));
        }
        if self.by_key.contains_key(&entity.key) {
            return Err(EntityCatalogError::DuplicateKey(entity.key));
        }

        self.by_key.insert(entity.key.clone(), entity.uid);
        self.by_uid.insert(entity.uid, entity);
        Ok(())
    }

    pub fn get(&self, uid: EntityUid) -> Option<&Entity> {
        self.by_uid.get(&uid)
    }

    pub fn get_by_key(&self, key: &str) -> Option<&Entity> {
        let uid = self.by_key.get(key)?;
        self.by_uid.get(uid)
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
    use crate::Definition;
    use vbf_types::{DisplayName, Key};

    fn definition(key: &str, class: EntityClass) -> Definition {
        Definition::new(
            DefinitionUid::new(),
            Key::new(key).unwrap(),
            DisplayName::new(key).unwrap(),
            class,
        )
    }

    #[test]
    fn definition_catalog_rejects_duplicate_keys() {
        let mut catalog = DefinitionCatalog::new();
        catalog
            .register(definition("definition.vehicle.base", EntityClass::Asset))
            .unwrap();
        assert!(matches!(
            catalog.register(definition("definition.vehicle.base", EntityClass::Asset)),
            Err(DefinitionCatalogError::DuplicateKey(_))
        ));
    }

    #[test]
    fn definition_resolution_preserves_lineage() {
        let parent = definition("definition.vehicle.base", EntityClass::Asset);
        let parent_uid = parent.uid;
        let child_uid = DefinitionUid::new();
        let child = Definition::new(
            child_uid,
            Key::new("definition.vehicle.m8").unwrap(),
            DisplayName::new("M8 Light Armored Car").unwrap(),
            EntityClass::Asset,
        )
        .with_parent(parent_uid);

        let mut catalog = DefinitionCatalog::new();
        catalog.register(parent).unwrap();
        catalog.register(child).unwrap();

        let resolved = catalog.resolve(child_uid).unwrap();
        assert_eq!(resolved.lineage, vec![parent_uid, child_uid]);
        assert_eq!(resolved.class, EntityClass::Asset);
    }

    #[test]
    fn definition_resolution_rejects_cross_class_inheritance() {
        let parent = definition("definition.actor.base", EntityClass::Actor);
        let parent_uid = parent.uid;
        let child = definition("definition.asset.bad", EntityClass::Asset).with_parent(parent_uid);
        let child_uid = child.uid;

        let mut catalog = DefinitionCatalog::new();
        catalog.register(parent).unwrap();
        catalog.register(child).unwrap();

        assert!(matches!(
            catalog.resolve(child_uid),
            Err(DefinitionResolutionError::ParentClassMismatch { .. })
        ));
    }

    #[test]
    fn definition_resolution_detects_cycles() {
        let a_uid = DefinitionUid::new();
        let b_uid = DefinitionUid::new();
        let a = Definition::new(
            a_uid,
            Key::new("definition.test.a").unwrap(),
            DisplayName::new("A").unwrap(),
            EntityClass::Asset,
        )
        .with_parent(b_uid);
        let b = Definition::new(
            b_uid,
            Key::new("definition.test.b").unwrap(),
            DisplayName::new("B").unwrap(),
            EntityClass::Asset,
        )
        .with_parent(a_uid);

        let mut catalog = DefinitionCatalog::new();
        catalog.register(a).unwrap();
        catalog.register(b).unwrap();

        assert!(matches!(
            catalog.resolve(a_uid),
            Err(DefinitionResolutionError::InheritanceCycle(_))
        ));
    }
}
