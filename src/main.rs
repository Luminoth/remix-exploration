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
mod util;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorParams, WorldInspectorPlugin};

use plugins::debug::*;
use plugins::states::*;
use plugins::ui::*;
use resources::automata::*;
use resources::gridworld::*;
use resources::ui::*;
use resources::*;
use states::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;
//const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

pub const CELL_WIDTH: usize = 1;
pub const CELL_HEIGHT: usize = 1;

pub const STAT_POINTS: isize = 20;

/// Initial setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    let random = Random::default();

    let gridworld = GridWorld::new(GRID_WIDTH, GRID_HEIGHT);
    commands.insert_resource(gridworld);

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

    commands.insert_resource(random);
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
    app.add_plugin(DebugPlugin)
        .add_plugin(UIPlugin)
        .add_plugins(StatesPlugins);

    // initial game state
    app.add_state(GameState::Intro);

    // main setup
    app.add_startup_system(setup.system());

    // register components for inspector
    let mut registry = app
        .world_mut()
        .get_resource_or_insert_with(InspectableRegistry::default);

    registry.register::<components::MainCamera>();
    registry.register::<components::UiCamera>();
    registry.register::<components::automata::Automata>();
    registry.register::<components::automata::AutomataStats>();
    registry.register::<components::automata::PlayerAutomata>();
    registry.register::<components::automata::AIAutomata>();
    registry.register::<components::gridworld::GridWorldCell>();
    registry.register::<components::ui::ButtonHelper>();
    registry.register::<components::ui::ActionButton>();
    registry.register::<components::ui::StatModifierButton>();
    registry.register::<components::ui::PointsText>();
    registry.register::<components::ui::StatModifierText>();
    registry.register::<game::stats::Stat>();
    registry.register::<game::stats::StatSet>();
    registry.register::<resources::automata::StatModifierType>();

    app.run();
}
