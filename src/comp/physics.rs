use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

/// This component represents entity's velocity.
#[derive(Clone, Debug, Default, Properties)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Vec2::new(x, y))
    }
}

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// This component represents how much "drag" affects this entity.
#[derive(Debug, Default, Properties)]
pub struct Drag(pub f32);

/// This component represents primitive box collider around an entity.
#[derive(Debug, Default, Properties)]
pub struct ColliderBox {
    pub w: u32,
    pub h: u32,
}
