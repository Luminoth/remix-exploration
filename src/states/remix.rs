//! Remix state systems

use bevy::prelude::*;

use super::*;

use crate::bundles::ui::*;
use crate::components::ui::*;
use crate::components::*;
use crate::events::remix::*;
use crate::game::stats::*;
use crate::resources::automata::*;
use crate::resources::ui::*;

/// Spawn a stat input
fn spawn_stat_input(
    parent: &mut ChildBuilder,
    ui_materials: &UiMaterials,
    button_materials: &ButtonMaterials,
    fonts: &Fonts,
    statid: StatId,
    player_stats: &PlayerAutomataStats,
) {
    let stat_name = stat_name(statid);

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: ui_materials.none.clone(),
            ..Default::default()
        })
        .insert(Name::new(stat_name.clone()))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    stat_name,
                    TextStyle {
                        font: fonts.normal.clone(),
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });

            parent.spawn_bundle(StatModifierTextBundle {
                text: TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("{}", player_stats.stats().value(statid)),
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                },
                modifier_text: StatModifierText { statid },
            });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: ui_materials.none.clone(),
                    ..Default::default()
                })
                .insert(Name::new("Modifier Buttons"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(StatModifierButtonBundle {
                            button: ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(15.0), Val::Px(15.0)),
                                    margin: Rect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                material: button_materials.normal.clone(),
                                ..Default::default()
                            },
                            helper: ButtonHelper { interactable: true },
                            modifier_button: StatModifierButton {
                                statid,
                                modifier: 1,
                            },
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "^",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size: 12.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });

                    parent
                        .spawn_bundle(StatModifierButtonBundle {
                            button: ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(15.0), Val::Px(15.0)),
                                    margin: Rect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                material: button_materials.disabled.clone(),
                                ..Default::default()
                            },
                            helper: ButtonHelper {
                                interactable: false,
                            },
                            modifier_button: StatModifierButton {
                                statid,
                                modifier: -1,
                            },
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "v",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size: 12.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        });
}

/// Remix setup
pub fn setup(
    mut commands: Commands,
    player_stats: Res<PlayerAutomataStats>,
    ui_materials: Res<UiMaterials>,
    button_materials: Res<ButtonMaterials>,
    fonts: Res<Fonts>,
) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera)
        .insert(Name::new("UI Camera"));

    // UI
    let root = spawn_ui_root(&mut commands, &ui_materials);
    commands.entity(root).with_children(|parent| {
        spawn_header(parent, &fonts, "Remix Your Automaton");

        // remaining points
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: ui_materials.none.clone(),
                ..Default::default()
            })
            .insert(Name::new("Stat Points"))
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Remaining Stat Points: ",
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });

                parent.spawn_bundle(PointsTextBundle {
                    text: TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            format!("{}", player_stats.points()),
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    },
                    points_text: PointsText,
                });
            });

        spawn_stat_input(
            parent,
            &ui_materials,
            &button_materials,
            &fonts,
            StatId::Fortitude,
            &player_stats,
        );

        spawn_spacer(parent, &ui_materials);

        spawn_ok_action(parent, &ui_materials, &button_materials, &fonts, "Run");
    });
}

/// Stat modified event handler
pub fn stat_modified_event_handler(
    stats: ResMut<PlayerAutomataStats>,
    mut events: EventReader<StatModifiedEvent>,
    mut text_query: Query<(&mut Text, &StatModifierText), Without<PointsText>>,
    mut points_text_query: Query<&mut Text, With<PointsText>>,
    mut modifier_query: Query<(&mut ButtonHelper, &StatModifierButton), Without<ActionButton>>,
    mut action_query: Query<&mut ButtonHelper, With<ActionButton>>,
) {
    for event in events.iter() {
        for (mut text, modifier) in text_query.iter_mut() {
            if modifier.statid == event.0 {
                text.sections[0].value = format!("{}", stats.value(modifier.statid));
            }
        }

        if let Ok(mut text) = points_text_query.single_mut() {
            text.sections[0].value = format!("{}", stats.points());
        }

        for (mut helper, modifier) in modifier_query.iter_mut() {
            if modifier.statid == event.0 {
                match modifier.modifier.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        helper.interactable = stats.value(modifier.statid) > 0
                    }
                    std::cmp::Ordering::Greater => helper.interactable = stats.points() > 0,
                    std::cmp::Ordering::Equal => (),
                }
            }
        }

        if let Ok(mut action_helper) = action_query.single_mut() {
            action_helper.interactable = stats.points() == 0;
        }
    }
}

/// Modifier button handler
pub fn modifier_button_handler(
    mut stats: ResMut<PlayerAutomataStats>,
    mut modifier_query: Query<
        (&Interaction, &ButtonHelper, &StatModifierButton),
        (Changed<Interaction>, Without<ActionButton>),
    >,
    mut state_modified_events: EventWriter<StatModifiedEvent>,
) {
    if let Ok((interaction, helper, modifier)) = modifier_query.single_mut() {
        #[allow(clippy::collapsible_if)]
        if helper.interactable && *interaction == Interaction::Clicked {
            if stats.modify(modifier.statid, modifier.modifier) {
                state_modified_events.send(StatModifiedEvent(modifier.statid));
            }
        }
    }
}

/// Action button handler
pub fn action_button_handler(
    mut action_query: Query<
        (&Interaction, &ButtonHelper),
        (Changed<Interaction>, With<ActionButton>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok((interaction, helper)) = action_query.single_mut() {
        if helper.interactable && *interaction == Interaction::Clicked {
            state.set(GameState::Game).unwrap();
        }
    }
}

/// Remix teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
