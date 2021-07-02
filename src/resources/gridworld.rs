//! Gridworld resources

/// A cell in the grid world
#[derive(Debug, Default)]
pub struct Cell {
    // TODO: what do we need here?
}

/// The grid... world
#[derive(Debug, Default)]
pub struct GridWorld {
    pub cells: Vec<Cell>,
}

impl GridWorld {
    pub fn new(x: usize, y: usize) -> Self {
        let size = x * y;
        let mut cells = Vec::with_capacity(size);
        for _ in 0..size {
            cells.push(Cell::default());
        }

        Self { cells }
    }
}
