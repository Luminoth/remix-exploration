//! Automata resources

use bevy::prelude::*;

use crate::game::dna::*;
use crate::game::stats::*;
use crate::resources::*;

/// General automata stat trait
pub trait AutomataStats {
    /// Gets the automata stat set
    fn stats(&self) -> &StatSet;

    /// Modifies a stat by amount
    fn modify(&mut self, statid: StatId, amount: isize) -> bool;
}

macro_rules! impl_modify_stats {
    () => {
        fn modify(&mut self, statid: StatId, amount: isize) -> bool {
            if self.points - amount < 0 {
                return false;
            }

            match statid {
                StatId::Fortitude => {
                    if self.stats.fortitude() + amount < 0 {
                        return false;
                    }
                    self.stats.set_fortitude(self.stats.fortitude() + amount);
                }
            }

            self.points -= amount;

            true
        }
    };
}

/// Player automata stats
#[derive(Debug, Default, Clone, Copy)]
pub struct PlayerAutomataStats {
    points: isize,

    pub stats: StatSet,
}

impl AutomataStats for PlayerAutomataStats {
    fn stats(&self) -> &StatSet {
        &self.stats
    }

    impl_modify_stats!();
}

impl PlayerAutomataStats {
    /// Creates new player automata stats
    pub fn new(points: isize) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }

    /// Gets the number of unspent points
    pub fn points(&self) -> isize {
        self.points
    }

    /// Gets the value of a stat
    pub fn value(&self, statid: StatId) -> isize {
        match statid {
            StatId::Fortitude => self.stats.fortitude(),
        }
    }
}

/// AI automata stats
#[derive(Debug, Default, Clone, Copy)]
pub struct AIAutomataStats {
    points: isize,

    pub stats: StatSet,
}

impl AutomataStats for AIAutomataStats {
    fn stats(&self) -> &StatSet {
        &self.stats
    }

    impl_modify_stats!();
}

impl AIAutomataStats {
    /// Creates new, randomized AI automata stats
    pub fn new(points: isize, random: &mut Random) -> Self {
        Self {
            points,
            stats: StatSet::random(points, random),
        }
    }
}

/// AI automata population
#[derive(Debug)]
pub struct AIAutomataPopulation {
    mutation_rate: f64,

    population: Vec<AIAutomataStats>,
    mating_pool: Vec<DNA>,
}

impl AIAutomataPopulation {
    /// Creates a new randomized population
    pub fn new(mutation_rate: f64, rounds: usize, points: isize, random: &mut Random) -> Self {
        let mut population = Vec::with_capacity(rounds);
        for _ in 0..population.capacity() {
            population.push(AIAutomataStats::new(points, random));
        }

        Self {
            mutation_rate,
            population,
            mating_pool: vec![],
        }
    }
}

/// Materials container resource
pub struct Materials {
    pub player_automata: Handle<ColorMaterial>,
    pub ai_automata: Handle<ColorMaterial>,
}
