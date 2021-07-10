//! Automata stats

use std::borrow::Cow;

use bevy_inspector_egui::Inspectable;

use crate::resources::*;

/// Base automata health
const BASE_HEALTH: isize = 10;

/// Stat identifier enum for things that need it
#[derive(Debug, Inspectable, Eq, PartialEq, Copy, Clone)]
pub enum StatId {
    Fortitude,
}

pub fn stat_name(statid: StatId) -> Cow<'static, str> {
    match statid {
        StatId::Fortitude => "Fortitude".into(),
    }
}

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
    pub fn size() -> usize {
        1
    }

    /// Creates a new stat set
    #[allow(dead_code)]
    pub fn new(fortitude: Stat) -> Self {
        Self { fortitude }
    }

    /// Crates a new, randomized stat set
    pub fn random(mut points: isize, random: &mut Random) -> Self {
        let mut stats = Self::default();

        // shuffle the stat types
        let mut buckets = vec![StatId::Fortitude];
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

    /// Returns the value of the given stat
    pub fn value(&self, statid: StatId) -> isize {
        match statid {
            StatId::Fortitude => self.fortitude(),
        }
    }

    /// Randomizes a single stat
    pub fn randomize_stat(&mut self, statid: StatId) {
        // TODO: not sure how to handle this,
        // if the stat value changes we need to shuffle other stats around
        match statid {
            StatId::Fortitude => {
                // TODO:
            }
        }
    }

    /// Modifies a stat by amount
    #[inline]
    fn modify(&mut self, statid: StatId, amount: isize) {
        match statid {
            StatId::Fortitude => {
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
