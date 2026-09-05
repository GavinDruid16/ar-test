use crate::LocalPose;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vbf_types::{DefinitionUid, EntityUid, Key};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialAnchorDefinition {
    pub key: Key,
    pub local_pose: LocalPose,
}
impl SpatialAnchorDefinition {
    pub fn new(key: Key, local_pose: LocalPose) -> Self {
        Self { key, local_pose }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinitionAnchors {
    pub definition: DefinitionUid,
    pub anchors: BTreeMap<Key, SpatialAnchorDefinition>,
}
impl DefinitionAnchors {
    pub fn new(definition: DefinitionUid) -> Self {
        Self {
            definition,
            anchors: BTreeMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        anchor: SpatialAnchorDefinition,
    ) -> Option<SpatialAnchorDefinition> {
        self.anchors.insert(anchor.key.clone(), anchor)
    }
    pub fn get(&self, key: &str) -> Option<&SpatialAnchorDefinition> {
        self.anchors.get(key)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostedAnchorRef {
    pub host: EntityUid,
    pub anchor: Key,
}