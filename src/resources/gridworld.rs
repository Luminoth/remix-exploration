//! Gridworld resources

use bevy::prelude::*;

/// A GridWorld cell
#[derive(Debug)]
pub struct Cell(pub UVec2);

/// The grid... world
#[derive(Debug, Default)]
pub struct GridWorld {
    pub cells: Vec<Cell>,
}

impl GridWorld {
    /// Creates a new grid world
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut cells = Vec::with_capacity(size);

        for y in 0..height {
            for x in 0..width {
                cells.push(Cell(UVec2::new(x as u32, y as u32)));
            }
        }

        Self { cells }
    }
}
