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
    constitution: f32,
    dexterity: f32,
    strength: f32,
    fortitude: f32,
    aggression: f32,
    intellect: f32,
}

/// Genetic algorithm DNA
#[derive(Debug, Inspectable)]
pub struct Dna {
    genes: StatSet,

    fitness: StatSetFitness,
    rounds: usize,
    points: isize,
}

impl Dna {
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
        self.fitness.constitution = health.pow(2) as f32 / stats.initial_health().pow(2) as f32;
        //self.fitness.dexterity = ???
        //self.fitness.strength = ???
        //self.fitenss.fortitude = ???
        //self.fitness.aggression = ???
        //self.fitness.intellect = ???
    }

    /// Create a child through gentics crossover
    fn crossover(&self, partner: &Dna, method: CrossoverMethod, random: &mut Random) -> Dna {
        let mut child = Dna::new(self.rounds, self.points, random);

        match method {
            CrossoverMethod::Midpoint => {
                child.genes.set_constitution(self.genes.constitution());
                child.genes.set_dexterity(self.genes.dexterity());
                child.genes.set_strength(self.genes.strength());
                child.genes.set_fortitude(partner.genes.fortitude());
                child.genes.set_aggression(partner.genes.aggression());
                child.genes.set_intellect(partner.genes.intellect());
            }
            CrossoverMethod::Coin => {
                child
                    .genes
                    .set_constitution(if random.random_range(0..=1) == 0 {
                        self.genes.constitution()
                    } else {
                        partner.genes.constitution()
                    });

                child
                    .genes
                    .set_dexterity(if random.random_range(0..=1) == 0 {
                        self.genes.dexterity()
                    } else {
                        partner.genes.dexterity()
                    });

                child
                    .genes
                    .set_strength(if random.random_range(0..=1) == 0 {
                        self.genes.strength()
                    } else {
                        partner.genes.strength()
                    });

                child
                    .genes
                    .set_fortitude(if random.random_range(0..=1) == 0 {
                        self.genes.fortitude()
                    } else {
                        partner.genes.fortitude()
                    });

                child
                    .genes
                    .set_aggression(if random.random_range(0..=1) == 0 {
                        self.genes.aggression()
                    } else {
                        partner.genes.aggression()
                    });

                child
                    .genes
                    .set_intellect(if random.random_range(0..=1) == 0 {
                        self.genes.intellect()
                    } else {
                        partner.genes.intellect()
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
        self.mutate_stat(mutation_rate, random, StatId::Constitution);
        self.mutate_stat(mutation_rate, random, StatId::Dexterity);
        self.mutate_stat(mutation_rate, random, StatId::Strength);
        self.mutate_stat(mutation_rate, random, StatId::Fortitude);
        self.mutate_stat(mutation_rate, random, StatId::Aggression);
        self.mutate_stat(mutation_rate, random, StatId::Intellect);
    }
}
