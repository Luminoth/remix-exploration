//! Automata components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bundles::automata::*;
use crate::game::stats::*;
use crate::resources;
use crate::resources::automata::*;

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
}

impl Automata {
    fn spawn(
        commands: &mut Commands,
        stats: AutomataStats,
        cell: UVec2,
        material: Handle<ColorMaterial>,
        name: impl Into<String>,
    ) -> Entity {
        let position = Vec2::new(cell.x as f32, cell.y as f32)
            * Vec2::new(crate::CELL_WIDTH as f32, crate::CELL_HEIGHT as f32);
        let position = position.extend(0.0);

        commands
            .spawn_bundle(AutomataBundle {
                automata: Automata::new(&stats),
                stats,
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
        stats: PlayerAutomataStats,
        cell: UVec2,
    ) {
        let stats = AutomataStats::new(stats.into());

        let entity = Automata::spawn(
            commands,
            stats,
            cell,
            materials.player_automata.clone(),
            "Player automata",
        );
        commands.entity(entity).insert(PlayerAutomata);
    }

    /// Spawn a new AI automata
    pub fn spawn_ai(
        commands: &mut Commands,
        materials: &resources::automata::Materials,
        stats: AIAutomataStats,
        cell: UVec2,
    ) {
        let stats = AutomataStats::new(stats.into());

        let entity = Automata::spawn(
            commands,
            stats,
            cell,
            materials.ai_automata.clone(),
            "AI automata",
        );
        commands.entity(entity).insert(AIAutomata);
    }

    /// Creates a new automata component
    pub fn new(stats: &AutomataStats) -> Self {
        let mut automata = Self::default();

        automata.reset(stats);
        automata
    }

    /// Resets an automata to its initial state
    pub fn reset(&mut self, stats: &AutomataStats) {
        self.health = stats.initial_health();
    }
}

/// Player automata tag
#[derive(Debug, Inspectable, Default)]
pub struct PlayerAutomata;

/// AI automata state
#[derive(Debug, Inspectable, Default)]
pub struct AIAutomata;
