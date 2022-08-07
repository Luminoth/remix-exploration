//! Game state systems

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use super::*;

use crate::bundles::ui::*;
use crate::bundles::*;
use crate::components::automata::*;
use crate::components::gridworld::*;
use crate::components::ui::*;
use crate::components::*;
use crate::events::game::*;
use crate::resources::automata::*;
use crate::resources::game::*;
use crate::resources::gridworld::*;
use crate::resources::ui::*;
use crate::util::*;
use crate::*;

const GRID_BUTTON_WIDTH: f32 = crate::WINDOW_WIDTH / crate::GRID_WIDTH as f32;
const GRID_BUTTON_HEIGHT: f32 = crate::WINDOW_HEIGHT / crate::GRID_HEIGHT as f32;

/// Spawn a cell selection row
fn spawn_cell_selection_row(parent: &mut ChildBuilder, button_colors: &ButtonColors, row: usize) {
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
        .insert(Name::new("Cell Selection Row"))
        .with_children(|parent| {
            for column in 0..crate::GRID_WIDTH {
                parent.spawn_bundle(CellSelectionButtonBundle {
                    button: ButtonBundle {
                        style: Style {
                            size: Size::new(
                                Val::Px(GRID_BUTTON_WIDTH),
                                Val::Px(GRID_BUTTON_HEIGHT),
                            ),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: button_colors.normal,
                        ..Default::default()
                    },
                    helper: ButtonHelper::new(true),
                    cell_selection_button: CellSelectionButton {
                        cell: Vec2::new(column as f32, row as f32),
                    },
                });
            }
        });
}

/// Game setup
pub fn setup(
    mut commands: Commands,
    gridworld: Res<GridWorld>,
    mut round: ResMut<GameRound>,
    colors: Res<AutomataColors>,
    button_colors: Res<ButtonColors>,
    fonts: Res<Fonts>,
) {
    // cameras
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(GRID_HEIGHT as f32 / 2.0);

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(UiCamera)
        .insert(Name::new("Main Camera"));

    // grid world
    let parent = commands
        .spawn_bundle(EmptyBundle::default())
        .insert(Name::new("GridWorld"))
        .id();
    for cell in gridworld.cells.iter() {
        GridWorldCell::spawn(&mut commands, parent, cell.0, colors.cell);
    }

    round.reset();

    // cell selection UI
    let root = spawn_ui_root(&mut commands);
    commands
        .entity(root)
        .insert(CellSelection)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(Name::new("Cell Selection"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            "Select a cell for your automaton",
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                });

            for row in 0..GRID_HEIGHT {
                spawn_cell_selection_row(parent, &button_colors, row);
            }
        });

    // HUD UI
    let root = spawn_ui_root(&mut commands);
    commands.entity(root).insert(Hud).with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(Name::new("HUD"))
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Player Health:",
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                });

                parent.spawn_bundle(AutomataHealthTextBundle {
                    text: TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    },
                    health_text: AutomataHealthText { player: true },
                });

                // TODO: why doesn't this work?
                spawn_spacer(parent);

                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Round:",
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                });

                parent.spawn_bundle(RoundTextBundle {
                    text: TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            format!("{}", round.round + 1),
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    },
                    round_text: RoundText,
                });

                // TODO: why doesn't this work?
                spawn_spacer(parent);

                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "AI Health:",
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                });

                parent.spawn_bundle(AutomataHealthTextBundle {
                    text: TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            format!("{}", 0),
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    },
                    health_text: AutomataHealthText { player: false },
                });
            });
    });
}

/// Cell selection button handler
pub fn cell_selection_button_handler(
    mut commands: Commands,
    mut random: ResMut<Random>,
    mut round: ResMut<GameRound>,
    colors: Res<AutomataColors>,
    query: Query<(&Interaction, &ButtonHelper, &CellSelectionButton), Changed<Interaction>>,
    cell_selection_ui_query: Query<Entity, With<CellSelection>>,
    hud_query: Query<Entity, With<Hud>>,
    mut visibility_query: Query<&mut Visibility>,
    children_query: Query<&Children>,

    mut game_start_events: EventWriter<GameStartEvent>,
) {
    if round.stage != GameStage::CellSelection {
        return;
    }

    if let Ok((interaction, helper, selection)) = query.get_single() {
        if helper.interactable() && *interaction == Interaction::Clicked {
            // hide cell selection UI
            let cell_selection_ui = cell_selection_ui_query.single();
            debug!("Disabling cell selection");
            set_visible_recursive(
                cell_selection_ui,
                false,
                &mut visibility_query,
                &children_query,
            );

            // show HUD
            let hud = hud_query.single();
            debug!("Enabling HUD...");
            set_visible_recursive(hud, true, &mut visibility_query, &children_query);

            // spawn automata
            let parent = commands
                .spawn_bundle(EmptyBundle::default())
                .insert(Name::new("Automata"))
                .id();

            let player_cell = UVec2::new(selection.cell.x as u32, selection.cell.y as u32);
            Automata::spawn_player(&mut commands, parent, colors.player_automata, player_cell);
            Automata::spawn_ai(
                &mut commands,
                parent,
                colors.ai_automata,
                player_cell,
                &mut random,
            );

            game_start_events.send(GameStartEvent);

            round.stage = GameStage::Running;
        }
    }
}

/// Game start event handler
pub fn game_start_event_handler(
    mut events: EventReader<GameStartEvent>,
    round: Res<GameRound>,
    player_stats: Res<PlayerAutomataStats>,
    ai_population: Res<AIAutomataPopulation>,
    mut player_automata_query: Query<&mut Automata, (With<PlayerAutomata>, Without<AIAutomata>)>,
    mut ai_automata_query: Query<&mut Automata, (With<AIAutomata>, Without<PlayerAutomata>)>,
    mut health_text_query: Query<(&mut Text, &AutomataHealthText)>,
) {
    for _ in events.iter() {
        if let Ok(mut automata) = player_automata_query.get_single_mut() {
            automata.reset(&*player_stats);

            for (mut text, health) in health_text_query.iter_mut() {
                if !health.player {
                    continue;
                }

                text.sections[0].value = format!("{}", automata.health);
            }
        }

        if let Ok(mut automata) = ai_automata_query.get_single_mut() {
            let ai_stats = ai_population.round_stats(round.round);
            automata.reset(ai_stats);

            for (mut text, health) in health_text_query.iter_mut() {
                if health.player {
                    continue;
                }

                text.sections[0].value = format!("{}", automata.health);
            }
        }
    }
}

/// Automata health changed event handler
pub fn health_changed_event_handler(
    mut events: EventReader<HealthChangedEvent>,
    mut text_query: Query<(&mut Text, &AutomataHealthText)>,
) {
    for event in events.iter() {
        for (mut text, health) in text_query.iter_mut() {
            if health.player == event.player {
                text.sections[0].value = format!("{}", event.value);
            }
        }
    }
}

/// Automata action handler
#[allow(clippy::needless_return)]
pub fn automata_action(
    mut round: ResMut<GameRound>,
    mut player_automata_query: Query<&mut Automata, (With<PlayerAutomata>, Without<AIAutomata>)>,
    mut ai_automata_query: Query<&mut Automata, (With<AIAutomata>, Without<PlayerAutomata>)>,
) {
    if round.stage != GameStage::Running {
        return;
    }

    // TODO: we need a cooldown on taking actions
    // so things don't progress too fast

    if let Ok(mut player) = player_automata_query.get_single_mut() {
        if let Ok(mut ai) = ai_automata_query.get_single_mut() {
            match round.action {
                GameAction::PlayerMove => {
                    player.move_action();

                    round.action = GameAction::PlayerAttack;
                }
                GameAction::PlayerAttack => {
                    player.attack_action();

                    round.action = GameAction::AIMove;
                }
                GameAction::AIMove => {
                    ai.move_action();

                    round.action = GameAction::AIAttack;
                }
                GameAction::AIAttack => {
                    ai.attack_action();

                    round.action = GameAction::PlayerMove;
                }
            };
        }
    }
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
