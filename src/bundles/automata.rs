//! Automata bundles

use bevy::prelude::*;

use crate::components::automata::*;

/// Automata component bundle
#[derive(Bundle)]
pub struct AutomataBundle {
    pub automata: Automata,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
