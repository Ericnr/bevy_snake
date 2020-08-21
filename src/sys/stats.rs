use crate::comp::actor;
use crate::comp::physics;
use crate::comp::stats;
use crate::res;
use crate::spawn;

use bevy::prelude::*;

/// Plugin bringing in mostly stats manipulating systems and resources
pub struct GameStatsPlugin;

impl Plugin for GameStatsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<res::DamageEvent>()
            .add_event::<res::PickupEvent>()
            .add_event::<res::KillEvent>()
            .init_resource::<res::ContactDamageListenerState>()
            .init_resource::<res::ContactPickupListenerState>()
            .init_resource::<res::OnHitDieState>()
            .init_resource::<res::OnKillScoreState>()
            .init_resource::<res::ApplySpeedPowerUpState>()
            .init_resource::<res::ApplyHealthPowerUpState>()
            .init_resource::<res::ApplyEnergyPowerUpState>()
            .add_system_to_stage(stage::PRE_UPDATE, contact_damage_system.system())
            .add_system(detect_pickup_system.system())
            .add_system(timed_dissipation_system.system())
            .add_system(apply_speed_powerup_system.system())
            .add_system(apply_health_powerup_system.system())
            .add_system(apply_energy_powerup_system.system())
            .add_system(regen_energy_system.system())
            .add_system_to_stage(stage::POST_UPDATE, collectible_cleanup_system.system())
            .add_system_to_stage(stage::POST_UPDATE, on_hit_die_system.system())
            .add_system_to_stage(stage::POST_UPDATE, dead_cleanup_system.system())
            .add_system_to_stage(stage::POST_UPDATE, on_kill_score_system.system())
            .add_system_to_stage(stage::POST_UPDATE, death_effect_particles_system.system());
    }
}

/// Processes damage on contact of two entities.
///
/// TODO: Make it less terrific, ideally by only querying components
/// of collided entities, without iterating over entities.
pub fn contact_damage_system(
    mut damage_events: ResMut<Events<res::DamageEvent>>,
    mut kill_events: ResMut<Events<res::KillEvent>>,
    collision_events: Res<Events<res::CollisionEvent>>,
    mut collision_event_reader: ResMut<res::ContactDamageListenerState>,
    mut damages: Query<(Entity, &stats::Damage)>,
    mut damageables: Query<(Entity, &mut stats::HealthStats, &mut stats::CanDie)>,
    damage_sources: Query<(Entity, &stats::DamageOrigin)>,
) {
    for event in collision_event_reader.event_reader.iter(&collision_events) {
        for (src_ent, damage) in &mut damages.iter() {
            if src_ent == event.0 || src_ent == event.1 {
                for (hit_ent, mut health, mut death) in &mut damageables.iter() {
                    if (hit_ent == event.0 || hit_ent == event.1) && hit_ent != src_ent {
                        // TODO: For reasons unknown to me, this query fails with `CannotReadArchetype` the
                        // first time if a projectile is fired AND this system runs in the UPDATE stage.
                        // After the first time, it works as expected, however.
                        // I moved this system into PRE_UPDATE because of this
                        let dmg_source = damage_sources.get::<stats::DamageOrigin>(src_ent);
                        let mut source_exists = None;
                        let hit_origin = if let Ok(origin) = dmg_source {
                            source_exists = Some(origin.entity);
                            hit_ent == origin.entity
                        } else {
                            false
                        };
                        if !hit_origin {
                            // Little debug info
                            //println!("Damage detected!");
                            if health.hull <= damage.hull {
                                health.hull = 0;
                                damage_events.send(res::DamageEvent::new(src_ent, hit_ent));
                                // verify execution flow of this
                                if let Some(source) = source_exists {
                                    if !death.is_dead {
                                        kill_events.send(res::KillEvent::new(source, hit_ent))
                                    }
                                };
                                death.is_dead = true;
                            } else {
                                health.hull -= damage.hull;
                                damage_events.send(res::DamageEvent::new(src_ent, hit_ent));
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Increases dissipation timer on entities that Dissipate
/// and removes the entity if the timer finishes.
pub fn timed_dissipation_system(
    time: Res<Time>,
    mut dispose_events: ResMut<Events<res::DisposeEntityEvent>>,
    mut query: Query<(Entity, &mut stats::Dissipates)>,
) {
    for (ent, mut dissipates) in &mut query.iter() {
        if dissipates.timer.finished {
            dispose_events.send(res::DisposeEntityEvent::new(ent));
        } else {
            dissipates.timer.tick(time.delta_seconds);
        }
    }
}

/// Despawns `DiesOnHit` entities that got hitted a certain number of times
/// and actualizes this count.
pub fn on_hit_die_system(
    damage_events: ResMut<Events<res::DamageEvent>>,
    mut dispose_events: ResMut<Events<res::DisposeEntityEvent>>,
    mut state: ResMut<res::OnHitDieState>,
    mut query: Query<(Entity, &mut stats::DiesOnHit)>,
) {
    for event in state.event_reader.iter(&damage_events) {
        for (ent, hitted) in &mut query.iter() {
            if ent == event.src {
                if hitted.hits_to_die - 1 <= 0 {
                    dispose_events.send(res::DisposeEntityEvent::new(ent));
                }
            }
        }
    }
}

/// Detects pickup on contact of two entities.
///
/// TODO: Make it less terrific, ideally by only querying components
/// of collided entities, without iterating over entities.
pub fn detect_pickup_system(
    mut pickup_events: ResMut<Events<res::PickupEvent>>,
    collisions: Res<Events<res::CollisionEvent>>,
    mut collision_reader: ResMut<res::ContactPickupListenerState>,
    mut collectors: Query<(Entity, &stats::Collector)>,
    mut pickups: Query<(Entity, &mut stats::Collectible)>,
) {
    for event in collision_reader.event_reader.iter(&collisions) {
        for (collector_ent, _) in &mut collectors.iter() {
            if collector_ent == event.0 || collector_ent == event.1 {
                for (pickup_ent, mut pickup) in &mut pickups.iter() {
                    if (pickup_ent == event.0 || pickup_ent == event.1)
                        && pickup_ent != collector_ent
                    {
                        // Little debug info
                        //println!("Pickup detected!");
                        pickup_events.send(res::PickupEvent::new(collector_ent, pickup_ent));
                        pickup.collected = true;
                    }
                }
            }
        }
    }
}

/// Applies "Speed" PowerUp effects.
pub fn apply_speed_powerup_system(
    pickups: ResMut<Events<res::PickupEvent>>,
    mut pickup_reader: ResMut<res::ApplySpeedPowerUpState>,
    mut powerups: Query<(Entity, &stats::SpeedPowerUp)>,
    mut collectors: Query<(Entity, &mut stats::MovementSpeed)>,
) {
    for event in pickup_reader.event_reader.iter(&pickups) {
        for (collector_ent, mut speed) in &mut collectors.iter() {
            if collector_ent == event.collector {
                for (pickup_ent, powerup) in &mut powerups.iter() {
                    if pickup_ent == event.pickup {
                        speed.accel += powerup.speed.accel;
                        speed.max += powerup.speed.max;
                    }
                }
            }
        }
    }
}

/// Applies "Health" PowerUp effects.
pub fn apply_health_powerup_system(
    pickups: ResMut<Events<res::PickupEvent>>,
    mut pickup_reader: ResMut<res::ApplyHealthPowerUpState>,
    mut powerups: Query<(Entity, &stats::HealthPowerUp)>,
    mut collectors: Query<(Entity, &mut stats::HealthStats)>,
) {
    for event in pickup_reader.event_reader.iter(&pickups) {
        for (collector_ent, mut stats) in &mut collectors.iter() {
            if collector_ent == event.collector {
                for (pickup_ent, powerup) in &mut powerups.iter() {
                    if pickup_ent == event.pickup {
                        stats.max_hull += powerup.health.max_hull;
                        if stats.hull + powerup.health.hull > stats.max_hull {
                            stats.hull = stats.max_hull;
                        } else {
                            stats.hull += powerup.health.hull;
                        }
                    }
                }
            }
        }
    }
}

/// Applies "Energy" PowerUp effects.
pub fn apply_energy_powerup_system(
    pickups: ResMut<Events<res::PickupEvent>>,
    mut pickup_reader: ResMut<res::ApplyEnergyPowerUpState>,
    mut powerups: Query<(Entity, &stats::EnergyPowerUp)>,
    mut collectors: Query<(Entity, &mut stats::EnergyStats)>,
) {
    for event in pickup_reader.event_reader.iter(&pickups) {
        for (collector_ent, mut stats) in &mut collectors.iter() {
            if collector_ent == event.collector {
                for (pickup_ent, powerup) in &mut powerups.iter() {
                    if pickup_ent == event.pickup {
                        stats.max_energy += powerup.energy.max_energy;
                        if stats.energy + powerup.energy.energy > stats.max_energy {
                            stats.energy = stats.max_energy;
                        } else {
                            stats.energy += powerup.energy.energy;
                        }
                    }
                }
            }
        }
    }
}

/// Periodically changes EnergyStats if entity has EnergyRegen.
///
/// TODO: Currently, the system also ticks EnergyRegen timer. This might change.
pub fn regen_energy_system(
    time: Res<Time>,
    mut query: Query<(&mut stats::EnergyStats, &mut stats::EnergyRegen)>,
) {
    for (mut stats, mut regen) in &mut query.iter() {
        if regen.cycle.finished {
            if regen.energy + stats.energy > stats.max_energy {
                stats.energy = stats.max_energy;
            } else {
                stats.energy += regen.energy;
            }
            regen.cycle.reset();
        }
        regen.cycle.tick(time.delta_seconds);
    }
}

/// Removes `collected` `Collectibles`.
///
/// TODO: Check execution order, for it might be possible that
/// the `Collectible` gets removed before it's powerups get applied.
pub fn collectible_cleanup_system(
    mut dispose_events: ResMut<Events<res::DisposeEntityEvent>>,
    mut collectibles: Query<(Entity, &stats::Collectible)>,
) {
    for (ent, collectible) in &mut collectibles.iter() {
        if collectible.collected {
            dispose_events.send(res::DisposeEntityEvent::new(ent));
        }
    }
}

/// Removes `is_dead` entities that `CanDie`.
///
/// TODO: Check execution order, review the `CanDie` component
/// and entity de/spawning in general.
pub fn dead_cleanup_system(
    mut dispose_events: ResMut<Events<res::DisposeEntityEvent>>,
    mut deaths: Query<(Entity, &stats::CanDie)>,
) {
    for (ent, death) in &mut deaths.iter() {
        if death.is_dead {
            dispose_events.send(res::DisposeEntityEvent::new(ent));
        }
    }
}

/// Produces particles on death of an entities that `CanDie`
/// and has `DeathEffectParticles`
pub fn death_effect_particles_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut deaths: Query<(&stats::CanDie, &stats::DeathEffectParticles, &Translation)>,
) {
    for (death, effect, translation) in &mut deaths.iter() {
        if death.is_dead {
            if effect.effect == stats::DEATH_EFFECT_PARTICLES_ASTEROID {
                let mut rng = rand::thread_rng();
                use rand::Rng;

                for i in 0..5 {
                    let vx: f32 = rng.gen_range(-40.0, 40.0);
                    let vy: f32 = rng.gen_range(-40.0, 40.0);

                    let vel = physics::Velocity(Vec2::new(vx, vy));
                    let mut translation = *translation;
                    // to fix the broken transparency, so the particles do not have visible rectangle bounds around them
                    translation.set_z(i as f32 / 100.0);
                    let data = spawn::AsteroidParticleDataComponents {
                        translation,
                        velocity: vel,
                        ..Default::default()
                    };

                    spawn::spawn_asteroid_particle(
                        &mut commands,
                        &asset_server,
                        &mut materials,
                        data,
                        None,
                    );
                }
            }
        }
    }
}

/// Increments player score on death of entity with `Score`.
///
/// TODO: Review the `KillEvent` and the `CanDie` component.
/// I suspect there are some race conditions here.
pub fn on_kill_score_system(
    events: Res<Events<res::KillEvent>>,
    mut state: ResMut<res::OnKillScoreState>,
    mut deaths: Query<(Entity, &stats::CanDie, &stats::Score)>,
    mut players: Query<(Entity, &mut actor::Player)>,
) {
    for event in state.event_reader.iter(&events) {
        for (dead_ent, death, score) in &mut deaths.iter() {
            if dead_ent == event.targ && death.is_dead {
                for (killer_ent, mut player) in &mut players.iter() {
                    if killer_ent == event.src {
                        player.score += score.score;
                    }
                }
            }
        }
    }
}
