use serde::{Deserialize, Serialize};
use vbf_entity::{ComponentData, Entity};
use vbf_relationship::Relationship;
use vbf_spatial::{AngularVelocity3, Orientation3, SpatialPosition, Velocity3};
use vbf_types::{EntityUid, Key, RelationshipUid};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum StateMutation {
    AddEntity {
        entity: Entity,
    },
    RemoveEntity {
        entity: EntityUid,
    },
    SetComponent {
        entity: EntityUid,
        component: ComponentData,
    },
    RemoveComponent {
        entity: EntityUid,
        component: Key,
    },
    AddRelationship {
        relationship: Relationship,
    },
    EndRelationship {
        relationship: RelationshipUid,
    },
    SetPosition {
        entity: EntityUid,
        position: SpatialPosition,
    },
    SetOrientation {
        entity: EntityUid,
        orientation: Orientation3,
    },
    SetLinearVelocity {
        entity: EntityUid,
        velocity: Velocity3,
    },
    SetAngularVelocity {
        entity: EntityUid,
        velocity: AngularVelocity3,
    },
}
