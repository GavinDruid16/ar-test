use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vbf_types::Key;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkNode {
    pub key: Key,
    pub x_mm: i64,
    pub y_mm: i64,
    pub z_mm: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEdge {
    pub key: Key,
    pub from: Key,
    pub to: Key,
    pub length_mm: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Network {
    pub key: Option<Key>,
    pub nodes: BTreeMap<Key, NetworkNode>,
    pub edges: BTreeMap<Key, NetworkEdge>,
}
