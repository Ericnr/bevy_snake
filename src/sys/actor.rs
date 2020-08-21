use crate::comp::actor;
use crate::comp::physics;
use crate::comp::stats;
use crate::res;
use crate::spawn;

use bevy::prelude::*;

/// Plugin that brings in all actor related functionality.
/// Includes Controller system for input abstraction.
pub struct GameActorPlugin;

impl Plugin for GameActorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::EntityCommandEvent>()
            .init_resource::<res::WeaponShootCommandListenerState>()
            .add_system(process_commands_system.system())
            .add_system(weapon_shoot_system.system())
            .add_system(weapon_reload_system.system());
    }
}

/// Processes controller inputs.
/// This way, it should easier to make AI with same "input abilities" as a human actor (Player).
pub fn process_commands_system(
    mut entity_command_events: ResMut<Events<res::EntityCommandEvent>>,
    mut query: Query<(
        Entity,
        &mut actor::Controller,
        &mut physics::Velocity,
        &stats::MovementSpeed,
        // TODO: This is a hack, remove this
        &Translation,
    )>,
) {
    for (ent, mut controller, mut velocity, speed, trans) in &mut query.iter() {
        // Check and potentially normalize movement vector (in case of a different input method??).
        // If the vector is zero, do not normalize it (produces garbage).
        let movement = if controller.movement.x() + controller.movement.y() != 0.0 {
            controller.movement.normalize()
        } else {
            controller.movement
        };

        let horizontal_direction = movement.x();
        let vertical_direction = movement.y();

        // Process input events, like shoot commands
        for command in controller.action.drain(..) {
            match command {
                actor::ControllerAction::Shoot => {
                    entity_command_events.send(res::EntityCommandEvent::new(
                        ent,
                        res::ENTITY_COMMAND_SHOOT.to_owned(),
                        trans.truncate(),
                    ));
                }
            }
        }

        // Now apply movement vector to input.
        // Cap horizontal speed.
        if (velocity.x() + horizontal_direction * speed.accel).abs() < speed.max {
            *velocity.x_mut() += horizontal_direction * speed.accel;
        } else {
            if velocity.x() < 0.0 {
                *velocity.x_mut() = -speed.max;
            } else {
                *velocity.x_mut() = speed.max;
            }
        }
        // Cap vertical speed.
        if (velocity.y() + vertical_direction * speed.accel).abs() < speed.max {
            *velocity.y_mut() += vertical_direction * speed.accel;
        } else {
            if velocity.y() < 0.0 {
                *velocity.y_mut() = -speed.max;
            } else {
                *velocity.y_mut() = speed.max;
            }
        }

        // Reset controller movement
        controller.movement.set_x(0.0);
        controller.movement.set_y(0.0);
    }
}

/// Handles weapon shooting.
///
/// Processes "shoot" command for Controlled entities.
pub fn weapon_shoot_system(
    mut commands: Commands,
    entity_command_events: Res<Events<res::EntityCommandEvent>>,
    mut state: ResMut<res::WeaponShootCommandListenerState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut weapons: Query<(&actor::Controlled, &mut actor::Weapon, &Translation)>,
    energy_sources: Query<(Entity, &mut stats::EnergyStats)>,
) {
    for event in state.event_reader.iter(&entity_command_events) {
        if event.command != res::ENTITY_COMMAND_SHOOT {
            continue;
        }
        for (controlled, mut weapon, trans) in &mut weapons.iter() {
            if controlled.by == event.src {
                if weapon.reload.finished {
                    if weapon.kind == "nova_blast" {
                        let orig_loc = event.src_loc; // TODO: <- This is a hack to circumvent relative translation, remove this.

                        let data = spawn::ProjectileDataComponents {
                            translation: Translation::new(
                                trans.x() + orig_loc.x(),
                                trans.y() + orig_loc.y(),
                                2.0,
                            ),
                            velocity: physics::Velocity::new(0.0, 320.0),
                            damage: stats::Damage { hull: 50 },
                            ..Default::default()
                        };
                        // TODO: This energy requirement handling should be reviewed.
                        let energy = energy_sources.get_mut::<stats::EnergyStats>(controlled.by);
                        let can_shoot = if let Ok(mut energy) = energy {
                            if energy.energy - weapon.energy_drain >= 0 {
                                energy.energy -= weapon.energy_drain;
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        };
                        if can_shoot {
                            spawn::spawn_projectile(
                                &mut commands,
                                &asset_server,
                                &mut materials,
                                controlled.by,
                                data,
                                None,
                            );
                            weapon.reload.reset();
                        }
                    }
                }
            }
        }
    }
}

/// Reloads Weapons (ticks reload Timer).
pub fn weapon_reload_system(time: Res<Time>, mut query: Query<&mut actor::Weapon>) {
    for mut weapon in &mut query.iter() {
        weapon.reload.tick(time.delta_seconds);
    }
}
