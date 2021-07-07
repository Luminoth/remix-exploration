//! Utilities

#![allow(dead_code)]

use bevy::prelude::*;
use num_traits::Float;

/// Clamps an ord between a min and a max
pub fn clamp<T: Ord>(v: T, min: T, max: T) -> T {
    std::cmp::min(max, std::cmp::max(min, v))
}

/// Clamps a float between a min and a max
pub fn clampf<F: Float>(v: F, min: F, max: F) -> F {
    Float::min(max, Float::max(min, v))
}

/// Recursively set the visibility of an entity and its children
// https://github.com/bevyengine/bevy/issues/838
pub fn set_visible_recursive(
    entity: Entity,
    is_visible: bool,
    visible_query: &mut Query<&mut Visible>,
    children_query: &Query<&Children>,
) {
    if let Ok(mut visible) = visible_query.get_mut(entity) {
        visible.is_visible = is_visible;
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            set_visible_recursive(*child, is_visible, visible_query, children_query);
        }
    }
}
