//! Automata stats

use bevy_inspector_egui::Inspectable;

/// A single automata stat
#[derive(Debug, Clone, Copy, Inspectable, Default)]
pub struct Stat {
    value: isize,
}

impl Stat {
    pub fn value(&self) -> isize {
        self.value
    }
}
