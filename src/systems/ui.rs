//! UI systems

use bevy::prelude::*;

use crate::components::ui::*;
use crate::resources::ui::*;

/// Generic button update
pub fn update_buttons(
    materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &ButtonHelper),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut material, helper) in query.iter_mut() {
        if helper.interactable {
            match *interaction {
                Interaction::Clicked => {
                    *material = materials.pressed.clone();
                }
                Interaction::Hovered => {
                    *material = materials.hovered.clone();
                }
                Interaction::None => {
                    *material = materials.normal.clone();
                }
            }
        }
    }
}
