//! Automata components

use bevy_inspector_egui::Inspectable;

use crate::game::stats::*;

/// Automata state
#[derive(Debug, Inspectable, Default)]
pub struct Automata {
    /// Fortitude - HP
    fortitude: Stat,

    /// Current HP
    health: usize,
}

impl Automata {
    /// Creates a new automata component
    pub fn new(fortitude: Stat) -> Self {
        let mut automata = Self {
            fortitude,
            ..Default::default()
        };

        automata.reset();

        automata
    }

    /// Gets the automata initial health, based on Fortitude stat
    pub fn initial_health(&self) -> usize {
        (10 + self.fortitude.value()).max(0) as usize
    }

    /// Resets an automata to its initial state
    pub fn reset(&mut self) {
        self.health = self.initial_health();
    }
}
