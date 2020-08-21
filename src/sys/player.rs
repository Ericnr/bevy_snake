use crate::comp;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

/// Converts real player input into Controller input
pub fn handle_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&comp::actor::Player, &mut comp::actor::Controller)>,
) {
    for (_, mut controller) in &mut query.iter() {
        // up
        if keyboard_input.pressed(KeyCode::W) {
            *controller.movement.y_mut() += 1.0;
        }
        // down
        if keyboard_input.pressed(KeyCode::S) {
            *controller.movement.y_mut() -= 1.0;
        }
        // right
        if keyboard_input.pressed(KeyCode::D) {
            *controller.movement.x_mut() += 1.0;
        }
        // left
        if keyboard_input.pressed(KeyCode::A) {
            *controller.movement.x_mut() -= 1.0;
        }
        // shoot
        if keyboard_input.pressed(KeyCode::Space) {
            controller
                .action
                .push_back(comp::actor::ControllerAction::Shoot);
        }
    }
}
