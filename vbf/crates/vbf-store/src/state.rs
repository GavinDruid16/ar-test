use std::collections::BTreeMap;
use vbf_entity::Entity;
use vbf_information::InformationRecord;
use vbf_relationship::Relationship;
use vbf_spatial::{AngularVelocity3, Orientation3, SpatialPosition, Velocity3};
use vbf_types::{EntityUid, Key, RelationshipUid, SimTime, StateRevision};
#[derive(Clone, Debug)]
pub struct WorldState {
    pub revision: StateRevision,
    pub sim_time: SimTime,
    pub entities: BTreeMap<EntityUid, Entity>,
    pub relationships: BTreeMap<RelationshipUid, Relationship>,
    pub positions: BTreeMap<EntityUid, SpatialPosition>,
    pub orientations: BTreeMap<EntityUid, Orientation3>,
    pub linear_velocities: BTreeMap<EntityUid, Velocity3>,
    pub angular_velocities: BTreeMap<EntityUid, AngularVelocity3>,
    pub information: BTreeMap<Key, InformationRecord>,
}
impl Default for WorldState {
    fn default() -> Self {
        Self {
            revision: StateRevision::INITIAL,
            sim_time: SimTime::ZERO,
            entities: BTreeMap::new(),
            relationships: BTreeMap::new(),
            positions: BTreeMap::new(),
            orientations: BTreeMap::new(),
            linear_velocities: BTreeMap::new(),
            angular_velocities: BTreeMap::new(),
            information: BTreeMap::new(),
        }
    }
}
