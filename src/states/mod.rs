//! Game states

pub mod game;
pub mod gameover;
pub mod intro;
pub mod remix;

use bevy::prelude::*;

use crate::bundles::ui::*;
use crate::components::ui::*;
use crate::resources::ui::*;

/// The game state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    /// Intro state - Explain how to play the game
    Intro,

    /// Remix state - Assign attribute points
    Remix,

    /// Game state - Run the simulation
    Game,

    /// Game over state - All rounds complete, show results
    GameOver,
}

// TODO: move these UI helpers somewhere else

/// Spawn a UI root node
fn spawn_ui_root(commands: &mut Commands) -> Entity {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("UI Root"))
        .id()
}

/// Spawn a UI spacer
fn spawn_spacer(parent: &mut ChildBuilder) {
    parent.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            flex_grow: 1.0,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    });
}

/// Spawn a UI header
fn spawn_header(parent: &mut ChildBuilder, fonts: &Fonts, text: impl Into<String>) {
    parent.spawn_bundle(TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        text: Text::with_section(
            text,
            TextStyle {
                font: fonts.normal.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    });
}

/// Spawn a UI OK action row
fn spawn_ok_action(
    parent: &mut ChildBuilder,
    button_colors: &ButtonColors,
    fonts: &Fonts,
    text: impl Into<String>,
    interactable: bool,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Actions"))
        .with_children(|parent| {
            parent
                .spawn_bundle(ActionButtonBundle {
                    button: ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: button_colors.normal,
                        ..Default::default()
                    },
                    helper: ButtonHelper::new(interactable),
                    action_button: ActionButton,
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            text,
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}
