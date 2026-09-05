use serde_json::json;
use vbf_entity::{Entity, EntityCatalog, EntityClass};
use vbf_relationship::{
    ParticipantRoleSchema, Relationship, RelationshipCatalog, RelationshipParticipant,
    RelationshipRule, RelationshipSchema, RelationshipSchemaRegistry,
    RelationshipValidationIssueKind, SlotRequirement,
};
use vbf_schema::SchemaContext;
use vbf_types::{DisplayName, EntityUid, Key, RelationshipUid, SimTime};

fn entity(key: &str, name: &str, class: EntityClass) -> Entity {
    Entity::new(
        EntityUid::new(),
        Key::new(key).unwrap(),
        DisplayName::new(name).unwrap(),
        class,
    )
}

fn m8_entities() -> (EntityCatalog, EntityUid, Vec<(EntityUid, &'static str)>) {
    let m8 = entity(
        "entity.us.42crs.baker.m8_b12",
        "Baker Troop M8 B-12",
        EntityClass::Asset,
    );
    let m8_uid = m8.uid;

    let crew = [
        (
            entity(
                "entity.us.42crs.baker.m8_b12.commander",
                "B-12 Car Commander / Loader",
                EntityClass::Actor,
            ),
            "station.commander_loader",
        ),
        (
            entity(
                "entity.us.42crs.baker.m8_b12.gunner",
                "B-12 Gunner / Radio Operator",
                EntityClass::Actor,
            ),
            "station.gunner_radio",
        ),
        (
            entity(
                "entity.us.42crs.baker.m8_b12.driver",
                "B-12 Driver",
                EntityClass::Actor,
            ),
            "station.driver",
        ),
        (
            entity(
                "entity.us.42crs.baker.m8_b12.assistant_driver",
                "B-12 Radio Operator / Assistant Driver",
                EntityClass::Actor,
            ),
            "station.assistant_driver_radio",
        ),
    ];

    let mut entities = EntityCatalog::new();
    entities.register(m8).unwrap();

    let mut assignments = Vec::new();
    for (actor, slot) in crew {
        assignments.push((actor.uid, slot));
        entities.register(actor).unwrap();
    }

    (entities, m8_uid, assignments)
}

fn crewed_by_schema() -> RelationshipSchema {
    RelationshipSchema::new(
        Key::new("relationship.crewed_by").unwrap(),
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
                Key::new("crew").unwrap(),
                1,
                Some(1),
                vec![EntityClass::Actor],
                SlotRequirement::Required,
            )
            .with_allowed_slots(vec![
                Key::new("station.commander_loader").unwrap(),
                Key::new("station.gunner_radio").unwrap(),
                Key::new("station.driver").unwrap(),
                Key::new("station.assistant_driver_radio").unwrap(),
            ]),
        ],
        vec![],
        vec![
            RelationshipRule::RoleExclusive {
                role: Key::new("crew").unwrap(),
            },
            RelationshipRule::SlotCapacity {
                host_role: Key::new("asset").unwrap(),
                occupant_role: Key::new("crew").unwrap(),
                capacity: 1,
            },
        ],
    )
    .unwrap()
}

fn assignment(index: usize, asset: EntityUid, crew: EntityUid, slot: &str) -> Relationship {
    Relationship::new(
        RelationshipUid::new(),
        Key::new(format!("relationship.us.42crs.b12.crew_{index}")).unwrap(),
        crewed_by_schema().schema_ref(),
        vec![
            RelationshipParticipant::new(asset, Key::new("asset").unwrap()),
            RelationshipParticipant::new(crew, Key::new("crew").unwrap())
                .in_slot(Key::new(slot).unwrap()),
        ],
        json!({}),
    )
}

fn schema_registry() -> RelationshipSchemaRegistry {
    let mut schemas = RelationshipSchemaRegistry::new();
    schemas.register(crewed_by_schema()).unwrap();
    schemas
}

#[test]
fn m8_b12_accepts_four_distinct_crew_station_assignments() {
    let (entities, m8, crew) = m8_entities();
    let schemas = schema_registry();
    let mut relationships = RelationshipCatalog::new();

    for (index, (actor, slot)) in crew.into_iter().enumerate() {
        relationships
            .register(assignment(index, m8, actor, slot))
            .unwrap();
    }

    let report = relationships.validate_active_at(
        &schemas,
        &entities,
        SimTime::ZERO,
        SchemaContext::InitialState,
    );

    assert!(report.is_valid(), "{:#?}", report.issues);
}

#[test]
fn m8_station_capacity_one_rejects_double_occupancy() {
    let (entities, m8, crew) = m8_entities();
    let schemas = schema_registry();
    let mut relationships = RelationshipCatalog::new();

    relationships
        .register(assignment(0, m8, crew[0].0, "station.driver"))
        .unwrap();
    relationships
        .register(assignment(1, m8, crew[1].0, "station.driver"))
        .unwrap();

    let report = relationships.validate_active_at(
        &schemas,
        &entities,
        SimTime::ZERO,
        SchemaContext::InitialState,
    );

    assert!(report.issues.iter().any(|issue| matches!(
        &issue.kind,
        RelationshipValidationIssueKind::SlotCapacityExceeded { .. }
    )));
}

#[test]
fn m8_crew_actor_cannot_hold_two_active_crewed_by_assignments() {
    let (mut entities, m8, crew) = m8_entities();
    let second_m8 = entity(
        "entity.us.42crs.baker.m8_b13",
        "Baker Troop M8 B-13",
        EntityClass::Asset,
    );
    let second_m8_uid = second_m8.uid;
    entities.register(second_m8).unwrap();

    let schemas = schema_registry();
    let mut relationships = RelationshipCatalog::new();

    relationships
        .register(assignment(0, m8, crew[2].0, "station.driver"))
        .unwrap();
    relationships
        .register(assignment(1, second_m8_uid, crew[2].0, "station.driver"))
        .unwrap();

    let report = relationships.validate_active_at(
        &schemas,
        &entities,
        SimTime::ZERO,
        SchemaContext::InitialState,
    );

    assert!(report.issues.iter().any(|issue| matches!(
        &issue.kind,
        RelationshipValidationIssueKind::RoleExclusiveConflict { .. }
    )));
}

#[test]
fn m8_crew_assignment_rejects_unknown_station() {
    let (entities, m8, crew) = m8_entities();
    let schemas = schema_registry();
    let mut relationships = RelationshipCatalog::new();

    relationships
        .register(assignment(0, m8, crew[0].0, "station.nonexistent"))
        .unwrap();

    let report = relationships.validate_active_at(
        &schemas,
        &entities,
        SimTime::ZERO,
        SchemaContext::InitialState,
    );

    assert!(report.issues.iter().any(|issue| matches!(
        &issue.kind,
        RelationshipValidationIssueKind::SlotNotAllowed { .. }
    )));
}

#[test]
fn station_handoff_at_same_sim_time_does_not_overlap() {
    let (entities, m8, crew) = m8_entities();
    let schemas = schema_registry();
    let handoff = SimTime::from_millis(60_000);
    let mut relationships = RelationshipCatalog::new();

    relationships
        .register(assignment(0, m8, crew[2].0, "station.driver").with_validity(None, Some(handoff)))
        .unwrap();

    relationships
        .register(assignment(1, m8, crew[3].0, "station.driver").with_validity(Some(handoff), None))
        .unwrap();

    let report =
        relationships.validate_active_at(&schemas, &entities, handoff, SchemaContext::RuntimeState);

    assert!(report.is_valid(), "{:#?}", report.issues);
}
