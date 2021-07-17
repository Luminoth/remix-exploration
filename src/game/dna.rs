//! AI dna

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::resources::*;

use super::stats::*;

pub const MUTATION_RATE: f64 = 0.01; // 1% chance to mutate

#[derive(Debug, Copy, Clone)]
enum CrossoverMethod {
    #[allow(dead_code)]
    Midpoint,

    #[allow(dead_code)]
    Coin,
}

#[derive(Debug, Inspectable, Default)]
struct StatSetFitness {
    fortitude: f32,
    dexterity: f32,
}

/// Genetic algorithm DNA
#[derive(Debug, Inspectable)]
pub struct DNA {
    genes: StatSet,

    fitness: StatSetFitness,
    rounds: usize,
    points: isize,
}

impl DNA {
    /// Creates a new, randomized DNA
    pub fn new(rounds: usize, points: isize, random: &mut Random) -> Self {
        Self {
            genes: StatSet::random(points, random),
            fitness: StatSetFitness::default(),
            rounds,
            points,
        }
    }

    /// Adjust genetic fitness based on round results
    pub fn fitness(&mut self, stats: &StatSet, health: usize) {
        self.fitness.fortitude = health.pow(2) as f32 / stats.initial_health().pow(2) as f32;
        //self.fitness.dexterity = ???
    }

    /// Create a child through gentics crossover
    fn crossover(&self, partner: &DNA, method: CrossoverMethod, random: &mut Random) -> DNA {
        let mut child = DNA::new(self.rounds, self.points, random);

        match method {
            CrossoverMethod::Midpoint => {
                child.genes.set_fortitude(self.genes.fortitude());
                child.genes.set_dexterity(partner.genes.dexterity());
            }
            CrossoverMethod::Coin => {
                child
                    .genes
                    .set_fortitude(if random.random_range(0..=1) == 0 {
                        self.genes.fortitude()
                    } else {
                        partner.genes.fortitude()
                    });

                child
                    .genes
                    .set_dexterity(if random.random_range(0..=1) == 0 {
                        self.genes.dexterity()
                    } else {
                        partner.genes.dexterity()
                    });
            }
        }

        child
    }

    /// Randomly mutate a specific stat at the given mutation rate
    fn mutate_stat(&mut self, mutation_rate: f64, random: &mut Random, statid: StatId) {
        if random.random() < mutation_rate {
            debug!("{} mutation!", statid.name());
            self.genes.randomize_stat(statid);
        }
    }

    /// Mutate random genes at the given mutation rate
    fn mutate(&mut self, mutation_rate: f64, random: &mut Random) {
        self.mutate_stat(mutation_rate, random, StatId::Fortitude);
        self.mutate_stat(mutation_rate, random, StatId::Dexterity);
    }
}
