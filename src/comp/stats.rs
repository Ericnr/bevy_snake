use bevy::prelude::*;

/// This component causes damage.
///
/// TODO: Maybe split this into components for each damage type when implemented?
#[derive(Debug, Default, Properties)]
pub struct Damage {
    pub hull: i32,
}

/// This component provides information about the source/origin of the damage,
/// used to prevent "my projectile hit me" scenarios.
///
/// TODO: Maybe add groups for friendly-fire scenarios?
/// Or maybe even add such functionality to collision system?
#[derive(Debug, Properties)]
pub struct DamageOrigin {
    pub entity: Entity,
}

/// This component marks entity as a subject of death.
///
/// TODO: Move this to [HealthStats] or something?
/// I am not sure when component additio/removal is
/// handled and same goes for event updates.
/// I believe that if `is_dead` is true, any POST_UPDATE systems
/// can react with things like actual component removal or
/// "particles on death". This design will need a review eventually.
#[derive(Debug, Default, Properties)]
pub struct CanDie {
    pub is_dead: bool,
}

/// Helper constant to simulate "particle type".
pub const DEATH_EFFECT_PARTICLES_ASTEROID: &str = "asteroid";

/// This component makes entity to produce particles on death.
#[derive(Debug, Properties)]
pub struct DeathEffectParticles {
    pub effect: String,
}

/// This component represents entity's health.
#[derive(Debug, Default, Properties)]
pub struct HealthStats {
    pub hull: i32,
    pub max_hull: i32,
}

/// This component represents movement stats.
#[derive(Debug, Default, Properties)]
pub struct MovementSpeed {
    pub accel: f32,
    pub max: f32,
}

/// This component represents entity's energy.
#[derive(Debug, Default, Properties)]
pub struct EnergyStats {
    pub energy: i32,
    pub max_energy: i32,
}

/// This component allows entity to regenerate energy.
///
/// TODO: Change `energy` to f32 and remove the `cycle` Timer??
#[derive(Debug, Default, Properties)]
pub struct EnergyRegen {
    pub energy: i32,
    pub cycle: Timer,
}

/// Component providing information that entity dissipates
/// and it will stop existing after some time.
///
/// NOTE: It might die sooner, if the dissipation would "damage" the entity
#[derive(Debug, Default, Properties)]
pub struct Dissipates {
    pub timer: Timer,
}

/// Component providing information that entity dies after it **is hit**
/// one or more times (defined by hits_to_die)
///
/// TODO: Maybe put this into [HealthStats]??
#[derive(Debug, Default, Properties)]
pub struct DiesOnHit {
    pub hits_to_die: i32,
}

/// This component allows entity to collect [Collectible] entities on touch.
#[derive(Debug, Default, Properties)]
pub struct Collector;

/// This component makes entity collectible.
///
/// `collected` is flag for post-pickup systems.
#[derive(Debug, Default, Properties)]
pub struct Collectible {
    pub collected: bool,
}

/// This component affects [MovementSpeed] stats of entities.
#[derive(Debug, Default, Properties)]
pub struct SpeedPowerUp {
    pub speed: MovementSpeed,
}

/// This component affects [HealthStats] stats of entities.
#[derive(Debug, Default, Properties)]
pub struct HealthPowerUp {
    pub health: HealthStats,
}

/// This component affects [EnergyStats] stats of entities.
#[derive(Debug, Default, Properties)]
pub struct EnergyPowerUp {
    pub energy: EnergyStats,
}

/// Represents that an entity has a score.
#[derive(Debug, Default, Properties)]
pub struct Score {
    pub score: i32,
}
