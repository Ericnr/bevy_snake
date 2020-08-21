//! This file contains everything entity spawning related.
//!
//! Because Bevy's scene instancing is currently problematic
//! (graphical component information fail to deserialize),
//! everything here is essentially a workaround around the lack of scenes (prefabs).
//!
//! TODO: Once Bevy's scene loading gets more robust,
//! this file should be reviewed/removed.

use crate::comp;

use bevy::prelude::*;

/// This component is simply a label for a spawner.
///
/// TODO: Remove this hack.
#[derive(Debug, Default)]
pub struct SpawnerName(String);

/// This is a pseudo-actor, representing a spawner.
///
/// TODO: Change it from a hack into something valid.
#[derive(Bundle, Default)]
pub struct Spawner {
    pub name: SpawnerName,
}

/// Component bundle defining logical properties of an asteroid.
#[derive(Bundle)]
pub struct AsteroidDataComponents {
    pub translation: Translation,
    pub collider: comp::physics::ColliderBox,
    pub velocity: comp::physics::Velocity,
    pub health: comp::stats::HealthStats,
    pub damage: comp::stats::Damage,
}

impl Default for AsteroidDataComponents {
    fn default() -> Self {
        AsteroidDataComponents {
            translation: Translation::default(),
            collider: comp::physics::ColliderBox { w: 80, h: 80 },
            velocity: comp::physics::Velocity(Vec2::new(0.0, 0.0)),
            health: comp::stats::HealthStats {
                hull: 100,
                max_hull: 100,
            },
            damage: comp::stats::Damage { hull: 5 },
        }
    }
}

/// Component bundle defining logical properties of a projectile.
#[derive(Bundle)]
pub struct ProjectileDataComponents {
    pub translation: Translation,
    pub collider: comp::physics::ColliderBox,
    pub velocity: comp::physics::Velocity,
    pub damage: comp::stats::Damage,
    pub dissipates: comp::stats::Dissipates,
}

impl Default for ProjectileDataComponents {
    fn default() -> Self {
        ProjectileDataComponents {
            translation: Translation::default(),
            collider: comp::physics::ColliderBox { w: 35, h: 35 },
            velocity: comp::physics::Velocity(Vec2::new(0.0, 0.0)),
            damage: comp::stats::Damage { hull: 50 },
            dissipates: comp::stats::Dissipates {
                timer: Timer::from_seconds(2.0),
            },
        }
    }
}

/// Component bundle defining *common* logical properties of a powerup.
#[derive(Bundle)]
pub struct PowerupDataComponents {
    pub translation: Translation,
    pub collider: comp::physics::ColliderBox,
    pub velocity: comp::physics::Velocity,
    pub collectible: comp::stats::Collectible,
}

impl Default for PowerupDataComponents {
    fn default() -> Self {
        PowerupDataComponents {
            translation: Translation::default(),
            collider: comp::physics::ColliderBox { w: 120, h: 20 },
            velocity: comp::physics::Velocity(Vec2::new(0.0, -40.0)),
            collectible: comp::stats::Collectible::default(),
        }
    }
}

/// Component bundle defining logical properties of an asteroid particle.
#[derive(Bundle)]
pub struct AsteroidParticleDataComponents {
    pub translation: Translation,
    pub velocity: comp::physics::Velocity,
    pub dissipates: comp::stats::Dissipates,
}

impl Default for AsteroidParticleDataComponents {
    fn default() -> Self {
        AsteroidParticleDataComponents {
            translation: Translation::default(),
            velocity: comp::physics::Velocity(Vec2::new(0.0, 0.0)),
            dissipates: comp::stats::Dissipates {
                timer: Timer::from_seconds(1.5),
            },
        }
    }
}

/// Spawns an asteroid from data and graphical bundles.
///
/// The graphical bundle is optional and "default asteroid" will be
/// used if None is passed.
///
/// NOTE: Sprite's Tranlocation will be overriden by that in `data` bundle.
pub fn spawn_asteroid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    data: AsteroidDataComponents,
    sprite: Option<SpriteComponents>,
) -> Entity {
    if let Some(sprite) = sprite {
        commands.spawn(sprite);
    } else {
        let mut rng = rand::thread_rng();
        use rand::Rng;

        // make the asteroid look a bit varied
        let select: u32 = rng.gen_range(0, 4);
        let texture_handle = match select {
            0 => asset_server
                .load("assets/textures/asteroid/brown_big1.png")
                .unwrap(),
            1 => asset_server
                .load("assets/textures/asteroid/brown_big2.png")
                .unwrap(),
            2 => asset_server
                .load("assets/textures/asteroid/brown_big3.png")
                .unwrap(),
            _ => asset_server
                .load("assets/textures/asteroid/brown_big4.png")
                .unwrap(),
        };
        let rot: f32 = rng.gen_range(-1.0, 1.0);

        let new_sprite = SpriteComponents {
            material: materials.add(texture_handle.into()),
            scale: Scale(1.1),
            rotation: Rotation(Quat::from_rotation_z(rot)),
            ..Default::default()
        };
        commands.spawn(new_sprite);
    }
    commands
        .with_bundle(data)
        .with(comp::AutoCleaned)
        .with(comp::stats::CanDie::default())
        .with(comp::stats::Score { score: 100 })
        .with(comp::stats::DeathEffectParticles {
            effect: String::from(comp::stats::DEATH_EFFECT_PARTICLES_ASTEROID),
        });

    commands.current_entity().unwrap()
}

/// Spawns a projectile from data and graphical bundles.
///
/// The graphical bundle is optional and "default projectile" will be
/// used if None is passed.
///
/// NOTE: Sprite's Tranlocation will be overriden by that in `data` bundle.
pub fn spawn_projectile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    origin: Entity,
    data: ProjectileDataComponents,
    sprite: Option<SpriteComponents>,
) -> Entity {
    if let Some(sprite) = sprite {
        commands.spawn(sprite);
    } else {
        let texture_handle = asset_server
            .load("assets/textures/projectile/nova_blast.png")
            .unwrap();
        let new_sprite = SpriteComponents {
            material: materials.add(texture_handle.into()),
            scale: Scale(1.0),
            ..Default::default()
        };
        commands.spawn(new_sprite);
    }
    commands
        .with_bundle(data)
        //.with(comp::actor::Projectile)
        .with(comp::stats::DiesOnHit { hits_to_die: 1 })
        .with(comp::AutoCleaned)
        .with(comp::stats::DamageOrigin { entity: origin });

    commands.current_entity().unwrap()
}

/// This enum wraps powerup effect arguments for [spawn_powerup].
pub enum PowerUpKind {
    Speed(comp::stats::SpeedPowerUp),
    Health(comp::stats::HealthPowerUp),
    Energy(comp::stats::EnergyPowerUp),
}

/// Spawns a powerup from basic data, PowerUp effects and graphical bundles.
///
/// The graphical bundle is optional and "default powerup" will be
/// used if None is passed.
///
/// NOTE: Sprite's Tranlocation will be overriden by that in `data` bundle.
pub fn spawn_powerup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    data: PowerupDataComponents,
    kinds: Vec<PowerUpKind>,
    sprite: Option<SpriteComponents>,
) -> Entity {
    if let Some(sprite) = sprite {
        commands.spawn(sprite);
    } else {
        // Select powerup sprite from 1st kind passed.
        // This is purely cosmetic and could be removed.
        let sprite_asset = match kinds.first().unwrap() {
            PowerUpKind::Speed(_) => "assets/textures/powerup/speed.png",
            PowerUpKind::Health(_) => "assets/textures/powerup/health.png",
            PowerUpKind::Energy(_) => "assets/textures/powerup/energy.png",
        };
        let texture_handle = asset_server.load(sprite_asset).unwrap();
        let new_sprite = SpriteComponents {
            material: materials.add(texture_handle.into()),
            scale: Scale(1.5),
            ..Default::default()
        };
        commands.spawn(new_sprite);
    }
    commands.with_bundle(data).with(comp::AutoCleaned);
    // Adds "effect" components to this powerup.
    for kind in kinds {
        match kind {
            PowerUpKind::Speed(c) => {
                commands.with(c);
            }
            PowerUpKind::Health(c) => {
                commands.with(c);
            }
            PowerUpKind::Energy(c) => {
                commands.with(c);
            }
        }
    }

    commands.current_entity().unwrap()
}

/// Spawns an asteroid particle from data and graphical bundles.
///
/// The graphical bundle is optional and "default projectile" will be
/// used if None is passed.
///
/// NOTE: Sprite's Tranlocation will be overriden by that in `data` bundle.
pub fn spawn_asteroid_particle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    data: AsteroidParticleDataComponents,
    sprite: Option<SpriteComponents>,
) -> Entity {
    if let Some(sprite) = sprite {
        commands.spawn(sprite);
    } else {
        let mut rng = rand::thread_rng();
        use rand::Rng;

        // make the asteroid particle look a bit varied
        let select: u32 = rng.gen_range(0, 4);
        let texture_handle = match select {
            0 => asset_server
                .load("assets/textures/asteroid/brown_small1.png")
                .unwrap(),
            1 => asset_server
                .load("assets/textures/asteroid/brown_small2.png")
                .unwrap(),
            2 => asset_server
                .load("assets/textures/asteroid/brown_tiny1.png")
                .unwrap(),
            _ => asset_server
                .load("assets/textures/asteroid/brown_tiny2.png")
                .unwrap(),
        };
        let rot: f32 = rng.gen_range(-1.0, 1.0);

        let new_sprite = SpriteComponents {
            material: materials.add(texture_handle.into()),
            scale: Scale(1.1),
            rotation: Rotation(Quat::from_rotation_z(rot)),
            ..Default::default()
        };
        commands.spawn(new_sprite);
    }
    commands.with_bundle(data).with(comp::AutoCleaned);

    commands.current_entity().unwrap()
}
