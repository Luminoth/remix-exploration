//! UI bundles

use bevy::prelude::*;

use crate::components::ui::*;

/// Action button component bundle
#[derive(Bundle)]
pub struct ActionButtonBundle {
    pub helper: ButtonHelper,
    pub action_button: ActionButton,

    #[bundle]
    pub button: ButtonBundle,
}

/// Stat modifier button component bundle
#[derive(Bundle)]
pub struct StatModifierButtonBundle {
    pub helper: ButtonHelper,
    pub modifier_button: StatModifierButton,

    #[bundle]
    pub button: ButtonBundle,
}

/// Points text component bundle
#[derive(Bundle)]
pub struct PointsTextBundle {
    pub points_text: PointsText,

    #[bundle]
    pub text: TextBundle,
}

/// Stat Modifier text component bundle
#[derive(Bundle)]
pub struct StatModifierTextBundle {
    pub modifier_text: StatModifierText,

    #[bundle]
    pub text: TextBundle,
}
