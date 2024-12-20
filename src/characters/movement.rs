use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
// use bevy_retrograde::prelude::Velocity;

use crate::TILE_SIZE;

// find the right place to put this component (indicator)
#[derive(Component)]
pub struct CharacterHitbox;

#[derive(Component)]
pub struct Dazed {
    /// should be a non-repeating timer
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(50. * TILE_SIZE)
    }
}

#[derive(Bundle)]
#[allow(clippy::module_name_repetitions)]
pub struct MovementBundle {
    pub speed: Speed,
    pub velocity: Velocity,
}
