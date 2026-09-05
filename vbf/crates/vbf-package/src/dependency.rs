use serde::{Deserialize, Serialize};
use vbf_types::Key;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageDependency {
    pub package: Key,
    pub version_requirement: String,
    pub content_hash: Option<String>,
}
