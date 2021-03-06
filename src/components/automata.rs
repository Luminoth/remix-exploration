//! Automata components

use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::automata::*;
use crate::resources::automata::*;
use crate::resources::*;
use crate::util::*;

/// Automata state
#[derive(Debug, Default, Component, Inspectable)]
pub struct Automata {
    /// Current HP (health)
    pub health: usize,
    // TODO:
    // # of moves made towards enemy
    // # of moves made towards food
    // # of moves made total
    // damage dealt
    // damage absorbed
}

impl Automata {
    fn spawn(
        commands: &mut Commands,
        parent: Entity,
        cell: UVec2,
        color: Color,
        name: impl Into<String>,
    ) -> Entity {
        let position = cell_position(cell, 1.0);
        debug!("Automata position: {}", position);

        let mut ret = None;

        commands.entity(parent).with_children(|parent| {
            let entity = parent
                .spawn_bundle(AutomataBundle {
                    automata: Automata::default(),
                    transform: Transform::from_translation(position),
                    global_transform: GlobalTransform::default(),
                })
                .insert(Name::new(name.into()))
                .with_children(|parent| {
                    parent.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(crate::CELL_WIDTH, crate::CELL_HEIGHT)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                })
                .id();

            ret = Some(entity);
        });

        ret.unwrap()
    }

    /// Spawn a new player automata
    pub fn spawn_player(commands: &mut Commands, parent: Entity, color: Color, player_cell: UVec2) {
        info!("Spawning player at {}", player_cell);

        let entity = Automata::spawn(commands, parent, player_cell, color, "Player automata");

        commands.entity(entity).insert(PlayerAutomata);
    }

    /// Spawn a new AI automata population
    pub fn spawn_ai(
        commands: &mut Commands,
        parent: Entity,
        color: Color,
        player_cell: UVec2,
        random: &mut Random,
    ) {
        // spawn AI in mirror cell
        // TODO: this isn't working exactly correct
        let midpoint = UVec2::new(crate::GRID_WIDTH as u32 / 2, crate::GRID_HEIGHT as u32 / 2);
        let ai_cell = UVec2::new(
            match player_cell.x.cmp(&midpoint.x) {
                Ordering::Less => midpoint.x + (midpoint.x - player_cell.x),
                Ordering::Greater => midpoint.x - (player_cell.x - midpoint.x),
                _ => {
                    if random.coin() {
                        0
                    } else {
                        crate::GRID_WIDTH as u32 - 1
                    }
                }
            },
            match player_cell.y.cmp(&midpoint.y) {
                Ordering::Less => midpoint.y + (midpoint.y - player_cell.y),
                Ordering::Greater => midpoint.y - (player_cell.y - midpoint.y),
                _ => {
                    if random.coin() {
                        0
                    } else {
                        crate::GRID_HEIGHT as u32 - 1
                    }
                }
            },
        );

        info!("Spawning AI at {}", ai_cell);

        let entity = Automata::spawn(commands, parent, ai_cell, color, "AI automata");

        commands.entity(entity).insert(AIAutomata);
    }

    /// Resets an automata to its initial state
    pub fn reset(&mut self, stats: &dyn AutomataStats) {
        self.health = stats.stats().initial_health();
    }

    pub fn move_action(&mut self) {
        debug!("move");
    }

    pub fn attack_action(&mut self) {
        debug!("attack");
    }
}

/// Player automata tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct PlayerAutomata;

/// AI automata state
#[derive(Debug, Component, Inspectable)]
pub struct AIAutomata;
