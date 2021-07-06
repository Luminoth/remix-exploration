//! ECS resources

pub mod automata;
pub mod debug;
pub mod gridworld;
pub mod ui;

use bevy::prelude::*;
use num_traits::Float;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::*;
use rand_distr::{Normal, StandardNormal};

use crate::util::clampf;

/// Random wrapper
pub struct Random {
    // TODO: would SmallRng be better here? we don't need a secure rng
    random: StdRng,
}

impl Default for Random {
    /// Constructs a default random from system entropy
    fn default() -> Self {
        Self {
            random: StdRng::from_entropy(),
        }
    }
}

impl Random {
    /// Constructs a new random from a seed
    #[allow(dead_code)]
    pub fn new(seed: u64) -> Self {
        Self {
            random: StdRng::seed_from_u64(seed),
        }
    }

    #[allow(dead_code)]
    pub fn shuffle<T>(&mut self, v: &mut Vec<T>) {
        v.shuffle(&mut self.random);
    }

    /// Coin returns a random boolean
    #[allow(dead_code)]
    pub fn coin(&mut self) -> bool {
        self.random_range(0..=1) == 1
    }

    /// Dice returns a random in the range [1..faces]
    #[allow(dead_code)]
    pub fn dice(&mut self, faces: usize) -> usize {
        self.random_range(1..=faces)
    }

    /// Generates a uniform random value in the range [0..1)
    #[allow(dead_code)]
    pub fn random(&mut self) -> f64 {
        self.random_range(0.0..1.0)
    }

    /// Generates a uniform random value in the given range
    #[allow(dead_code)]
    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.random.gen_range(range)
    }

    /// Generates a uniform random vector in the range ([0..1], [0..1])
    #[allow(dead_code)]
    pub fn vec2(&mut self) -> Vec2 {
        self.vec2_range(0.0..=1.0, 0.0..=1.0)
    }

    /// Generates a uniform random vector in the given range
    #[allow(dead_code)]
    pub fn vec2_range<R>(&mut self, xrange: R, yrange: R) -> Vec2
    where
        R: SampleRange<f32>,
    {
        Vec2::new(self.random_range(xrange), self.random_range(yrange))
    }

    /// Generates a uniform random direction vector
    #[allow(dead_code)]
    pub fn direction(&mut self) -> Vec2 {
        let theta = self.random() as f32 * std::f32::consts::PI * 2.0;
        Vec2::new(theta.cos(), theta.sin())
    }

    /// Generates a random value with the given normal distribution
    #[allow(dead_code)]
    pub fn normal<F>(&mut self, mean: F, std_dev: F) -> F
    where
        F: Float,
        StandardNormal: Distribution<F>,
    {
        Normal::new(mean, std_dev).unwrap().sample(&mut self.random)
    }

    /// Generates a random value with the given normal distribution
    /// Clamped to the given min / max
    #[allow(dead_code)]
    pub fn normal_clamped<F>(&mut self, mean: F, std_dev: F, min: F, max: F) -> F
    where
        F: Float,
        StandardNormal: Distribution<F>,
    {
        clampf(
            Normal::new(mean, std_dev).unwrap().sample(&mut self.random),
            min,
            max,
        )
    }
}
