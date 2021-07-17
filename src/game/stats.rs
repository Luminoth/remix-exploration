//! Automata stats

use std::borrow::Cow;

use bevy_inspector_egui::Inspectable;
use paste::paste;

use crate::resources::*;

/// Base automata health
const BASE_HEALTH: isize = 10;

/// Base automata movement
const BASE_MOVEMENT: isize = 1;

/// Stat identifier enum for things that need it
#[derive(Debug, /*Inspectable,*/ Eq, PartialEq, Copy, Clone)]
pub enum StatId {
    /// Fortitude - HP
    Fortitude,

    /// Dexterity - Movement
    Dexterity,
}

impl StatId {
    // TODO: replace this with a From<> impl
    pub fn name(&self) -> Cow<'static, str> {
        match self {
            StatId::Fortitude => "Fortitude".into(),
            StatId::Dexterity => "Dexterity".into(),
        }
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
    fortitude: Stat,
    dexterity: Stat,
}

macro_rules! impl_stat {
    ($statid:tt) => {
        /// Gets the value of the stat
        #[inline]
        pub fn $statid(&self) -> isize {
            self.$statid.value
        }

        paste! {
            /// Sets the value of the stat
            #[inline]
            pub fn [<set_ $statid>](&mut self, value: isize) {
                self.$statid.value = value;
            }
        }
    };
}

impl StatSet {
    pub fn size() -> usize {
        1
    }

    /// Creates a new stat set
    #[allow(dead_code)]
    pub fn new(fortitude: Stat, dexterity: Stat) -> Self {
        Self {
            fortitude,
            dexterity,
        }
    }

    /// Crates a new, randomized stat set
    pub fn random(mut points: isize, random: &mut Random) -> Self {
        let mut stats = Self::default();

        // shuffle the stat types
        let mut buckets = vec![StatId::Fortitude, StatId::Dexterity];
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
            StatId::Dexterity => self.dexterity(),
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
            StatId::Dexterity => {
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
            StatId::Dexterity => {
                self.dexterity.value += amount;
            }
        }
    }

    impl_stat!(fortitude);

    /// Gets the automata initial health, based on Fortitude stat
    #[inline]
    pub fn initial_health(&self) -> usize {
        (BASE_HEALTH + self.fortitude()).max(1) as usize
    }

    impl_stat!(dexterity);

    /// Gets the automata movement, based on Dexterity stat
    #[inline]
    pub fn movement(&self) -> usize {
        (BASE_MOVEMENT + self.dexterity() / 5).max(1) as usize
    }
}
