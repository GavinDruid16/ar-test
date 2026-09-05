use serde_json::json;
use vbf_entity::{
    ComponentData, Definition, DefinitionCatalog, Entity, EntityCatalog, EntityClass,
};
use vbf_schema::{
    ComponentSchema, ComponentSchemaRef, FieldConstraint, FieldRequirement, FieldSchema, FieldType,
    PersistenceClass, SchemaContext, SchemaRegistry,
};
use vbf_types::{DefinitionUid, DisplayName, EntityUid, Key};

fn field(key: &str, field_type: FieldType, persistence: PersistenceClass) -> FieldSchema {
    FieldSchema::new(
        Key::new(key).unwrap(),
        field_type,
        FieldRequirement::Required,
        persistence,
    )
}

fn schemas() -> SchemaRegistry {
    let vehicle_profile = ComponentSchema::new(
        Key::new("component.vehicle_profile").unwrap(),
        1,
        Key::new("module.vehicle").unwrap(),
        vec![
            field(
                "vehicle_class",
                FieldType::Key,
                PersistenceClass::Definition,
            ),
            field("drive", FieldType::Key, PersistenceClass::Definition),
            field(
                "crew_capacity",
                FieldType::UnsignedInteger,
                PersistenceClass::Definition,
            ),
            field("armored", FieldType::Bool, PersistenceClass::Definition),
            field(
                "open_topped_turret",
                FieldType::Bool,
                PersistenceClass::Definition,
            ),
            field(
                "fuel_capacity_us_gal",
                FieldType::Decimal,
                PersistenceClass::Definition,
            ),
        ],
    )
    .unwrap();

    let mobility_state = field("mobility_state", FieldType::Key, PersistenceClass::Mutable)
        .with_constraint(FieldConstraint::AllowedStrings {
            values: vec![
                "mobility.ready".into(),
                "mobility.degraded".into(),
                "mobility.restricted".into(),
                "mobility.bogged".into(),
                "mobility.immobilized".into(),
                "mobility.disabled".into(),
                "mobility.destroyed".into(),
                "mobility.overturned".into(),
            ],
        });

    let vehicle_state = ComponentSchema::new(
        Key::new("component.vehicle_state").unwrap(),
        1,
        Key::new("module.vehicle").unwrap(),
        vec![mobility_state],
    )
    .unwrap();

    let mut registry = SchemaRegistry::new();
    registry.register(vehicle_profile).unwrap();
    registry.register(vehicle_state).unwrap();
    registry
}

#[test]
fn m8_definition_and_b12_instance_validate_against_real_game_shape() {
    let schemas = schemas();

    let m8_definition_uid = DefinitionUid::new();
    let mut m8 = Definition::new(
        m8_definition_uid,
        Key::new("definition.us.m8_light_armored_car").unwrap(),
        DisplayName::new("M8 Light Armored Car").unwrap(),
        EntityClass::Asset,
    );
    m8.set_component(ComponentData::new(
        ComponentSchemaRef::new(Key::new("component.vehicle_profile").unwrap(), 1).unwrap(),
        json!({
            "vehicle_class": "vehicle.armored_reconnaissance",
            "drive": "drive.6x6",
            "crew_capacity": 4,
            "armored": true,
            "open_topped_turret": true,
            "fuel_capacity_us_gal": 54.0
        }),
    ));
    assert!(m8.validate_components(&schemas).is_valid());

    let mut definitions = DefinitionCatalog::new();
    definitions.register(m8).unwrap();

    let mut b12 = Entity::new(
        EntityUid::new(),
        Key::new("entity.us.42crs.baker.m8_b12").unwrap(),
        DisplayName::new("Baker Troop M8 B-12").unwrap(),
        EntityClass::Asset,
    )
    .with_template(m8_definition_uid);
    b12.set_component(ComponentData::new(
        ComponentSchemaRef::new(Key::new("component.vehicle_state").unwrap(), 1).unwrap(),
        json!({"mobility_state": "mobility.ready"}),
    ));

    assert!(
        b12.validate(&schemas, &definitions, SchemaContext::InitialState)
            .is_valid()
    );
}

#[test]
fn m8_historical_four_person_crew_can_exist_as_distinct_actor_entities() {
    let roles = [
        ("commander", "Car Commander / Loader"),
        ("gunner", "Gunner / Radio Operator"),
        ("driver", "Driver"),
        ("assistant_driver", "Radio Operator / Assistant Driver"),
    ];

    let mut entities = EntityCatalog::new();
    for (key_suffix, display_role) in roles {
        let actor = Entity::new(
            EntityUid::new(),
            Key::new(format!("entity.us.42crs.baker.m8_b12.{key_suffix}")).unwrap(),
            DisplayName::new(format!("B-12 {display_role}")).unwrap(),
            EntityClass::Actor,
        );
        entities.register(actor).unwrap();
    }

    assert_eq!(entities.len(), 4);
    assert!(
        entities
            .get_by_key("entity.us.42crs.baker.m8_b12.commander")
            .is_some()
    );
}
