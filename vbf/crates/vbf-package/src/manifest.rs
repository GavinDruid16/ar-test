use crate::PackageDependency;
use serde::{Deserialize, Serialize};
use vbf_types::{DisplayName, Key, PackageUid};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageKind {
    Rules,
    Content,
    Map,
    Scenario,
    Instance,
    Campaign,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageManifest {
    pub uid: PackageUid,
    pub key: Key,
    pub name: DisplayName,
    pub kind: PackageKind,
    pub version: String,
    pub layer0_schema_version: u32,
    pub dependencies: Vec<PackageDependency>,
}
