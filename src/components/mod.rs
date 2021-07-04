//! ECS components

pub mod automata;
pub mod gridworld;

use bevy_inspector_egui::Inspectable;

/// Main camera tag
#[derive(Debug, Inspectable, Default)]
pub struct MainCamera;

/// UI camera tag
#[derive(Debug, Inspectable, Default)]
pub struct UiCamera;
