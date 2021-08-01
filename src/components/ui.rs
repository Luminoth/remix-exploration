//! UI components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game::stats::*;
use crate::resources::ui::*;

/// Button helper
#[derive(Debug, Inspectable, Default)]
pub struct ButtonHelper {
    interactable: bool,
}

impl ButtonHelper {
    pub fn new(interactable: bool) -> Self {
        Self { interactable }
    }

    #[inline]
    pub fn interactable(&self) -> bool {
        self.interactable
    }

    pub fn set_interactable(
        &mut self,
        interactable: bool,
        material: &mut Handle<ColorMaterial>,
        materials: &ButtonMaterials,
    ) {
        self.interactable = interactable;
        *material = if self.interactable {
            materials.normal.clone()
        } else {
            materials.disabled.clone()
        };
    }
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
#[derive(Debug /*, Inspectable*/)]
pub struct StatModifierButton {
    pub statid: StatId,
    pub modifier: isize,
}

/// Points text tag
#[derive(Debug, Inspectable)]
pub struct PointsText;

/// Stat modifier text
#[derive(Debug /*, Inspectable*/)]
pub struct StatModifierText {
    pub statid: StatId,
}

/// Round text
#[derive(Debug /*, Inspectable*/)]
pub struct RoundText;

/// Automata health text
#[derive(Debug /*, Inspectable*/)]
pub struct AutomataHealthText {
    pub player: bool,
}

/// Cell selection tag
pub struct CellSelection;

/// HUD tag
pub struct Hud;
