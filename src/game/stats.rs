//! Automata stats

use bevy_inspector_egui::Inspectable;

/// A single automata stat
#[derive(Debug, Clone, Copy, Inspectable, Default)]
pub struct Stat {
    value: isize,
}

impl Stat {
    /// Gets the stat value
    pub fn value(&self) -> isize {
        self.value
    }
}

/// A set of automata stats
#[derive(Debug, Clone, Copy, Inspectable, Default)]
pub struct StatSet {
    /// Fortitude - HP
    fortitude: Stat,
}

impl StatSet {
    /// Creates a new stat set
    pub fn new(fortitude: Stat) -> Self {
        Self {
            fortitude,
            ..Default::default()
        }
    }

    /// Gets the value of the fortitude state
    pub fn fortitude(&self) -> isize {
        self.fortitude.value()
    }
}
