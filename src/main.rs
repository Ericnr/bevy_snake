mod wall;
mod snake;
mod collision;

use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct Snake {
    pub length: u32,
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(wall::WallsPlugin)
        .add_plugin(snake::SnakePlugin)
        .run();
}

