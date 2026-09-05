use serde::{Deserialize, Serialize};
use vbf_entity::{Definition, Entity};
use vbf_information::InformationRecord;
use vbf_package::PackageManifest;
use vbf_relationship::Relationship;
use vbf_spatial::{CoordinateFrame, GridOverlay, SpatialRegion};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer0Source {
    pub manifest: PackageManifest,
    pub definitions: Vec<Definition>,
    pub entities: Vec<Entity>,
    pub relationships: Vec<Relationship>,
    pub coordinate_frames: Vec<CoordinateFrame>,
    pub regions: Vec<SpatialRegion>,
    pub grids: Vec<GridOverlay>,
    pub information: Vec<InformationRecord>,
}
