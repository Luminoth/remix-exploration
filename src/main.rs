//! Exploration of the 2021 Remix Summer Slow Jam

// bevy queries can produce a lot of these
#![allow(clippy::type_complexity)]
// bevy systems can produce a lot of this
#![allow(clippy::too_many_arguments)]
// TODO: remove this
#![allow(dead_code)]

// TODO: enable this
//#![deny(warnings)]

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
use bevy::window::PresentMode;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};

use plugins::debug::*;
use plugins::states::*;
use plugins::ui::*;
use resources::ui::*;
use resources::*;
use states::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;
const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

pub const TOP_MARGIN: f32 = 1.0;
pub const LEFT_MARGIN: f32 = 0.0;

const BASE_CELL_WIDTH: f32 = 1.0;
const BASE_CELL_HEIGHT: f32 = 1.0;

// TODO: this is a mess...
pub const CELL_X_SCALE: f32 =
    ((GRID_WIDTH as f32 * BASE_CELL_WIDTH) - LEFT_MARGIN) / (GRID_WIDTH as f32 * BASE_CELL_WIDTH);
pub const CELL_Y_SCALE: f32 = ((GRID_HEIGHT as f32 * BASE_CELL_HEIGHT) - TOP_MARGIN)
    / (GRID_HEIGHT as f32 * BASE_CELL_HEIGHT);

pub const CELL_WIDTH: f32 = BASE_CELL_WIDTH * CELL_X_SCALE * ASPECT_RATIO;
pub const CELL_HEIGHT: f32 = BASE_CELL_HEIGHT * CELL_Y_SCALE;

pub const ROUNDS: usize = 10;
pub const STAT_POINTS: isize = 50;

/// Initial setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    let random = Random::default();

    // assets
    let fonts = Fonts {
        normal: asset_server.load("fonts/FiraSans-Bold.ttf"),
    };
    commands.insert_resource(fonts);

    // materials
    let automata_materials = resources::automata::AutomataColors {
        cell: Color::BISQUE,
        player_automata: Color::TEAL,
        ai_automata: Color::ORANGE_RED,
    };
    commands.insert_resource(automata_materials);

    let button_materials = ButtonColors {
        disabled: Color::DARK_GRAY.into(),
        normal: Color::GRAY.into(),
        hovered: Color::WHITE.into(),
        pressed: Color::GRAY.into(),
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

    let mut app = App::new();

    // basic bevy
    app.insert_resource(WindowDescriptor {
        title: "Remix - Exploration".to_owned(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        present_mode: PresentMode::Immediate,
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
    app.insert_resource(EguiSettings {
        scale_factor: 0.75,
        ..Default::default()
    })
    .add_plugin(EguiPlugin);

    // inspector
    app.insert_resource(WorldInspectorParams {
        enabled: false,
        despawnable_entities: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new())
    // inspectable types
    .register_inspectable::<components::MainCamera>()
    .register_inspectable::<components::UiCamera>()
    .register_inspectable::<components::automata::Automata>()
    .register_inspectable::<components::automata::PlayerAutomata>()
    .register_inspectable::<components::automata::AIAutomata>()
    .register_inspectable::<components::gridworld::GridWorldCell>()
    .register_inspectable::<components::ui::ButtonHelper>()
    .register_inspectable::<components::ui::ActionButton>()
    .register_inspectable::<components::ui::StatModifierButton>()
    .register_inspectable::<components::ui::PointsText>()
    .register_inspectable::<components::ui::StatModifierText>()
    .register_inspectable::<game::stats::StatId>()
    .register_inspectable::<game::stats::Stat>()
    .register_inspectable::<game::stats::StatSet>()
    .register_inspectable::<game::dna::Dna>();

    // plugins
    app.add_plugin(DebugPlugin)
        .add_plugin(UIPlugin)
        .add_plugins(StatesPlugins);

    // initial game state
    app.add_state(GameState::Intro);

    // main setup
    app.add_startup_system(setup);

    app.run();
}
