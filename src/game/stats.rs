//! Automata stats

use std::borrow::Cow;

use bevy_inspector_egui::Inspectable;
use paste::paste;

use crate::resources::*;

/// Base automata health
const BASE_HEALTH: isize = 10;

/// Constitution modifier for calculating initial health
const CONSTITUTION_MOD: f32 = 1.0;

/// Base automata movement
const BASE_MOVEMENT: isize = 1;

/// Dexterity modifier for calculating movement
const DEXTERITY_MOD: f32 = 0.2;

/// Base automata attack damage
const BASE_ATTACK: isize = 1;

/// Strength modifier for calculating attack damage
const STRENGTH_MOD: f32 = 0.25;

/// Base automata attack damage absorb
const BASE_ATTACK_ABSORB: isize = 0;

/// Fortitude modifier for calculating damage absorb
const FORTITUDE_MOD: f32 = 0.25;

/// Stat identifier enum for things that need it
#[derive(Debug, /*Inspectable,*/ Eq, PartialEq, Copy, Clone)]
pub enum StatId {
    /// Constitution - HP
    Constitution,

    /// Dexterity - Movement
    Dexterity,

    /// Strength - Attack
    Strength,

    /// Fortitude - Defense
    Fortitude,
}

impl StatId {
    // TODO: replace this with a From<> impl
    pub fn name(&self) -> Cow<'static, str> {
        match self {
            StatId::Constitution => "Constitution".into(),
            StatId::Dexterity => "Dexterity".into(),
            StatId::Strength => "Strength".into(),
            StatId::Fortitude => "Fortitude".into(),
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
    constitution: Stat,
    dexterity: Stat,
    strength: Stat,
    fortitude: Stat,
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
    pub fn new(constitution: Stat, dexterity: Stat, strength: Stat, fortitude: Stat) -> Self {
        Self {
            constitution,
            dexterity,
            strength,
            fortitude,
        }
    }

    /// Crates a new, randomized stat set
    pub fn random(mut points: isize, random: &mut Random) -> Self {
        let mut stats = Self::default();

        // shuffle the stat types
        let mut buckets = vec![
            StatId::Constitution,
            StatId::Dexterity,
            StatId::Strength,
            StatId::Fortitude,
        ];
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
            StatId::Constitution => self.constitution(),
            StatId::Dexterity => self.dexterity(),
            StatId::Strength => self.strength(),
            StatId::Fortitude => self.fortitude(),
        }
    }

    /// Randomizes a single stat
    pub fn randomize_stat(&mut self, statid: StatId) {
        // TODO: not sure how to handle this,
        // if the stat value changes we need to shuffle other stats around
        match statid {
            StatId::Constitution => {
                // TODO:
            }
            StatId::Dexterity => {
                // TODO:
            }
            StatId::Strength => {
                // TODO:
            }
            StatId::Fortitude => {
                // TODO:
            }
        }
    }

    /// Modifies a stat by amount
    #[inline]
    fn modify(&mut self, statid: StatId, amount: isize) {
        match statid {
            StatId::Constitution => {
                self.constitution.value += amount;
            }
            StatId::Dexterity => {
                self.dexterity.value += amount;
            }
            StatId::Strength => {
                self.strength.value += amount;
            }
            StatId::Fortitude => {
                self.fortitude.value += amount;
            }
        }
    }

    impl_stat!(constitution);

    /// Gets the automata initial health, based on Constitution stat
    #[inline]
    pub fn initial_health(&self) -> usize {
        (BASE_HEALTH + (self.constitution() as f32 * CONSTITUTION_MOD) as isize).max(1) as usize
    }

    impl_stat!(dexterity);

    /// Gets the automata movement, based on Dexterity stat
    #[inline]
    pub fn movement(&self) -> usize {
        (BASE_MOVEMENT + (self.dexterity() as f32 * DEXTERITY_MOD) as isize).max(1) as usize
    }

    impl_stat!(strength);

    /// Gets the automata attack damage, based on Strength stat
    #[inline]
    pub fn attack_damage(&self) -> usize {
        (BASE_ATTACK + (self.strength() as f32 * STRENGTH_MOD) as isize).max(1) as usize
    }

    impl_stat!(fortitude);

    /// Gets the amount of damage absorbed, based on Fortitude stat
    #[inline]
    pub fn absorbed_damage(&self) -> usize {
        (BASE_ATTACK_ABSORB + (self.fortitude() as f32 * FORTITUDE_MOD) as isize).max(1) as usize
    }
}
