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
    }

    /// Create a child through gentics crossover
    fn crossover(&self, partner: &DNA, method: CrossoverMethod, random: &mut Random) -> DNA {
        let mut child = DNA::new(self.rounds, self.points, random);

        match method {
            CrossoverMethod::Midpoint => {
                child.genes.set_fortitude(self.genes.fortitude());
            }
            CrossoverMethod::Coin => {
                child
                    .genes
                    .set_fortitude(if random.random_range(0..=1) == 0 {
                        self.genes.fortitude()
                    } else {
                        partner.genes.fortitude()
                    });
            }
        }

        child
    }

    /// Mutate random genes at the given mutation rate
    fn mutate(&mut self, mutation_rate: f64, random: &mut Random) {
        if random.random() < mutation_rate {
            debug!("fortitude mutation!");
            self.genes.randomize_stat(StatId::Fortitude);
        }
    }
}
