use serde::{Deserialize, Deserializer, Serialize, de};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use vbf_entity::EntityClass;
use vbf_schema::{
    ComponentSchema, ComponentValidationReport, FieldSchema, SchemaContext, SchemaDefinitionError,
};
use vbf_types::Key;

/// Stable reference to one exact relationship schema version.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct RelationshipSchemaRef {
    pub key: Key,
    pub version: u32,
}

impl RelationshipSchemaRef {
    pub fn new(key: Key, version: u32) -> Result<Self, RelationshipSchemaDefinitionError> {
        if version == 0 {
            return Err(RelationshipSchemaDefinitionError::ZeroVersion);
        }
        Ok(Self { key, version })
    }
}

impl<'de> Deserialize<'de> for RelationshipSchemaRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawRelationshipSchemaRef {
            key: Key,
            version: u32,
        }

        let raw = RawRelationshipSchemaRef::deserialize(deserializer)?;
        Self::new(raw.key, raw.version).map_err(de::Error::custom)
    }
}

/// Whether a participant role may, must, or must not name an internal slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotRequirement {
    Forbidden,
    Optional,
    Required,
}

/// Structural rules for one participant role in a relationship.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParticipantRoleSchema {
    pub role: Key,
    pub minimum: usize,
    pub maximum: Option<usize>,
    pub allowed_classes: Vec<EntityClass>,
    pub slot_requirement: SlotRequirement,
    #[serde(default)]
    pub allowed_slots: Vec<Key>,
}

impl ParticipantRoleSchema {
    pub fn new(
        role: Key,
        minimum: usize,
        maximum: Option<usize>,
        allowed_classes: Vec<EntityClass>,
        slot_requirement: SlotRequirement,
    ) -> Self {
        Self {
            role,
            minimum,
            maximum,
            allowed_classes,
            slot_requirement,
            allowed_slots: Vec::new(),
        }
    }

    pub fn with_allowed_slots(mut self, allowed_slots: Vec<Key>) -> Self {
        self.allowed_slots = allowed_slots;
        self
    }

    pub fn allows_class(&self, class: EntityClass) -> bool {
        self.allowed_classes.is_empty() || self.allowed_classes.contains(&class)
    }

    pub fn allows_slot(&self, slot: &Key) -> bool {
        self.allowed_slots.is_empty() || self.allowed_slots.contains(slot)
    }
}

/// Cross-record invariants applied to active relationships of one schema.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RelationshipRule {
    /// An Entity may participate in at most one active relationship of this
    /// schema in the named role.
    RoleExclusive { role: Key },

    /// At most `capacity` active occupant relationships may use the same slot
    /// on the same host Entity.
    SlotCapacity {
        host_role: Key,
        occupant_role: Key,
        capacity: usize,
    },
}

/// Versioned structural contract for one relationship type.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelationshipSchema {
    pub key: Key,
    pub version: u32,
    pub owner: Key,
    pub roles: Vec<ParticipantRoleSchema>,
    pub properties: Vec<FieldSchema>,
    pub rules: Vec<RelationshipRule>,
    pub allow_unknown_properties: bool,
    pub allow_same_entity_in_multiple_roles: bool,
}

impl RelationshipSchema {
    pub fn new(
        key: Key,
        version: u32,
        owner: Key,
        roles: Vec<ParticipantRoleSchema>,
        properties: Vec<FieldSchema>,
        rules: Vec<RelationshipRule>,
    ) -> Result<Self, RelationshipSchemaDefinitionError> {
        let schema = Self {
            key,
            version,
            owner,
            roles,
            properties,
            rules,
            allow_unknown_properties: false,
            allow_same_entity_in_multiple_roles: false,
        };
        schema.validate_definition()?;
        Ok(schema)
    }

    pub fn schema_ref(&self) -> RelationshipSchemaRef {
        RelationshipSchemaRef {
            key: self.key.clone(),
            version: self.version,
        }
    }

    pub fn allow_unknown_properties(mut self, allow: bool) -> Self {
        self.allow_unknown_properties = allow;
        self
    }

    pub fn allow_same_entity_in_multiple_roles(mut self, allow: bool) -> Self {
        self.allow_same_entity_in_multiple_roles = allow;
        self
    }

    pub fn role(&self, role: &str) -> Option<&ParticipantRoleSchema> {
        self.roles
            .iter()
            .find(|candidate| candidate.role.as_str() == role)
    }

    pub fn validate_definition(&self) -> Result<(), RelationshipSchemaDefinitionError> {
        if self.version == 0 {
            return Err(RelationshipSchemaDefinitionError::ZeroVersion);
        }
        if self.roles.is_empty() {
            return Err(RelationshipSchemaDefinitionError::NoRoles);
        }

        let mut role_keys = BTreeSet::new();
        for role in &self.roles {
            if !role_keys.insert(role.role.clone()) {
                return Err(RelationshipSchemaDefinitionError::DuplicateRole(
                    role.role.clone(),
                ));
            }

            if role.maximum.is_some_and(|maximum| role.minimum > maximum) {
                return Err(RelationshipSchemaDefinitionError::InvalidRoleCardinality {
                    role: role.role.clone(),
                    minimum: role.minimum,
                    maximum: role.maximum,
                });
            }

            if role.slot_requirement == SlotRequirement::Forbidden && !role.allowed_slots.is_empty()
            {
                return Err(
                    RelationshipSchemaDefinitionError::SlotsForbiddenButDeclared(role.role.clone()),
                );
            }

            let mut slots = BTreeSet::new();
            for slot in &role.allowed_slots {
                if !slots.insert(slot.clone()) {
                    return Err(RelationshipSchemaDefinitionError::DuplicateAllowedSlot {
                        role: role.role.clone(),
                        slot: slot.clone(),
                    });
                }
            }

            let mut classes = Vec::new();
            for class in &role.allowed_classes {
                if classes.contains(class) {
                    return Err(RelationshipSchemaDefinitionError::DuplicateAllowedClass {
                        role: role.role.clone(),
                        class: *class,
                    });
                }
                classes.push(*class);
            }
        }

        ComponentSchema::new(
            self.key.clone(),
            self.version,
            self.owner.clone(),
            self.properties.clone(),
        )
        .map_err(RelationshipSchemaDefinitionError::InvalidPropertySchema)?;

        for rule in &self.rules {
            match rule {
                RelationshipRule::RoleExclusive { role } => {
                    if self.role(role.as_str()).is_none() {
                        return Err(RelationshipSchemaDefinitionError::UnknownRuleRole(
                            role.clone(),
                        ));
                    }
                }
                RelationshipRule::SlotCapacity {
                    host_role,
                    occupant_role,
                    capacity,
                } => {
                    if *capacity == 0 {
                        return Err(RelationshipSchemaDefinitionError::ZeroSlotCapacity);
                    }

                    if host_role == occupant_role {
                        return Err(RelationshipSchemaDefinitionError::SlotCapacitySameRole(
                            host_role.clone(),
                        ));
                    }

                    let Some(host) = self.role(host_role.as_str()) else {
                        return Err(RelationshipSchemaDefinitionError::UnknownRuleRole(
                            host_role.clone(),
                        ));
                    };

                    if host.minimum != 1 || host.maximum != Some(1) {
                        return Err(
                            RelationshipSchemaDefinitionError::SlotCapacityRequiresSingleHost(
                                host_role.clone(),
                            ),
                        );
                    }

                    let Some(occupant) = self.role(occupant_role.as_str()) else {
                        return Err(RelationshipSchemaDefinitionError::UnknownRuleRole(
                            occupant_role.clone(),
                        ));
                    };

                    if occupant.slot_requirement == SlotRequirement::Forbidden {
                        return Err(
                            RelationshipSchemaDefinitionError::SlotCapacityRequiresSlottedRole(
                                occupant_role.clone(),
                            ),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub fn validate_properties(
        &self,
        properties: &serde_json::Value,
        context: SchemaContext,
    ) -> ComponentValidationReport {
        let schema = ComponentSchema::new(
            self.key.clone(),
            self.version,
            self.owner.clone(),
            self.properties.clone(),
        )
        .expect("registered relationship schema has valid property fields")
        .allow_unknown_fields(self.allow_unknown_properties);

        schema.validate_payload(properties, context)
    }
}

impl<'de> Deserialize<'de> for RelationshipSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawRelationshipSchema {
            key: Key,
            version: u32,
            owner: Key,
            roles: Vec<ParticipantRoleSchema>,
            #[serde(default)]
            properties: Vec<FieldSchema>,
            #[serde(default)]
            rules: Vec<RelationshipRule>,
            #[serde(default)]
            allow_unknown_properties: bool,
            #[serde(default)]
            allow_same_entity_in_multiple_roles: bool,
        }

        let raw = RawRelationshipSchema::deserialize(deserializer)?;
        let schema = Self::new(
            raw.key,
            raw.version,
            raw.owner,
            raw.roles,
            raw.properties,
            raw.rules,
        )
        .map_err(de::Error::custom)?;

        Ok(schema
            .allow_unknown_properties(raw.allow_unknown_properties)
            .allow_same_entity_in_multiple_roles(raw.allow_same_entity_in_multiple_roles))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelationshipSchemaDefinitionError {
    ZeroVersion,
    NoRoles,
    DuplicateRole(Key),
    InvalidRoleCardinality {
        role: Key,
        minimum: usize,
        maximum: Option<usize>,
    },
    DuplicateAllowedSlot {
        role: Key,
        slot: Key,
    },
    DuplicateAllowedClass {
        role: Key,
        class: EntityClass,
    },
    SlotsForbiddenButDeclared(Key),
    InvalidPropertySchema(SchemaDefinitionError),
    UnknownRuleRole(Key),
    ZeroSlotCapacity,
    SlotCapacitySameRole(Key),
    SlotCapacityRequiresSingleHost(Key),
    SlotCapacityRequiresSlottedRole(Key),
}

impl fmt::Display for RelationshipSchemaDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroVersion => write!(f, "relationship schema version must be at least 1"),
            Self::NoRoles => write!(f, "relationship schema must declare at least one role"),
            Self::DuplicateRole(role) => write!(f, "duplicate relationship role: {role}"),
            Self::InvalidRoleCardinality {
                role,
                minimum,
                maximum,
            } => write!(
                f,
                "invalid cardinality for role {role}: minimum {minimum}, maximum {maximum:?}"
            ),
            Self::DuplicateAllowedSlot { role, slot } => {
                write!(f, "role {role} declares duplicate allowed slot {slot}")
            }
            Self::DuplicateAllowedClass { role, class } => {
                write!(f, "role {role} declares duplicate allowed class {class:?}")
            }
            Self::SlotsForbiddenButDeclared(role) => {
                write!(f, "role {role} forbids slots but declares allowed slots")
            }
            Self::InvalidPropertySchema(error) => {
                write!(f, "relationship property schema is invalid: {error}")
            }
            Self::UnknownRuleRole(role) => {
                write!(f, "relationship rule references unknown role {role}")
            }
            Self::ZeroSlotCapacity => write!(f, "slot capacity must be at least 1"),
            Self::SlotCapacitySameRole(role) => write!(
                f,
                "slot-capacity host and occupant roles must differ; both were {role}"
            ),
            Self::SlotCapacityRequiresSingleHost(role) => write!(
                f,
                "slot-capacity host role {role} must require exactly one participant"
            ),
            Self::SlotCapacityRequiresSlottedRole(role) => write!(
                f,
                "slot-capacity rule requires occupant role {role} to permit slots"
            ),
        }
    }
}

impl Error for RelationshipSchemaDefinitionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use vbf_schema::{FieldRequirement, FieldType, PersistenceClass};

    fn actor_role(name: &str) -> ParticipantRoleSchema {
        ParticipantRoleSchema::new(
            Key::new(name).unwrap(),
            1,
            Some(1),
            vec![EntityClass::Actor],
            SlotRequirement::Forbidden,
        )
    }

    #[test]
    fn schema_rejects_zero_version() {
        let result = RelationshipSchema::new(
            Key::new("relationship.test").unwrap(),
            0,
            Key::new("module.test").unwrap(),
            vec![actor_role("actor")],
            vec![],
            vec![],
        );
        assert!(matches!(
            result,
            Err(RelationshipSchemaDefinitionError::ZeroVersion)
        ));
    }

    #[test]
    fn schema_rejects_duplicate_roles() {
        let result = RelationshipSchema::new(
            Key::new("relationship.test").unwrap(),
            1,
            Key::new("module.test").unwrap(),
            vec![actor_role("actor"), actor_role("actor")],
            vec![],
            vec![],
        );
        assert!(matches!(
            result,
            Err(RelationshipSchemaDefinitionError::DuplicateRole(_))
        ));
    }

    #[test]
    fn schema_rejects_reversed_role_cardinality() {
        let role = ParticipantRoleSchema::new(
            Key::new("crew").unwrap(),
            2,
            Some(1),
            vec![EntityClass::Actor],
            SlotRequirement::Optional,
        );
        let result = RelationshipSchema::new(
            Key::new("relationship.test").unwrap(),
            1,
            Key::new("module.test").unwrap(),
            vec![role],
            vec![],
            vec![],
        );
        assert!(matches!(
            result,
            Err(RelationshipSchemaDefinitionError::InvalidRoleCardinality { .. })
        ));
    }

    #[test]
    fn relationship_properties_reuse_vbf_schema_validation() {
        let property = FieldSchema::new(
            Key::new("scope").unwrap(),
            FieldType::Key,
            FieldRequirement::Required,
            PersistenceClass::Mutable,
        );
        let schema = RelationshipSchema::new(
            Key::new("relationship.test").unwrap(),
            1,
            Key::new("module.test").unwrap(),
            vec![actor_role("actor")],
            vec![property],
            vec![],
        )
        .unwrap();

        assert!(
            schema
                .validate_properties(
                    &serde_json::json!({"scope": "scope.command"}),
                    SchemaContext::RuntimeState,
                )
                .is_valid()
        );

        assert!(
            !schema
                .validate_properties(
                    &serde_json::json!({"scope": "BAD KEY"}),
                    SchemaContext::RuntimeState,
                )
                .is_valid()
        );
    }
}
