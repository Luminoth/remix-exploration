//! Game resources

use derivative::*;

/// The game stages
#[derive(Debug, Clone, Copy, Eq, PartialEq, Derivative)]
#[derivative(Default)]
pub enum GameStage {
    /// The player is selecting their spawn cell
    #[derivative(Default)]
    CellSelection,

    /// The simulation is running
    Running,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Derivative)]
#[derivative(Default)]
pub enum GameTurn {
    /// The player's turn
    #[derivative(Default)]
    Player,

    /// The AI's turn
    AI,
}

#[derive(Debug, Default)]
pub struct GameRound {
    pub stage: GameStage,
    pub turn: GameTurn,
}
