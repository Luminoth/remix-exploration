//! Game state systems

use bevy::prelude::*;

//use crate::components::automata::*;
use crate::components::ui::*;
use crate::components::*;
use crate::resources::ui::*;
use crate::*;

// TODO: player has to select a grid slot
// and then the AI has to select a grid slot

/// Game setup
pub fn setup(
    mut commands: Commands,
    //player_stats: Res<PlayerAutomataStats>,
    //ai_stats: Res<AIAutomataStats>,
    //materials: Res<Materials>,
    ui_materials: Res<UiMaterials>,
    fonts: Res<Fonts>,
) {
    // cameras
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical;
    camera.orthographic_projection.scale = GRID_HEIGHT as f32 / 2.0;

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera)
        .insert(Name::new("UI Camera"));

    // spawn automata
    //Automata::spawn_player(&mut commands, &materials, *player_stats, UVec2::new(0, 0));
    //Automata::spawn_ai(&mut commands, &materials, *ai_stats, UVec2::new(1, 1));

    // cell selection UI
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            material: ui_materials.none.clone(),
            ..Default::default()
        })
        .insert(CellSelection)
        .insert(Name::new("UI Root"))
        .with_children(|parent| {
            // TODO:
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    material: ui_materials.none.clone(),
                    ..Default::default()
                })
                .insert(Name::new("Cell Selection"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Select a cell for your automaton",
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });

    // HUD UI
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            material: ui_materials.none.clone(),
            visible: Visible {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HUD)
        .insert(Name::new("UI Root"))
        .with_children(|parent| {
            // TODO:
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    material: ui_materials.none.clone(),
                    visible: Visible {
                        is_visible: false,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("HUD"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Running ...",
                            TextStyle {
                                font: fonts.normal.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        visible: Visible {
                            is_visible: false,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
