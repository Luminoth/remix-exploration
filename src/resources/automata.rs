//! Automata resources

use bevy::prelude::*;

/// Materials container resource
pub struct Materials {
    pub player_automata: Handle<ColorMaterial>,
    pub ai_automata: Handle<ColorMaterial>,
}
