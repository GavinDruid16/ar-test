//! Continuous spatial primitives.
//!
//! Hex grids are optional overlays. World position is expressed in coordinate
//! frames and integer millimetres.
pub mod anchor;
pub mod frame;
pub mod geometry;
pub mod grid;
pub mod motion;
pub mod network;
pub mod pose;
pub mod position;
pub mod region;
pub use anchor::{DefinitionAnchors, HostedAnchorRef, SpatialAnchorDefinition};
pub use frame::{AxisDirection, CoordinateFrame};
pub use geometry::{Geometry2, Point2};
pub use grid::{GridOverlay, GridRole, HexOrientation};
pub use motion::{AngularVelocity3, MotionSample, Velocity3};
pub use network::{Network, NetworkEdge, NetworkNode};
pub use pose::{LocalPose, Orientation3, Vector3Mm, WorldPose};
pub use position::{HostedPosition, SpatialPosition, WorldPosition};
pub use region::SpatialRegion;
