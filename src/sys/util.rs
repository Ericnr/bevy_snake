//! This module includes all sort of systems.
//! Many of them are developer/test systems.
use crate::comp;
use crate::res;
use crate::spawn;

use bevy::prelude::*;

/// Plugin bringing in all sort of utility/developer systems and resources
pub struct GameUtilPlugin;

impl Plugin for GameUtilPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::DeveloperCommandEvent>()
            .add_event::<res::DisposeEntityEvent>()
            .add_event::<res::SpawnEntityEvent>()
            .init_resource::<res::DeveloperExecutiveState>()
            .init_resource::<res::GenericSpawnEntityState>()
            .init_resource::<res::GameplaySpawnState>()
            .add_resource(res::GraveyardState {
                location: Translation::new(50000.0, 50000.0, -1.0),
                ..Default::default()
            })
            .add_resource(res::GameBounds {
                point: Vec2::new(0.0, 0.0),
                dimension: Vec2::new(res::GAME_BOUNDS_DIMENSION_W, res::GAME_BOUNDS_DIMENSION_H),
            })
            .add_startup_system(generic_spawn_entity_startup_system.system())
            .add_system(developer_input_system.system())
            .add_system(developer_executive_system.system())
            .add_system(keep_player_in_bounds_system.system())
            .add_system(generic_spawn_entity_system.system())
            .add_system(off_bounds_cleanup_system.system())
            .add_system(gameplay_spawn_system.system())
            .add_system(check_game_over_system.system())
            .add_system_to_stage(stage::POST_UPDATE, charon_system.system());
    }
}

/// Developer input system for creating DeveloperCommandEvents
pub fn developer_input_system(
    mut dev_events: ResMut<Events<res::DeveloperCommandEvent>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::F1) {
        dev_events.send(res::DeveloperCommandEvent {
            command: "spawn_asteroid".to_string(),
        });
    }
    if keyboard_input.just_pressed(KeyCode::F2) {
        dev_events.send(res::DeveloperCommandEvent {
            command: "spawn_projectile".to_string(),
        });
    }
    if keyboard_input.just_pressed(KeyCode::F3) {
        dev_events.send(res::DeveloperCommandEvent {
            command: "spawn_powerup".to_string(),
        });
    }
}

/// Little developer helper, that can do developerish things.
pub fn developer_executive_system(
    mut state: ResMut<res::DeveloperExecutiveState>,
    dev_events: Res<Events<res::DeveloperCommandEvent>>,
    cursor_events: Res<Events<CursorMoved>>,
    bounds: Res<res::GameBounds>,
    mut spawn_events: ResMut<Events<res::SpawnEntityEvent>>,
) {
    let cursor_change = state.cursor_event_reader.latest(&cursor_events);
    if let Some(evt) = cursor_change {
        state.last_cursor_pos = evt.position;
    }
    for event in state.developer_event_reader.iter(&dev_events) {
        if event.command == "spawn_asteroid" {
            let mut rng = rand::thread_rng();
            use rand::Rng;

            let vx: f32 = rng.gen_range(-10.0, 10.0);
            let vy: f32 = rng.gen_range(-10.0, 10.0);

            let tx: f32 = rng.gen_range(
                bounds.point.x() - bounds.dimension.x() / 2.0,
                bounds.point.x() + bounds.dimension.x() / 2.0,
            );
            let ty: f32 = rng.gen_range(
                bounds.point.y() - bounds.dimension.y() / 2.0,
                bounds.point.y() + bounds.dimension.y() / 2.0,
            );

            let event = res::SpawnEntityEvent {
                spawn_type: res::SpawnType::Asteroid,
                location: Translation::new(tx, ty, 0.0),
                velocity: Some(comp::physics::Velocity::new(vx, vy)),
                origin: None,
            };
            spawn_events.send(event);
        } else if event.command == "spawn_projectile" {
            let (tx, ty) = state.last_cursor_pos.into();

            let event = res::SpawnEntityEvent {
                spawn_type: res::SpawnType::Projectile,
                location: Translation::new(tx, ty, 2.0),
                velocity: Some(comp::physics::Velocity::new(0.0, 80.0)),
                origin: None,
            };
            spawn_events.send(event);
        } else if event.command == "spawn_powerup" {
            let mut rng = rand::thread_rng();
            use rand::Rng;

            let tx: f32 = rng.gen_range(
                bounds.point.x() - bounds.dimension.x() / 2.0,
                bounds.point.x() + bounds.dimension.x() / 2.0,
            );
            let ty: f32 = bounds.point.y() + bounds.dimension.y() * 0.8;

            let event = res::SpawnEntityEvent {
                spawn_type: res::SpawnType::Powerup,
                location: Translation::new(tx, ty, 1.0),
                velocity: Some(comp::physics::Velocity::new(0.0, -40.0)),
                origin: None,
            };
            spawn_events.send(event);
        } else {
            println!("Unknown developer command \'{}\'", event.command)
        }
    }
}

/// Keeps player within bounds defined as a resource
pub fn keep_player_in_bounds_system(
    bounds: Res<res::GameBounds>,
    mut query: Query<(
        &comp::actor::Player,
        &mut comp::physics::Velocity,
        &mut Translation,
    )>,
) {
    let half_x_bound = bounds.dimension.x() / 2.0;
    let half_y_bound = bounds.dimension.y() / 2.0;
    for (_, mut velocity, mut translation) in &mut query.iter() {
        if translation.x() > bounds.point.x() + half_x_bound {
            *translation.x_mut() = bounds.point.x() + half_x_bound;
            *velocity.x_mut() = 0.0;
        } else if translation.x() < bounds.point.x() - half_x_bound {
            *translation.x_mut() = bounds.point.x() - half_x_bound;
            *velocity.x_mut() = 0.0;
        }

        if translation.y() > bounds.point.y() + half_y_bound {
            *translation.y_mut() = bounds.point.y() + half_y_bound;
            *velocity.y_mut() = 0.0;
        } else if translation.y() < bounds.point.y() - half_y_bound {
            *translation.y_mut() = bounds.point.y() - half_y_bound;
            *velocity.y_mut() = 0.0;
        }
    }
}

/// Prepares global spawner
///
/// TODO: This is a hack. It should be removed.
pub fn generic_spawn_entity_startup_system(
    mut commands: Commands,
    mut state: ResMut<res::GenericSpawnEntityState>,
) {
    let representative = commands
        .spawn(spawn::Spawner::default())
        .current_entity()
        .unwrap();
    state.representative = Some(representative);
}

/// Spawns things on request
///
/// TODO: There is an entity required to spawn some things.
/// This "global spawner" is nothing more than a hack for testing.
/// This entire system should be redesigned eventually.
pub fn generic_spawn_entity_system(
    mut commands: Commands,
    events: Res<Events<res::SpawnEntityEvent>>,
    mut state: ResMut<res::GenericSpawnEntityState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in state.event_reader.iter(&events) {
        match event.spawn_type {
            res::SpawnType::Asteroid => {
                let mut data = spawn::AsteroidDataComponents {
                    translation: event.location,
                    ..Default::default()
                };
                if let Some(vel) = &event.velocity {
                    data.velocity = vel.clone();
                }
                spawn::spawn_asteroid(&mut commands, &asset_server, &mut materials, data, None);
            }
            res::SpawnType::Projectile => {
                let mut data = spawn::ProjectileDataComponents {
                    translation: event.location,
                    ..Default::default()
                };
                if let Some(vel) = &event.velocity {
                    data.velocity = vel.clone();
                }
                spawn::spawn_projectile(
                    &mut commands,
                    &asset_server,
                    &mut materials,
                    state.representative.unwrap(),
                    data,
                    None,
                );
            }
            res::SpawnType::Powerup => {
                let mut data = spawn::PowerupDataComponents {
                    translation: event.location,
                    ..Default::default()
                };
                if let Some(vel) = &event.velocity {
                    data.velocity = vel.clone();
                }

                // Randomly select PowerUp kind.
                let mut rng = rand::thread_rng();
                use rand::Rng;

                let select: u32 = rng.gen_range(0, 3);

                let kind = match select {
                    0 => spawn::PowerUpKind::Speed(comp::stats::SpeedPowerUp {
                        speed: comp::stats::MovementSpeed {
                            accel: 2.0,
                            max: 40.0,
                        },
                    }),
                    1 => spawn::PowerUpKind::Health(comp::stats::HealthPowerUp {
                        health: comp::stats::HealthStats {
                            hull: 90,
                            max_hull: 0,
                        },
                    }),
                    _ => spawn::PowerUpKind::Energy(comp::stats::EnergyPowerUp {
                        energy: comp::stats::EnergyStats {
                            energy: 200,
                            max_energy: 20,
                        },
                    }),
                };

                spawn::spawn_powerup(
                    &mut commands,
                    &asset_server,
                    &mut materials,
                    data,
                    vec![kind],
                    None,
                );
            }
        }
    }
}

/// Periodically spawns asteroids and powerups and "produces gameplay".
///
/// NOTE: This is a **purely placeholder system**, only for primitive gameplay/testing.
pub fn gameplay_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    bounds: Res<res::GameBounds>,
    mut state: ResMut<res::GameplaySpawnState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Stop spawning
    if state.game_over {
        return;
    }

    let spawn_ypos_threshold = 100.0;
    let mut rng = rand::thread_rng();
    use rand::Rng;

    if state.asteroid_timer.finished {
        let tx: f32 = rng.gen_range(
            bounds.point.x() - bounds.dimension.x() / 2.0,
            bounds.point.x() + bounds.dimension.x() / 2.0,
        );
        let ty = bounds.point.y() + bounds.dimension.y() / 2.0 + spawn_ypos_threshold;

        let base_speed = -100.0;
        let mut speed = (state.game_time / 70.0) * base_speed;
        if speed > base_speed {
            speed = base_speed
        }
        let vel = comp::physics::Velocity(Vec2::new(0.0, speed));

        let data = spawn::AsteroidDataComponents {
            translation: Translation::new(tx, ty, 0.0),
            velocity: vel,
            ..Default::default()
        };

        spawn::spawn_asteroid(&mut commands, &asset_server, &mut materials, data, None);

        state.asteroid_timer.reset();
    }

    if state.powerup_timer.finished {
        let tx: f32 = rng.gen_range(
            bounds.point.x() - bounds.dimension.x() / 2.0,
            bounds.point.x() + bounds.dimension.x() / 2.0,
        );
        let ty = bounds.point.y() + bounds.dimension.y() / 2.0 + spawn_ypos_threshold;

        let base_speed = -80.0;
        let mut speed = (state.game_time / 75.0) * base_speed;
        if speed > base_speed {
            speed = base_speed
        }
        let vel = comp::physics::Velocity(Vec2::new(0.0, speed));

        let data = spawn::PowerupDataComponents {
            translation: Translation::new(tx, ty, 1.0),
            velocity: vel,
            ..Default::default()
        };

        // Randomly select PowerUp kind.
        let mut rng = rand::thread_rng();
        //use rand::Rng;

        let select: u32 = rng.gen_range(0, 3);

        let kind = match select {
            0 => spawn::PowerUpKind::Speed(comp::stats::SpeedPowerUp {
                speed: comp::stats::MovementSpeed {
                    accel: 2.0,
                    max: 40.0,
                },
            }),
            1 => spawn::PowerUpKind::Health(comp::stats::HealthPowerUp {
                health: comp::stats::HealthStats {
                    hull: 90,
                    max_hull: 0,
                },
            }),
            _ => spawn::PowerUpKind::Energy(comp::stats::EnergyPowerUp {
                energy: comp::stats::EnergyStats {
                    energy: 200,
                    max_energy: 20,
                },
            }),
        };
        spawn::spawn_powerup(
            &mut commands,
            &asset_server,
            &mut materials,
            data,
            vec![kind],
            None,
        );

        state.powerup_timer.reset();
    }

    state.asteroid_timer.tick(time.delta_seconds);
    state.powerup_timer.tick(time.delta_seconds);
    state.game_time += time.delta_seconds;
}

/// Removes all AutoCleaned entities that leave bottom part of GameBounds
pub fn off_bounds_cleanup_system(
    bounds: Res<res::GameBounds>,
    mut dispose_events: ResMut<Events<res::DisposeEntityEvent>>,
    mut query: Query<(Entity, &comp::AutoCleaned, &Translation)>,
) {
    let threshold = 650.0; // To prevent "on-screen" cleanup. TODO: This is a hack.
    let half_y_bound = bounds.dimension.y() / 2.0;
    for (ent, _, translation) in &mut query.iter() {
        if translation.y() < bounds.point.y() - half_y_bound - threshold {
            dispose_events.send(res::DisposeEntityEvent::new(ent));
        }
    }
}

/// Checks if the game is over.
/// The game is over if the player is dead.
pub fn check_game_over_system(
    mut game_state: ResMut<res::GameplaySpawnState>,
    mut query: Query<(&comp::actor::Player, &comp::stats::CanDie)>,
) {
    if game_state.game_over {
        return;
    }

    for (_, death) in &mut query.iter() {
        if death.is_dead {
            game_state.game_over = true;
        }
    }
}

/// "Removes" graphical entities by translocating them "far, far away"
/// and removing most of their functional properties.
///
/// The system is called Charon, because in Greek mythology, Charon carried dead Away.
///
/// NOTE: This is a hack, as bevy_renderer is unable to handle removal/disabling
/// of an graphics component properly (ie. cannot handle despawning graphical entities).
/// DisposeEntityEvent instead marks an entity for the Charon system, that translocates
/// the entity "far, far away" and removes "functional" components from it,
/// effectively changing the entity into an offscreen sprite.
///
/// This has a downside that graphical entities will accumulate offscreen,
/// adding strain to the renderer and effectively leaking memory.
///
/// TODO: I believe that the bevy_render bug will get fixed soon and it will be possible to
/// remove the graphical entity/components. **This is a very short-term solution**,
/// unsuitable for "real life" application.
///
/// Alternative would be a full-blown entity pooling system,
/// to prevent the accumulation, but for now, this patches
/// the "delete a graphical entity = crash renderer" problem without much effort.
pub fn charon_system(
    mut commands: Commands,
    events: Res<Events<res::DisposeEntityEvent>>,
    mut state: ResMut<res::GraveyardState>,
) {
    for event in state.event_reader.iter(&events) {
        let entity = event.0;
        // Little debug info
        //println!("Removing {:?}", entity);
        // Carry it away
        commands.insert_one(entity, state.location);
        // Turn it into a ghost
        commands.remove_one::<comp::physics::Velocity>(entity);
        commands.remove_one::<comp::physics::ColliderBox>(entity);
        commands.remove_one::<comp::stats::Damage>(entity);
        commands.remove_one::<comp::stats::DamageOrigin>(entity);
        commands.remove_one::<comp::stats::DiesOnHit>(entity);
        commands.remove_one::<comp::stats::CanDie>(entity);
        commands.remove_one::<comp::stats::HealthStats>(entity);
        commands.remove_one::<comp::stats::EnergyStats>(entity);
        commands.remove_one::<comp::stats::EnergyRegen>(entity);
        commands.remove_one::<comp::stats::Dissipates>(entity);
        commands.remove_one::<comp::stats::Collectible>(entity);
        commands.remove_one::<comp::stats::SpeedPowerUp>(entity);
        commands.remove_one::<comp::stats::Score>(entity);
        commands.remove_one::<comp::actor::Weapon>(entity);

        commands.remove_one::<comp::AutoCleaned>(entity);

        state.population += 1;
    }
}
