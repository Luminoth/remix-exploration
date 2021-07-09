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
struct StatFitness {
    fortitude: f32,
}

/// Genetic algorithm DNA
#[derive(Debug, Inspectable)]
pub struct DNA {
    genes: Vec<StatSet>,

    fitness: StatFitness,
    rounds: usize,
    points: isize,
}

impl DNA {
    /// Creates a new, randomized DNA
    pub fn new(rounds: usize, points: isize, random: &mut Random) -> Self {
        let mut genes = Vec::with_capacity(rounds);
        for _ in 0..genes.capacity() {
            genes.push(StatSet::random(points, random));
        }

        Self {
            genes,
            fitness: StatFitness::default(),
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
                let midpoint = random.random_range(0..self.genes.len());
                for i in 0..self.genes.len() {
                    child.genes[i] = if i > midpoint {
                        self.genes[i]
                    } else {
                        partner.genes[i]
                    };
                }
            }
            CrossoverMethod::Coin => {
                for i in 0..self.genes.len() {
                    let coin = random.random_range(0..=1);
                    child.genes[i] = if coin == 0 {
                        self.genes[i]
                    } else {
                        partner.genes[i]
                    };
                }
            }
        }

        child
    }

    /// Mutate random genes at the given mutation rate
    fn mutate(&mut self, mutation_rate: f64, random: &mut Random) {
        for i in 0..self.genes.len() {
            if random.random() < mutation_rate {
                debug!("mutation!");
                // TODO:
            }
        }
    }
}
