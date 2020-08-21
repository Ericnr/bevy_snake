//! All systems are in this module, separated in "categories" (modules)
//!
//! Each category is packed in a plugin for ease of use.

pub mod actor;
pub mod physics;
pub mod player;
pub mod stats;
pub mod ui;
pub mod util;

use bevy::prelude::*;

/// This plugin bundles all the game logic and support systems.
pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player::handle_input_system.system())
            .add_plugin(actor::GameActorPlugin)
            .add_plugin(physics::GamePhysicsPlugin)
            .add_plugin(stats::GameStatsPlugin)
            .add_plugin(ui::GameUiPlugin)
            .add_plugin(util::GameUtilPlugin);
    }
}
