use bevy::prelude::*;

use crate::locations::Location;

pub mod style;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            // -- Style --
            .add_systems(
                Update,
                (style::add_dazed_effect, style::animate_dazed_effect)
                    // REFACTOR: create a new state `enum GameState { Playing, Cinematic }`
                    .run_if(in_state(Location::Level1000)),
            );
    }
}
