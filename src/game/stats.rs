//! Automata stats

use bevy_inspector_egui::Inspectable;

use crate::resources::automata::*;
use crate::resources::*;

/// Base automata health
const BASE_HEALTH: isize = 10;

/// A single automata stat
#[derive(Debug, Clone, Copy, Inspectable, Default)]
pub struct Stat {
    value: isize,
}

impl From<isize> for Stat {
    #[inline]
    fn from(value: isize) -> Self {
        Self { value }
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
    #[allow(dead_code)]
    pub fn new(fortitude: Stat) -> Self {
        Self { fortitude }
    }

    /// Crates a new, randomized stat set
    pub fn random(mut points: isize, random: &mut Random) -> Self {
        let mut stats = Self::default();

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

    /// Randomizes a single random stat
    ///
    /// Will never increase the state beyond its current value
    pub fn randomize_random_stat(&mut self) {
        // TODO: not sure how to handle this,
        // if the stat value changes we need to shuffle other stats around
    }

    /// Modifies a stat by amount
    fn modify(&mut self, r#type: StatModifierType, amount: isize) {
        match r#type {
            StatModifierType::Fortitude => {
                self.fortitude.value += amount;
            }
        }
    }

    /// Gets the value of the fortitude stat
    #[inline]
    pub fn fortitude(&self) -> isize {
        self.fortitude.value
    }

    /// Sets the value of the fortitude stat
    #[inline]
    pub fn set_fortitude(&mut self, value: isize) {
        self.fortitude.value = value;
    }

    /// Gets the automata initial health, based on Fortitude stat
    #[inline]
    pub fn initial_health(&self) -> usize {
        (BASE_HEALTH + self.fortitude()).max(1) as usize
    }
}
