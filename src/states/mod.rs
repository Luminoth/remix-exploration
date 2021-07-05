//! Game states

pub mod game;
pub mod gameover;
pub mod intro;
pub mod remix;

/// The game state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    /// Intro state - Explain how to play the game
    Intro,

    /// Remix state - Assign attribute points
    Remix,

    /// Game state - Run the simulation
    Game,

    /// Game over state - All rounds complete, show results
    GameOver,
}
