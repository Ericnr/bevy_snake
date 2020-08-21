use bevy::prelude::*;
use std::collections::VecDeque;

/// This component represents a player and an associated information
#[derive(Debug, Default, Properties)]
pub struct Player {
    pub score: i32,
}

/// This component represents a weapon and an associated information.
///
/// Entity with Weapon component can emit a
/// projectile (based on kind) if the weapon is `reload`ed.
///
/// NOTE: The Weapon component is intended to be separate from
/// the entity which actually have the weapon (owner, ie. ship) and
/// being a child of its owner.
/// This is to allow more flexible design, paired with `Controlled` component,
/// to easily allow things like semi-autonomous drones, but maybe this is
/// unnecesarily complex design and *might change in the future*.
#[derive(Debug, Default, Properties)]
pub struct Weapon {
    pub kind: String,
    pub reload: Timer,
    pub energy_drain: i32,
}

/// Component used for abstract input.
/// Both player and AIs are communicationg with their Controllers, rather than directly with underlying systems.
/// This can make systems player vs AI agnostic and set the same "conditions" for both humans and AIs.
#[derive(Debug, Default)]
pub struct Controller {
    pub movement: Vec2,
    pub action: VecDeque<ControllerAction>,
}

#[derive(Debug)]
pub enum ControllerAction {
    Shoot,
}

/// Controlled component responds to events originating
/// from entity that controls this entity (component).
///
/// This allows to dispatch events without actual parent-child
/// hierarchy relationship.
///
/// NOTE: This exists because of `Weapon` design. If it proves unnecesearily
/// complex, this might get removed.
#[derive(Debug, Properties)]
pub struct Controlled {
    pub by: Entity,
}
