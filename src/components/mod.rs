//! ECS components

pub mod automata;
pub mod gridworld;
pub mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

/// Main camera tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct MainCamera;

/// UI camera tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct UiCamera;
