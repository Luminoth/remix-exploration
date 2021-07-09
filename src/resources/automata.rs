//! Automata resources

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game::dna::*;
use crate::game::stats::*;
use crate::resources::*;

/// Which stat should be modified
#[derive(Debug, Inspectable, Eq, PartialEq, Copy, Clone)]
pub enum StatModifierType {
    Fortitude,
}

pub trait AutomataStats {
    /// Gets the automata stat set
    fn stats(&self) -> &StatSet;

    /// Modifies a stat by amount
    fn modify(&mut self, r#type: StatModifierType, amount: isize) -> bool;
}

// TODO: this would be less awkward as a macro
fn modify_stat(
    r#type: StatModifierType,
    amount: isize,
    points: &mut isize,
    stats: &mut StatSet,
) -> bool {
    if *points - amount < 0 {
        return false;
    }

    match r#type {
        StatModifierType::Fortitude => {
            if stats.fortitude() + amount < 0 {
                return false;
            }
            stats.set_fortitude(stats.fortitude() + amount);
        }
    }

    *points -= amount;

    true
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

    fn modify(&mut self, r#type: StatModifierType, amount: isize) -> bool {
        modify_stat(r#type, amount, &mut self.points, &mut self.stats)
    }
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
    pub fn value(&self, r#type: StatModifierType) -> isize {
        match r#type {
            StatModifierType::Fortitude => self.stats.fortitude(),
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

    fn modify(&mut self, r#type: StatModifierType, amount: isize) -> bool {
        modify_stat(r#type, amount, &mut self.points, &mut self.stats)
    }
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
