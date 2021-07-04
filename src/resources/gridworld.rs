//! Gridworld resources

/// A GridWorld cell
#[derive(Debug, Default)]
pub struct Cell;

/// The grid... world
#[derive(Debug, Default)]
pub struct GridWorld {
    pub cells: Vec<Cell>,
}

impl GridWorld {
    /// Creates a new grid world
    pub fn new(x: usize, y: usize) -> Self {
        let size = x * y;
        let mut cells = Vec::with_capacity(size);
        for _ in 0..size {
            cells.push(Cell::default());
        }

        Self { cells }
    }
}
