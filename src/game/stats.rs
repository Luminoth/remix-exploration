//! Automata stats

use bevy_inspector_egui::Inspectable;

use crate::resources::automata::*;

/// A single automata stat
#[derive(Debug, Clone, Copy, Inspectable, Default)]
pub struct Stat {
    value: isize,
}

impl From<isize> for Stat {
    fn from(value: isize) -> Self {
        Self { value }
    }
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

impl From<PlayerAutomataStats> for StatSet {
    fn from(stats: PlayerAutomataStats) -> Self {
        Self {
            fortitude: stats.fortitude.into(),
        }
    }
}

impl From<AIAutomataStats> for StatSet {
    fn from(stats: AIAutomataStats) -> Self {
        Self {
            fortitude: stats.fortitude.into(),
        }
    }
}

impl StatSet {
    /// Creates a new stat set
    #[allow(dead_code)]
    pub fn new(fortitude: Stat) -> Self {
        Self { fortitude }
    }

    /// Gets the value of the fortitude state
    pub fn fortitude(&self) -> isize {
        self.fortitude.value()
    }
}
