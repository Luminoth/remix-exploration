//! Game states plugin

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::events::game::*;
use crate::events::remix::*;
use crate::states;
use crate::states::*;

/// Game states plugin group
pub struct StatesPlugins;

impl PluginGroup for StatesPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(IntroStatePlugin)
            .add(RemixStatePlugin)
            .add(GameStatePlugin)
            .add(GameOverStatePlugin);
    }
}

/// Intro state plugin
struct IntroStatePlugin;

impl Plugin for IntroStatePlugin {
    fn build(&self, app: &mut App) {
        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Intro).with_system(states::intro::setup))
            .add_system_set(
                SystemSet::on_update(GameState::Intro)
                    .with_system(states::intro::action_button_handler),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Intro).with_system(states::intro::teardown),
            );
    }
}

/// Remix state plugin
struct RemixStatePlugin;

impl Plugin for RemixStatePlugin {
    fn build(&self, app: &mut App) {
        // events
        app.add_event::<StatModifiedEvent>();

        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Remix).with_system(states::remix::setup))
            .add_system_set(
                SystemSet::on_update(GameState::Remix)
                    .with_system(states::remix::modifier_button_handler)
                    .with_system(states::remix::action_button_handler)
                    .with_system(states::remix::stat_modified_event_handler),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Remix).with_system(states::remix::teardown),
            );
    }
}

/// Game state plugin
struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        // events
        app.add_event::<GameStartEvent>()
            .add_event::<HealthChangedEvent>();

        // systems
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(states::game::setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(states::game::cell_selection_button_handler)
                    .with_system(states::game::game_start_event_handler)
                    .with_system(states::game::health_changed_event_handler)
                    .with_system(states::game::automata_action),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(states::game::teardown),
            );
    }
}

/// Game over state plugin
struct GameOverStatePlugin;

impl Plugin for GameOverStatePlugin {
    fn build(&self, app: &mut App) {
        // systems
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(states::gameover::setup),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(states::gameover::action_button_handler),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(states::gameover::teardown),
        );
    }
}
