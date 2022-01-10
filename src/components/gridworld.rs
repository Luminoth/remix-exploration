//! GridWorld components

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::bundles::gridworld::*;
use crate::util::*;

const PADDING: f32 = 0.1;

/// GridWorld Cell tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct GridWorldCell;

impl GridWorldCell {
    pub fn spawn(commands: &mut Commands, parent: Entity, cell: UVec2, color: Color) {
        let position = cell_position(cell, 0.0);
        //debug!("Cell position: {}", position);

        commands.entity(parent).with_children(|parent| {
            parent
                .spawn_bundle(GridWorldCellBundle {
                    cell: GridWorldCell::default(),
                    transform: Transform::from_translation(position),
                    global_transform: GlobalTransform::default(),
                })
                .insert(Name::new(format!("Cell {}", cell)))
                .with_children(|parent| {
                    parent.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(
                                crate::CELL_WIDTH - PADDING,
                                crate::CELL_HEIGHT - PADDING,
                            )),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
    }
}
