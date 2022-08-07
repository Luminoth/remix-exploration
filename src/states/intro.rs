//! Intro state systems

use bevy::prelude::*;

use super::*;

use crate::components::ui::*;
use crate::components::*;
use crate::game::dna::MUTATION_RATE;
use crate::resources::automata::*;
use crate::resources::game::*;
use crate::resources::gridworld::*;
use crate::resources::ui::*;
use crate::resources::*;

/// Intro setup
pub fn setup(
    mut commands: Commands,
    mut random: ResMut<Random>,
    button_colors: Res<ButtonColors>,
    fonts: Res<Fonts>,
) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(UiCamera)
        .insert(Name::new("UI Camera"));

    // world
    let gridworld = GridWorld::new(crate::GRID_WIDTH, crate::GRID_HEIGHT);
    commands.insert_resource(gridworld);

    // player automata stats
    let player_stats = PlayerAutomataStats::new(crate::STAT_POINTS);
    commands.insert_resource(player_stats);

    // AI automata population
    let ai_population = AIAutomataPopulation::new(
        MUTATION_RATE,
        crate::ROUNDS,
        crate::STAT_POINTS,
        &mut random,
    );
    commands.insert_resource(ai_population);

    // round
    commands.insert_resource(GameRound::default());

    // UI
    let root = spawn_ui_root(&mut commands);
    commands.entity(root).with_children(|parent| {
        spawn_header(parent, &fonts, "Remix Exploration");

        spawn_spacer(parent);

        spawn_ok_action(parent, &button_colors, &fonts, "Play", true);
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
            state.set(GameState::Remix).unwrap();
        }
    }
}

/// Intro teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
