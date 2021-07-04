//! GridWorld bundles

use bevy::prelude::*;

use crate::components::gridworld::*;

/// GridWorld cell component bundle
#[derive(Bundle)]
pub struct GridWorldCellBundle {
    pub cell: GridWorldCell,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
