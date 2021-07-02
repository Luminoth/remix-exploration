//! Automata components

use bevy_inspector_egui::Inspectable;

use crate::game::stats::*;

/// Automata state
#[derive(Debug, Inspectable, Default)]
pub struct Automata {
    fortitude: Stat,

    health: usize,
}

impl Automata {
    pub fn new(fortitude: Stat) -> Self {
        let mut automata = Self {
            fortitude,
            ..Default::default()
        };

        automata.reset();

        automata
    }

    pub fn initial_health(&self) -> usize {
        (10 + self.fortitude.value()).max(0) as usize
    }

    pub fn reset(&mut self) {
        self.health = self.initial_health();
    }
}
