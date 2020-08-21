//! My home-made physics mechanics.
//! Nothing wonderful, but it does its job.
//!
//! This will be eventually replaced by real physics engine.
use crate::comp::physics;
use crate::res;

use bevy::prelude::*;

/// Plugin bringing in all custom physics systems, resources and events.
pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::CollisionEvent>()
            .add_system(process_velocity_system.system())
            .add_system(drag_system.system())
            .add_system_to_stage(stage::UPDATE, check_collision_system.system());
    }
}

/// Move entities with Velocity components
pub fn process_velocity_system(
    time: Res<Time>,
    mut query: Query<(&physics::Velocity, &mut Translation)>,
) {
    for (velocity, mut translation) in &mut query.iter() {
        *translation.x_mut() += time.delta_seconds * velocity.x();
        *translation.y_mut() += time.delta_seconds * velocity.y();
    }
}

pub fn drag_system(time: Res<Time>, mut query: Query<(&mut physics::Velocity, &physics::Drag)>) {
    for (mut velocity, drag) in &mut query.iter() {
        *velocity =
            physics::Velocity(velocity.lerp(Vec2::new(0.0, 0.0), time.delta_seconds * drag.0));
    }
}

/// Check and report entity collisions
pub fn check_collision_system(
    mut collision_events: ResMut<Events<res::CollisionEvent>>,
    mut query1: Query<(Entity, &physics::ColliderBox, &Translation)>,
    mut query2: Query<(Entity, &physics::ColliderBox, &Translation)>,
) {
    let mut registered_touches = Vec::new();

    for (ent, collider, translation) in &mut query1.iter() {
        let half_w = collider.w as f32 / 2.0;
        let left = translation.x() - half_w;
        let right = translation.x() + half_w;

        let half_h = collider.h as f32 / 2.0;
        let top = translation.y() + half_h;
        let bottom = translation.y() - half_h;
        for (ent2, collider2, translation2) in &mut query2.iter() {
            if ent == ent2 {
                continue;
            }
            let half_w2 = collider2.w as f32 / 2.0;
            let left2 = translation2.x() - half_w2;
            let right2 = translation2.x() + half_w2;

            let half_h2 = collider2.h as f32 / 2.0;
            let top2 = translation2.y() + half_h2;
            let bottom2 = translation2.y() - half_h2;

            let xc = left < right2 && right > left2;
            let yc = top > bottom2 && bottom < top2;

            let collision = res::CollisionEvent::new(ent, ent2);
            if xc && yc && !registered_touches.contains(&collision) {
                registered_touches.push(collision.clone());
                collision_events.send(collision);
            }
        }
    }
}
