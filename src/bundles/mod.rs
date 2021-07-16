//! ECS bundles

use bevy::prelude::*;

pub mod automata;
pub mod gridworld;
pub mod ui;

/// Empty bundle
#[derive(Bundle, Default)]
pub struct EmptyBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
