//! UI components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game::stats::*;

/// Button helper
#[derive(Debug, Inspectable, Default)]
pub struct ButtonHelper {
    pub interactable: bool,
}

/// Action button
#[derive(Debug, Inspectable, Default)]
pub struct ActionButton;

/// Cell selection button
#[derive(Debug, Inspectable, Default)]
pub struct CellSelectionButton {
    // TODO: this should be UVec2, but those aren't Inspectable
    pub cell: Vec2,
}

/// Stat modifier button
#[derive(Debug, Inspectable)]
pub struct StatModifierButton {
    pub statid: StatId,
    pub modifier: isize,
}

/// Points text tag
#[derive(Debug, Inspectable)]
pub struct PointsText;

/// Stat modifier text
#[derive(Debug, Inspectable)]
pub struct StatModifierText {
    pub statid: StatId,
}

/// Cell selection tag
pub struct CellSelection;

/// HUD tag
pub struct HUD;
