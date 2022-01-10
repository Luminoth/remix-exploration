//! UI components

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::game::stats::*;
use crate::resources::ui::*;

/// Button helper
#[derive(Debug, Default, Component, Inspectable)]
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
        color: &mut UiColor,
        colors: &ButtonColors,
    ) {
        self.interactable = interactable;
        *color = if self.interactable {
            colors.normal
        } else {
            colors.disabled
        };
    }
}

/// Action button
#[derive(Debug, Default, Component, Inspectable)]
pub struct ActionButton;

/// Cell selection button
#[derive(Debug, Default, Component, Inspectable)]
pub struct CellSelectionButton {
    // TODO: this should be UVec2, but those aren't Inspectable
    pub cell: Vec2,
}

/// Stat modifier button
#[derive(Debug, Component, Inspectable)]
pub struct StatModifierButton {
    pub statid: StatId,
    pub modifier: isize,
}

/// Points text tag
#[derive(Debug, Component, Inspectable)]
pub struct PointsText;

/// Stat modifier text
#[derive(Debug, Component, Inspectable)]
pub struct StatModifierText {
    pub statid: StatId,
}

/// Round text
#[derive(Debug, Component, Inspectable)]
pub struct RoundText;

/// Automata health text
#[derive(Debug, Component, Inspectable)]
pub struct AutomataHealthText {
    pub player: bool,
}

/// Cell selection tag
#[derive(Component)]
pub struct CellSelection;

/// HUD tag
#[derive(Component)]
pub struct Hud;
