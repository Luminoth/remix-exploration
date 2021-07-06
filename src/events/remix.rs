//! Remix state events

use crate::resources::automata::*;

/// Notifies about a stat being modified in the UI
pub struct StatModifiedEvent(pub StatModifierType);
