//! UI components

use bevy_inspector_egui::Inspectable;

/// Action button tag
#[derive(Debug, Inspectable, Default)]
pub struct ActionButton;

/// Stat modifier button tag
#[derive(Debug, Inspectable, Default)]
pub struct StatModifierButton {
    pub modifier: isize,
}
