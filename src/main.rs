//! Exploration of the 2021 Remix Summer Slow Jam

// bevy queries can produce a lot of this
#![allow(clippy::type_complexity)]

mod components;
mod game;
mod resources;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorParams, WorldInspectorPlugin};

use resources::gridworld::GridWorld;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 576.0;
const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

/// Initial setup
fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    _asset_server.watch_for_changes().unwrap();

    let gridworld = GridWorld::new(GRID_WIDTH, GRID_HEIGHT);

    commands.insert_resource(gridworld);
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
    app.insert_resource(EguiSettings { scale_factor: 0.8 })
        .add_plugin(EguiPlugin);

    // inspector
    app.insert_resource(WorldInspectorParams {
        enabled: false,
        despawnable_entities: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new());

    // plugins
    // TODO:

    // game states
    // TODO:

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
