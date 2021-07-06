//! Automata resources

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Inspectable, Eq, PartialEq, Copy, Clone)]
pub enum StatModifierType {
    Fortitude,
}

/// Player automata stats
#[derive(Debug, Default, Clone, Copy)]
pub struct PlayerAutomataStats {
    pub points: isize,

    pub fortitude: isize,
}

impl PlayerAutomataStats {
    pub fn new(points: isize) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }

    pub fn value(&self, r#type: StatModifierType) -> isize {
        match r#type {
            StatModifierType::Fortitude => self.fortitude,
        }
    }

    pub fn modify(&mut self, r#type: StatModifierType, amount: isize) -> bool {
        if self.points - amount < 0 {
            return false;
        }

        match r#type {
            StatModifierType::Fortitude => {
                if self.fortitude + amount < 0 {
                    return false;
                }
                self.fortitude += amount;
            }
        }

        self.points -= amount;

        true
    }
}

/// Materials container resource
pub struct Materials {
    pub player_automata: Handle<ColorMaterial>,
    pub ai_automata: Handle<ColorMaterial>,
}
