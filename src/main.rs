//! Exploration of the 2021 Remix Summer Slow Jam

// bevy queries can produce a lot of this
#![allow(clippy::type_complexity)]

mod bundles;
mod components;
mod events;
mod game;
mod plugins;
mod resources;
mod states;
mod systems;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorParams, WorldInspectorPlugin};

use events::remix::*;
use plugins::debug::*;
use resources::automata::*;
use resources::gridworld::*;
use resources::ui::*;
use states::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 576.0;
const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

pub const CELL_X_PIXELS: f32 = WINDOW_WIDTH / GRID_WIDTH as f32;
pub const CELL_Y_PIXELS: f32 = WINDOW_HEIGHT / GRID_HEIGHT as f32;

const STAT_POINTS: isize = 20;

/// Initial setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    let gridworld = GridWorld::new(GRID_WIDTH, GRID_HEIGHT);
    commands.insert_resource(gridworld);

    // resources
    let player_stats = PlayerAutomataStats::new(STAT_POINTS);
    commands.insert_resource(player_stats);

    // assets
    let fonts = Fonts {
        normal: asset_server.load("fonts/FiraSans-Bold.ttf"),
    };
    commands.insert_resource(fonts);

    // materials
    let automata_materials = resources::automata::Materials {
        player_automata: materials.add(Color::TEAL.into()),
        ai_automata: materials.add(Color::ORANGE_RED.into()),
    };
    commands.insert_resource(automata_materials);

    let ui_materials = UiMaterials {
        none: materials.add(Color::NONE.into()),
    };
    commands.insert_resource(ui_materials);

    let button_materials = ButtonMaterials {
        disabled: materials.add(Color::DARK_GRAY.into()),
        normal: materials.add(Color::GRAY.into()),
        hovered: materials.add(Color::WHITE.into()),
        pressed: materials.add(Color::GRAY.into()),
    };
    commands.insert_resource(button_materials);
}

/// Application entry
#[bevy_main]
fn main() {
    std::panic::set_hook(Box::new(|data| {
        error!(%data, "Unexpected panic!");
    }));

    let mut app = App::build();

    // basic bevy
    app.insert_resource(WindowDescriptor {
        title: "Remix - Exploration".to_owned(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        vsync: false,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin);

    // egui
    app.insert_resource(EguiSettings { scale_factor: 0.75 })
        .add_plugin(EguiPlugin);

    // inspector
    app.insert_resource(WorldInspectorParams {
        enabled: false,
        despawnable_entities: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new());

    // plugins
    app.add_plugin(DebugPlugin);

    // game states
    app.add_state(GameState::Intro)
        // intro state systems
        .add_system_set(
            SystemSet::on_enter(GameState::Intro).with_system(states::intro::setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Intro)
                .with_system(systems::ui::update_buttons.system())
                .with_system(states::intro::action_button_handler.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Intro).with_system(states::intro::teardown.system()),
        )
        // remix state systems
        .add_system_set(
            SystemSet::on_enter(GameState::Remix).with_system(states::remix::setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Remix)
                .with_system(systems::ui::update_buttons.system())
                .with_system(states::remix::modifier_button_handler.system())
                .with_system(states::remix::action_button_handler.system())
                .with_system(states::remix::stat_modified_event_handler.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Remix).with_system(states::remix::teardown.system()),
        )
        // game state systems
        .add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(states::game::setup.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(states::game::teardown.system()),
        )
        // game over state systems
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(states::gameover::setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(systems::ui::update_buttons.system())
                .with_system(states::gameover::action_button_handler.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(states::gameover::teardown.system()),
        );
    app.add_event::<StatModifiedEvent>();

    // setup
    app.add_startup_system(setup.system());

    // register components for inspector
    let mut registry = app
        .world_mut()
        .get_resource_or_insert_with(InspectableRegistry::default);

    registry.register::<components::MainCamera>();
    registry.register::<components::UiCamera>();
    registry.register::<components::automata::Automata>();

    app.run();
}
