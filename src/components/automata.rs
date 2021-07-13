//! Automata components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bundles::automata::*;
use crate::resources;
use crate::resources::automata::*;
use crate::util::*;

/// Automata state
#[derive(Debug, Inspectable, Default)]
pub struct Automata {
    /// Current HP (health)
    health: usize,
}

impl Automata {
    fn spawn(
        commands: &mut Commands,
        cell: UVec2,
        material: Handle<ColorMaterial>,
        name: impl Into<String>,
    ) -> Entity {
        let position = cell_position(cell);
        debug!("Position: {}", position);

        commands
            .spawn_bundle(AutomataBundle {
                automata: Automata::default(),
                transform: Transform::from_translation(position),
                global_transform: GlobalTransform::default(),
            })
            .insert(Name::new(name.into()))
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    material,
                    sprite: Sprite::new(Vec2::new(
                        crate::CELL_WIDTH as f32,
                        crate::CELL_HEIGHT as f32,
                    )),
                    ..Default::default()
                });
            })
            .id()
    }

    /// Spawn a new player automata
    pub fn spawn_player(
        commands: &mut Commands,
        materials: &resources::automata::Materials,
        cell: UVec2,
    ) {
        let entity = Automata::spawn(
            commands,
            cell,
            materials.player_automata.clone(),
            "Player automata",
        );

        commands.entity(entity).insert(PlayerAutomata);
    }

    /// Spawn a new AI automata population
    pub fn spawn_ai(
        commands: &mut Commands,
        materials: &resources::automata::Materials,
        cell: UVec2,
    ) {
        let entity = Automata::spawn(commands, cell, materials.ai_automata.clone(), "AI automata");

        commands.entity(entity).insert(AIAutomata);
    }

    /// Resets an automata to its initial state
    pub fn reset(&mut self, stats: &dyn AutomataStats) {
        self.health = stats.stats().initial_health();
    }
}

/// Player automata tag
#[derive(Debug, Inspectable, Default)]
pub struct PlayerAutomata;

/// AI automata state
#[derive(Debug, Inspectable)]
pub struct AIAutomata;
