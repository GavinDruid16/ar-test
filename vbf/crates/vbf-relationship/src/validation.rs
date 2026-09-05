use crate::{
    Relationship, RelationshipCatalog, RelationshipRule, RelationshipSchemaRef,
    RelationshipSchemaRegistry, SlotRequirement,
};
use std::collections::{BTreeMap, BTreeSet};
use vbf_entity::{EntityCatalog, EntityClass};
use vbf_schema::{ComponentValidationReport, SchemaContext};
use vbf_types::{EntityUid, Key, RelationshipUid, SimTime};

type ExclusiveIndexKey = (RelationshipSchemaRef, Key, EntityUid);
type SlotCapacityIndexKey = (RelationshipSchemaRef, Key, Key, EntityUid, Key, usize);

#[derive(Clone, Debug, PartialEq)]
pub enum RelationshipValidationIssueKind {
    UnknownSchema(RelationshipSchemaRef),
    InvalidTimeRange {
        valid_from: SimTime,
        valid_to: SimTime,
    },
    UnknownRole(Key),
    MissingRequiredRole {
        role: Key,
        minimum: usize,
        actual: usize,
    },
    TooManyParticipantsInRole {
        role: Key,
        maximum: usize,
        actual: usize,
    },
    MissingEntity(EntityUid),
    ParticipantClassMismatch {
        role: Key,
        entity: EntityUid,
        actual: EntityClass,
        allowed: Vec<EntityClass>,
    },
    DuplicateParticipant {
        entity: EntityUid,
        role: Key,
        slot: Option<Key>,
    },
    SameEntityInMultipleRoles(EntityUid),
    SlotRequired {
        entity: EntityUid,
        role: Key,
    },
    SlotForbidden {
        entity: EntityUid,
        role: Key,
        slot: Key,
    },
    SlotNotAllowed {
        entity: EntityUid,
        role: Key,
        slot: Key,
        allowed: Vec<Key>,
    },
    InvalidProperties(ComponentValidationReport),
    RoleExclusiveConflict {
        schema: RelationshipSchemaRef,
        role: Key,
        entity: EntityUid,
        relationships: Vec<RelationshipUid>,
    },
    SlotCapacityExceeded {
        schema: RelationshipSchemaRef,
        host: EntityUid,
        slot: Key,
        capacity: usize,
        actual: usize,
        relationships: Vec<RelationshipUid>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationshipValidationIssue {
    pub relationship: Option<RelationshipUid>,
    pub kind: RelationshipValidationIssueKind,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RelationshipValidationReport {
    pub issues: Vec<RelationshipValidationIssue>,
}

impl RelationshipValidationReport {
    pub fn push(
        &mut self,
        relationship: Option<RelationshipUid>,
        kind: RelationshipValidationIssueKind,
    ) {
        self.issues
            .push(RelationshipValidationIssue { relationship, kind });
    }

    pub fn extend(&mut self, other: RelationshipValidationReport) {
        self.issues.extend(other.issues);
    }

    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }
}

pub fn validate_relationship(
    relationship: &Relationship,
    schemas: &RelationshipSchemaRegistry,
    entities: &EntityCatalog,
    context: SchemaContext,
) -> RelationshipValidationReport {
    let mut report = RelationshipValidationReport::default();

    if let (Some(valid_from), Some(valid_to)) = (relationship.valid_from, relationship.valid_to)
        && valid_from >= valid_to
    {
        report.push(
            Some(relationship.uid),
            RelationshipValidationIssueKind::InvalidTimeRange {
                valid_from,
                valid_to,
            },
        );
    }

    let Some(schema) = schemas.get(
        relationship.schema.key.as_str(),
        relationship.schema.version,
    ) else {
        report.push(
            Some(relationship.uid),
            RelationshipValidationIssueKind::UnknownSchema(relationship.schema.clone()),
        );
        return report;
    };

    let properties = schema.validate_properties(&relationship.properties, context);
    if !properties.is_valid() {
        report.push(
            Some(relationship.uid),
            RelationshipValidationIssueKind::InvalidProperties(properties),
        );
    }

    let mut counts: BTreeMap<Key, usize> = BTreeMap::new();
    let mut seen_participants = BTreeSet::new();
    let mut seen_entities: BTreeMap<EntityUid, Key> = BTreeMap::new();

    for participant in &relationship.participants {
        let identity = (
            participant.entity,
            participant.role.clone(),
            participant.slot.clone(),
        );
        if !seen_participants.insert(identity) {
            report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::DuplicateParticipant {
                    entity: participant.entity,
                    role: participant.role.clone(),
                    slot: participant.slot.clone(),
                },
            );
        }

        if !schema.allow_same_entity_in_multiple_roles {
            if let Some(previous_role) = seen_entities.get(&participant.entity) {
                if previous_role != &participant.role {
                    report.push(
                        Some(relationship.uid),
                        RelationshipValidationIssueKind::SameEntityInMultipleRoles(
                            participant.entity,
                        ),
                    );
                }
            } else {
                seen_entities.insert(participant.entity, participant.role.clone());
            }
        }

        let Some(role) = schema.role(participant.role.as_str()) else {
            report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::UnknownRole(participant.role.clone()),
            );
            continue;
        };

        *counts.entry(role.role.clone()).or_default() += 1;

        match (&participant.slot, role.slot_requirement) {
            (None, SlotRequirement::Required) => report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::SlotRequired {
                    entity: participant.entity,
                    role: role.role.clone(),
                },
            ),
            (Some(slot), SlotRequirement::Forbidden) => report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::SlotForbidden {
                    entity: participant.entity,
                    role: role.role.clone(),
                    slot: slot.clone(),
                },
            ),
            (Some(slot), _) if !role.allows_slot(slot) => report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::SlotNotAllowed {
                    entity: participant.entity,
                    role: role.role.clone(),
                    slot: slot.clone(),
                    allowed: role.allowed_slots.clone(),
                },
            ),
            _ => {}
        }

        match entities.get(participant.entity) {
            Some(entity) => {
                if !role.allows_class(entity.class) {
                    report.push(
                        Some(relationship.uid),
                        RelationshipValidationIssueKind::ParticipantClassMismatch {
                            role: role.role.clone(),
                            entity: participant.entity,
                            actual: entity.class,
                            allowed: role.allowed_classes.clone(),
                        },
                    );
                }
            }
            None => report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::MissingEntity(participant.entity),
            ),
        }
    }

    for role in &schema.roles {
        let actual = counts.get(&role.role).copied().unwrap_or(0);
        if actual < role.minimum {
            report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::MissingRequiredRole {
                    role: role.role.clone(),
                    minimum: role.minimum,
                    actual,
                },
            );
        }

        match role.maximum {
            Some(maximum) if actual > maximum => report.push(
                Some(relationship.uid),
                RelationshipValidationIssueKind::TooManyParticipantsInRole {
                    role: role.role.clone(),
                    maximum,
                    actual,
                },
            ),
            _ => {}
        }
    }

    report
}

impl RelationshipCatalog {
    /// Validate every relationship active at `time`, including cross-record
    /// exclusivity and slot-capacity rules.
    pub fn validate_active_at(
        &self,
        schemas: &RelationshipSchemaRegistry,
        entities: &EntityCatalog,
        time: SimTime,
        context: SchemaContext,
    ) -> RelationshipValidationReport {
        let active: Vec<&Relationship> = self.active_at(time).collect();
        let mut report = RelationshipValidationReport::default();

        for relationship in &active {
            report.extend(validate_relationship(
                relationship,
                schemas,
                entities,
                context,
            ));
        }

        let mut exclusive: BTreeMap<ExclusiveIndexKey, Vec<RelationshipUid>> = BTreeMap::new();

        let mut slotted: BTreeMap<SlotCapacityIndexKey, Vec<RelationshipUid>> = BTreeMap::new();

        for relationship in &active {
            let Some(schema) = schemas.get(
                relationship.schema.key.as_str(),
                relationship.schema.version,
            ) else {
                continue;
            };

            for rule in &schema.rules {
                match rule {
                    RelationshipRule::RoleExclusive { role } => {
                        for participant in relationship.participants_in_role(role.as_str()) {
                            exclusive
                                .entry((
                                    relationship.schema.clone(),
                                    role.clone(),
                                    participant.entity,
                                ))
                                .or_default()
                                .push(relationship.uid);
                        }
                    }
                    RelationshipRule::SlotCapacity {
                        host_role,
                        occupant_role,
                        capacity,
                    } => {
                        let mut hosts = relationship.participants_in_role(host_role.as_str());
                        let Some(host) = hosts.next() else {
                            continue;
                        };
                        if hosts.next().is_some() {
                            continue;
                        }

                        for occupant in relationship.participants_in_role(occupant_role.as_str()) {
                            let Some(slot) = occupant.slot.clone() else {
                                continue;
                            };

                            slotted
                                .entry((
                                    relationship.schema.clone(),
                                    host_role.clone(),
                                    occupant_role.clone(),
                                    host.entity,
                                    slot,
                                    *capacity,
                                ))
                                .or_default()
                                .push(relationship.uid);
                        }
                    }
                }
            }
        }

        for ((schema, role, entity), relationships) in exclusive {
            if relationships.len() > 1 {
                report.push(
                    None,
                    RelationshipValidationIssueKind::RoleExclusiveConflict {
                        schema,
                        role,
                        entity,
                        relationships,
                    },
                );
            }
        }

        for ((schema, _host_role, _occupant_role, host, slot, capacity), relationships) in slotted {
            if relationships.len() > capacity {
                report.push(
                    None,
                    RelationshipValidationIssueKind::SlotCapacityExceeded {
                        schema,
                        host,
                        slot,
                        capacity,
                        actual: relationships.len(),
                        relationships,
                    },
                );
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ParticipantRoleSchema, RelationshipSchema, RelationshipSchemaRef, SlotRequirement,
    };
    use serde_json::json;
    use vbf_entity::{Entity, EntityClass};
    use vbf_types::{DisplayName, Key};

    fn asset(key: &str) -> Entity {
        Entity::new(
            EntityUid::new(),
            Key::new(key).unwrap(),
            DisplayName::new(key).unwrap(),
            EntityClass::Asset,
        )
    }

    fn basic_schema() -> RelationshipSchema {
        RelationshipSchema::new(
            Key::new("relationship.operated_by").unwrap(),
            1,
            Key::new("module.core").unwrap(),
            vec![
                ParticipantRoleSchema::new(
                    Key::new("asset").unwrap(),
                    1,
                    Some(1),
                    vec![EntityClass::Asset],
                    SlotRequirement::Forbidden,
                ),
                ParticipantRoleSchema::new(
                    Key::new("operator").unwrap(),
                    1,
                    Some(1),
                    vec![EntityClass::Actor],
                    SlotRequirement::Optional,
                ),
            ],
            vec![],
            vec![],
        )
        .unwrap()
    }

    #[test]
    fn participant_classes_are_checked_against_entity_catalog() {
        let asset_entity = asset("entity.test.asset");
        let wrong_operator = asset("entity.test.wrong_operator");

        let mut entities = EntityCatalog::new();
        entities.register(asset_entity.clone()).unwrap();
        entities.register(wrong_operator.clone()).unwrap();

        let schema = basic_schema();
        let mut schemas = RelationshipSchemaRegistry::new();
        schemas.register(schema.clone()).unwrap();

        let relationship = Relationship::new(
            RelationshipUid::new(),
            Key::new("relationship.instance.test").unwrap(),
            schema.schema_ref(),
            vec![
                crate::RelationshipParticipant::new(asset_entity.uid, Key::new("asset").unwrap()),
                crate::RelationshipParticipant::new(
                    wrong_operator.uid,
                    Key::new("operator").unwrap(),
                ),
            ],
            json!({}),
        );

        let report = validate_relationship(
            &relationship,
            &schemas,
            &entities,
            SchemaContext::InitialState,
        );

        assert!(report.issues.iter().any(|issue| matches!(
            &issue.kind,
            RelationshipValidationIssueKind::ParticipantClassMismatch { .. }
        )));
    }

    #[test]
    fn unknown_relationship_schema_is_reported() {
        let entities = EntityCatalog::new();
        let schemas = RelationshipSchemaRegistry::new();
        let relationship = Relationship::new(
            RelationshipUid::new(),
            Key::new("relationship.instance.test").unwrap(),
            RelationshipSchemaRef::new(Key::new("relationship.unknown").unwrap(), 1).unwrap(),
            vec![],
            json!({}),
        );

        let report = validate_relationship(
            &relationship,
            &schemas,
            &entities,
            SchemaContext::InitialState,
        );

        assert!(matches!(
            &report.issues[0].kind,
            RelationshipValidationIssueKind::UnknownSchema(_)
        ));
    }
}
