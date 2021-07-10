//! Remix state events

use crate::game::stats::*;

/// Notifies about a stat being modified in the UI
pub struct StatModifiedEvent(pub StatId);
