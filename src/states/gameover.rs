//! Game over state systems

use bevy::prelude::*;

use super::*;

use crate::components::ui::*;
use crate::components::*;
use crate::resources::automata::*;
use crate::resources::game::*;
use crate::resources::ui::*;

/// Game over setup
pub fn setup(mut commands: Commands, button_colors: Res<ButtonColors>, fonts: Res<Fonts>) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(UiCamera)
        .insert(Name::new("UI Camera"));

    // UI
    let root = spawn_ui_root(&mut commands);
    commands.entity(root).with_children(|parent| {
        spawn_header(parent, &fonts, "Game Over");

        spawn_spacer(parent);

        spawn_ok_action(parent, &button_colors, &fonts, "Continue", true);
    });
}

/// Action button handler
pub fn action_button_handler(
    mut action_query: Query<
        (&Interaction, &ButtonHelper),
        (Changed<Interaction>, With<ActionButton>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok((interaction, helper)) = action_query.get_single_mut() {
        if helper.interactable() && *interaction == Interaction::Clicked {
            state.set(GameState::Intro).unwrap();
        }
    }
}

/// Game over teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<PlayerAutomataStats>();
    commands.remove_resource::<AIAutomataPopulation>();
    commands.remove_resource::<GameRound>();

    commands.remove_resource::<ClearColor>();
}
