//! Game events

/// Notifies about the game starting
pub struct GameStartEvent;

/// Notifies about an automata health change
pub struct HealthChangedEvent {
    /// Was it the player's health that changed?
    pub player: bool,

    /// The new health value
    pub value: isize,
}
