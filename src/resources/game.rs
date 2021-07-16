//! Game resources

/// The game stages
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameStage {
    /// The player is selecting their spawn cell
    CellSelection,

    /// The simulation is running
    Running,
}
