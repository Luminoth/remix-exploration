//! Intro state systems

use bevy::prelude::*;

use super::*;

use crate::components::ui::*;
use crate::components::*;
use crate::resources::ui::*;

/// Intro setup
pub fn setup(
    mut commands: Commands,
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
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: ui_materials.none.clone(),
            ..Default::default()
        })
        .insert(Name::new("UI Root"))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    material: ui_materials.none.clone(),
                    ..Default::default()
                })
                .insert(Name::new("Instructions"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Remix Exploration",
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });

                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                margin: Rect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal.clone(),
                            ..Default::default()
                        })
                        .insert(ActionButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Play",
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
        });
}

pub fn update(
    materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<ActionButton>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok((interaction, mut material)) = query.single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = materials.pressed.clone();

                state.set(GameState::Remix).unwrap();
            }
            Interaction::Hovered => {
                *material = materials.hovered.clone();
            }
            Interaction::None => {
                *material = materials.normal.clone();
            }
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
