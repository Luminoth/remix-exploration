//! Game state systems

use bevy::prelude::*;

use super::*;

use crate::bundles::*;
use crate::components::automata::*;
use crate::components::gridworld::*;
use crate::components::ui::*;
use crate::components::*;
use crate::resources::automata::*;
use crate::resources::game::*;
use crate::resources::gridworld::*;
use crate::resources::ui::*;
use crate::util::*;
use crate::*;

const GRID_BUTTON_WIDTH: f32 = crate::WINDOW_WIDTH / crate::GRID_WIDTH as f32;
const GRID_BUTTON_HEIGHT: f32 = crate::WINDOW_HEIGHT / crate::GRID_HEIGHT as f32;

/// Spawn a cell selection row
fn spawn_cell_selection_row(
    parent: &mut ChildBuilder,
    ui_materials: &UiMaterials,
    button_materials: &ButtonMaterials,
    row: usize,
) {
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
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: button_materials.normal.clone(),
                        ..Default::default()
                    },
                    helper: ButtonHelper { interactable: true },
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
    materials: Res<Materials>,
    ui_materials: Res<UiMaterials>,
    button_materials: Res<ButtonMaterials>,
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

    // grid world
    let parent = commands
        .spawn_bundle(EmptyBundle::default())
        .insert(Name::new("GridWorld"))
        .id();
    for cell in gridworld.cells.iter() {
        GridWorldCell::spawn(&mut commands, parent, cell.0, materials.cell.clone());
    }

    // stage
    commands.insert_resource(GameStage::CellSelection);

    // cell selection UI
    let root = spawn_ui_root(&mut commands, &ui_materials);
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

            for row in 0..GRID_HEIGHT {
                spawn_cell_selection_row(parent, &ui_materials, &button_materials, row);
            }
        });

    // HUD UI
    let root = spawn_ui_root(&mut commands, &ui_materials);
    commands.entity(root).insert(HUD).with_children(|parent| {
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

/// Cell selection button handler
pub fn cell_selection_button_handler(
    mut commands: Commands,
    mut stage: ResMut<GameStage>,
    query: Query<(&Interaction, &ButtonHelper, &CellSelectionButton), Changed<Interaction>>,
    cell_selection_ui_query: Query<Entity, With<CellSelection>>,
    hud_query: Query<Entity, With<HUD>>,
    mut visible_query: Query<&mut Visible>,
    children_query: Query<&Children>,
    materials: Res<Materials>,
) {
    if *stage != GameStage::CellSelection {
        return;
    }

    if let Ok((interaction, helper, selection)) = query.single() {
        #[allow(clippy::collapsible_if)]
        if helper.interactable && *interaction == Interaction::Clicked {
            // hide cell selection UI
            if let Ok(cell_selection_ui) = cell_selection_ui_query.single() {
                debug!("Disabling cell selection");
                set_visible_recursive(
                    cell_selection_ui,
                    false,
                    &mut visible_query,
                    &children_query,
                );
            }

            // show HUD
            if let Ok(hud) = hud_query.single() {
                debug!("Enabling HUD...");
                set_visible_recursive(hud, true, &mut visible_query, &children_query);
            }

            // spawn automata
            info!("Spawning player at {}", selection.cell);

            let parent = commands
                .spawn_bundle(EmptyBundle::default())
                .insert(Name::new("Automata"))
                .id();

            let cell = UVec2::new(selection.cell.x as u32, selection.cell.y as u32);
            Automata::spawn_player(&mut commands, parent, &materials, cell);
            //Automata::spawn_ai(&mut commands, parent, &materials, UVec2::new(1, 1));

            *stage = GameStage::Running;
        }
    }
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
    commands.remove_resource::<GameStage>();
}
