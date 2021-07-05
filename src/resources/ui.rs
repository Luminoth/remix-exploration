//! UI resources

use bevy::prelude::*;

/// UI material container resource
pub struct UiMaterials {
    pub none: Handle<ColorMaterial>,
}

/// Button material container resource
pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}
