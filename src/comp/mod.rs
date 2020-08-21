//! All components are in this module, separated in "categories" (modules)
pub mod actor;
pub mod physics;
pub mod stats;
pub mod ui;

use bevy::prelude::*;

/// AutoCleaned entities are removed after leaving
/// bottom part of GameBounds.
///
/// TODO: Find this a better place and name.
#[derive(Debug, Default, Properties)]
pub struct AutoCleaned;
