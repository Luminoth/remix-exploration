//! Automata resources

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::resources::*;

/// Which stat should be modified
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
    /// Creates new player automata stats
    pub fn new(points: isize) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }

    /// Gets the value of a stat
    pub fn value(&self, r#type: StatModifierType) -> isize {
        match r#type {
            StatModifierType::Fortitude => self.fortitude,
        }
    }

    /// Modifies a stat by amount
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

/// AI automata stats
#[derive(Debug, Default, Clone, Copy)]
pub struct AIAutomataStats {
    pub points: isize,

    pub fortitude: isize,
}

impl AIAutomataStats {
    /// Creates new, randomized AI automata stats
    pub fn new(mut points: isize, random: &mut Random) -> Self {
        let mut stats = Self {
            points,
            ..Default::default()
        };

        // shuffle the stat types
        let mut buckets = vec![StatModifierType::Fortitude];
        random.shuffle(&mut buckets);

        // random points for each stat
        for stat in buckets.drain(1..) {
            let p = random.random_range(0..=points);
            stats.modify(stat, p);

            points -= p;
        }
        stats.modify(buckets[0], points);

        stats
    }

    /// Modifies a stat by amount
    fn modify(&mut self, r#type: StatModifierType, amount: isize) -> bool {
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
