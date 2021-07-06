//! UI resources

use bevy::prelude::*;

/// UI material container resource
pub struct UiMaterials {
    pub none: Handle<ColorMaterial>,
}

pub struct Fonts {
    pub normal: Handle<Font>,
}

/// Button material container resource
pub struct ButtonMaterials {
    pub disabled: Handle<ColorMaterial>,
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}
