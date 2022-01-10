//! UI resources

use bevy::prelude::*;

pub struct Fonts {
    pub normal: Handle<Font>,
}

/// Button colors container resource
pub struct ButtonColors {
    pub disabled: UiColor,
    pub normal: UiColor,
    pub hovered: UiColor,
    pub pressed: UiColor,
}
