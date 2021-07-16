//! GridWorld components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bundles::gridworld::*;
use crate::util::*;

const PADDING: f32 = 0.1;

/// GridWorld Cell tag
#[derive(Debug, Inspectable, Default)]
pub struct GridWorldCell;

impl GridWorldCell {
    pub fn spawn(
        commands: &mut Commands,
        parent: Entity,
        cell: UVec2,
        material: Handle<ColorMaterial>,
    ) {
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
                        material,
                        sprite: Sprite::new(Vec2::new(
                            crate::CELL_WIDTH - PADDING,
                            crate::CELL_HEIGHT - PADDING,
                        )),
                        ..Default::default()
                    });
                });
        });
    }
}
