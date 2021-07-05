//! Automata components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bundles::automata::*;
use crate::game::stats::*;
use crate::resources;
use crate::{CELL_X_PIXELS, CELL_Y_PIXELS};

/// Base automata health
const BASE_HEALTH: isize = 10;

/// Automata stats
#[derive(Debug, Inspectable, Default)]
pub struct AutomataStats {
    stats: StatSet,
}

impl AutomataStats {
    /// Creates a new automata stats component
    pub fn new(stats: StatSet) -> Self {
        Self { stats }
    }

    /// Gets the automata initial health, based on Fortitude stat
    pub fn initial_health(&self) -> usize {
        (BASE_HEALTH + self.stats.fortitude()).max(1) as usize
    }
}

/// Automata state
#[derive(Debug, Inspectable, Default)]
pub struct Automata {
    /// Current HP (health)
    health: usize,

    /// Is this automata the player's automata or the AI's?
    player: bool,
}

impl Automata {
    /// Spawn a new automata
    pub fn spawn(
        commands: &mut Commands,
        materials: Res<resources::automata::Materials>,
        stats: StatSet,
        player: bool,
        cell: Vec2,
    ) {
        let stats = AutomataStats::new(stats);

        let (name, material) = if player {
            ("Player automata", materials.player_automata.clone())
        } else {
            ("AI automata", materials.ai_automata.clone())
        };

        let position = cell * Vec2::new(CELL_X_PIXELS, CELL_Y_PIXELS);
        let position = position.extend(0.0);

        commands
            .spawn_bundle(AutomataBundle {
                automata: Automata::new(&stats, player),
                stats,
                transform: Transform::from_translation(position),
                global_transform: GlobalTransform::default(),
            })
            .insert(Name::new(name))
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    material,
                    sprite: Sprite::new(Vec2::new(CELL_X_PIXELS, CELL_Y_PIXELS)),
                    ..Default::default()
                });
            });
    }

    /// Creates a new automata component
    pub fn new(stats: &AutomataStats, player: bool) -> Self {
        let mut automata = Self {
            player,
            ..Default::default()
        };

        automata.reset(stats);
        automata
    }

    /// Resets an automata to its initial state
    pub fn reset(&mut self, stats: &AutomataStats) {
        self.health = stats.initial_health();
    }
}
