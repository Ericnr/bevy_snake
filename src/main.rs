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
// .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
// .add_resource(WindowDescriptor {
//     title: "Per Spatium".to_string(),
//     width: 800,
//     height: 720,
//     vsync: true,
//     mode: bevy::window::WindowMode::Windowed,
//     resizable: true,
// })
// .add_plugin(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
//     1.0 / 60.0,
// )))
// .add_plugin(FrameTimeDiagnosticsPlugin::default())
// .add_default_plugins()
// .add_plugin(setup::GameSetupPlugin)
// .add_plugin(sys::GameLogicPlugin)
