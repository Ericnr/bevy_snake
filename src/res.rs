//! All resources and events used by the systems are here.
use bevy::prelude::*;

pub const GAME_BOUNDS_DIMENSION_W: f32 = 720.0;
pub const GAME_BOUNDS_DIMENSION_H: f32 = 680.0;

/// Resource to keep player within area
#[derive(Debug, Default)]
pub struct GameBounds {
    pub point: Vec2,
    pub dimension: Vec2,
}

/// Resource to keep player within area
#[derive(Debug)]
pub struct GameplaySpawnState {
    pub asteroid_timer: Timer,
    pub powerup_timer: Timer,
    pub game_time: f32,
    pub game_over: bool,
}

impl Default for GameplaySpawnState {
    fn default() -> Self {
        GameplaySpawnState {
            asteroid_timer: Timer::from_seconds(1.7),
            powerup_timer: Timer::from_seconds(11.0),
            game_time: 0.0,
            game_over: false,
        }
    }
}

/// Collision between two entities.
/// The order of enitities inside the event is not
/// important when comparing CollisionEvents
#[derive(Debug, Clone)]
pub struct CollisionEvent(pub Entity, pub Entity);

impl CollisionEvent {
    pub fn new(e1: Entity, e2: Entity) -> Self {
        CollisionEvent(e1, e2)
    }
}

impl PartialEq for CollisionEvent {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

/// Resource for contact_damage_system,
/// used to read collisions.
#[derive(Default)]
pub struct ContactDamageListenerState {
    pub event_reader: EventReader<CollisionEvent>,
}

/// Resource for detect_pickup_system,
/// used to read collisions.
#[derive(Default)]
pub struct ContactPickupListenerState {
    pub event_reader: EventReader<CollisionEvent>,
}

/// DisposeEntityEvents are used for removing entities with graphics.
///
/// NOTE: This is a hack, as bevy_renderer is unable to handle removal/disabling
/// of an graphics component properly (ie. cannot handle despawning graphical entities).
/// This event instead marks an entity for the Charon system, that translocates
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
#[derive(Debug)]
pub struct DisposeEntityEvent(pub Entity);

impl DisposeEntityEvent {
    pub fn new(e: Entity) -> Self {
        DisposeEntityEvent(e)
    }
}

/// Resource for the charon_system.
#[derive(Default)]
pub struct GraveyardState {
    pub event_reader: EventReader<DisposeEntityEvent>,
    pub population: u32,
    pub location: Translation,
}

/// Event informing that src damaged targ.
#[derive(Debug)]
pub struct DamageEvent {
    pub src: Entity,
    pub targ: Entity,
}

impl DamageEvent {
    pub fn new(src: Entity, targ: Entity) -> Self {
        DamageEvent { src, targ }
    }
}

/// Event informing that src killed targ.
///
/// TODO: Is this event safe? Is it ensured that on delivery,
/// both src and targ will still be valid?
#[derive(Debug)]
pub struct KillEvent {
    pub src: Entity,
    pub targ: Entity,
}

impl KillEvent {
    pub fn new(src: Entity, targ: Entity) -> Self {
        KillEvent { src, targ }
    }
}

/// Resource for the on_hit_die_system.
#[derive(Default)]
pub struct OnHitDieState {
    pub event_reader: EventReader<DamageEvent>,
}

/// Resource for the on_kill_score_system.
#[derive(Default)]
pub struct OnKillScoreState {
    pub event_reader: EventReader<KillEvent>,
}

/// Event informing that `collector` entity picked up `pickup` entity.
#[derive(Debug)]
pub struct PickupEvent {
    pub collector: Entity,
    pub pickup: Entity,
}

impl PickupEvent {
    pub fn new(collector: Entity, pickup: Entity) -> Self {
        PickupEvent { collector, pickup }
    }
}

/// Resource for the apply_speed_powerup_system.
#[derive(Default)]
pub struct ApplySpeedPowerUpState {
    pub event_reader: EventReader<PickupEvent>,
}

/// Resource for the apply_health_powerup_system.
#[derive(Default)]
pub struct ApplyHealthPowerUpState {
    pub event_reader: EventReader<PickupEvent>,
}

/// Resource for the apply_energy_powerup_system.
#[derive(Default)]
pub struct ApplyEnergyPowerUpState {
    pub event_reader: EventReader<PickupEvent>,
}

/// Event passing information to all  entities Contolled by `src`.
///
/// Response of the entity is defined by it's components (related systems).
#[derive(Debug)]
pub struct EntityCommandEvent {
    pub src: Entity,
    pub command: String,
    /// TODO: This is hack, because Bevy does not easily (seamlessly) provide
    /// relative-to-world translations in a hierarchy.
    pub src_loc: Vec2,
}

impl EntityCommandEvent {
    pub fn new(src: Entity, command: String, src_loc: Vec2) -> Self {
        EntityCommandEvent {
            src,
            command,
            src_loc,
        }
    }
}

pub const ENTITY_COMMAND_SHOOT: &str = "shoot";

/// Resource for the weapon_shoot_system.
#[derive(Default)]
pub struct WeaponShootCommandListenerState {
    pub event_reader: EventReader<EntityCommandEvent>,
}

/// DeveloperCommandEvents are used for dispatching developer commands.
///
/// The `command` field contains the actual developer command.
#[derive(Debug)]
pub struct DeveloperCommandEvent {
    pub command: String,
}

/// Resource for developer_executive_system
#[derive(Default)]
pub struct DeveloperExecutiveState {
    pub developer_event_reader: EventReader<DeveloperCommandEvent>,
    pub cursor_event_reader: EventReader<CursorMoved>,
    pub last_cursor_pos: Vec2,
}

/// This is an identifier providing information about
/// what should be spawned.
///
/// TODO: This could be potentially dehardcoded.
#[derive(Debug)]
pub enum SpawnType {
    Asteroid,
    Projectile,
    Powerup,
}

/// SpawnEntityEvent are used for spawning game entities.
#[derive(Debug)]
pub struct SpawnEntityEvent {
    pub spawn_type: SpawnType,
    pub location: Translation,
    pub velocity: Option<crate::comp::physics::Velocity>,
    pub origin: Option<Entity>,
}

/// Resource for generic_spawn_entity_system
#[derive(Default)]
pub struct GenericSpawnEntityState {
    pub representative: Option<Entity>,
    pub event_reader: EventReader<SpawnEntityEvent>,
}
