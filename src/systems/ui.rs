//! UI systems

use bevy::prelude::*;

use crate::components::ui::*;
use crate::resources::ui::*;

/// Generic button update
pub fn update_buttons(
    colors: Res<ButtonColors>,
    mut query: Query<(&Interaction, &mut UiColor, &ButtonHelper), Changed<Interaction>>,
) {
    for (interaction, mut color, helper) in query.iter_mut() {
        if helper.interactable() {
            match *interaction {
                Interaction::Clicked => {
                    *color = colors.pressed;
                }
                Interaction::Hovered => {
                    *color = colors.hovered;
                }
                Interaction::None => {
                    *color = colors.normal;
                }
            }
        }
    }
}
