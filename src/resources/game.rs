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
pub enum GameAction {
    /// The player's move action
    #[derivative(Default)]
    PlayerMove,

    /// The player's attack action
    PlayerAttack,

    /// The AI's move action
    AIMove,

    /// The AI's attack action
    AIAttack,
}

#[derive(Debug, Default)]
pub struct GameRound {
    pub round: usize,
    pub stage: GameStage,
    pub action: GameAction,
}

impl GameRound {
    pub fn reset(&mut self) {
        self.stage = GameStage::default();
        self.action = GameAction::default();
    }
}
