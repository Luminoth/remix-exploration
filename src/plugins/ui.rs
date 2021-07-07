//! UI plugin

use bevy::prelude::*;

use crate::systems::ui::*;

/// UI plugin
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // systems
        app.add_system(update_buttons.system());
    }
}
