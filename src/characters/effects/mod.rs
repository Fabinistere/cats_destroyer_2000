use bevy::prelude::*;

pub mod style;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            // -- Style --
            .add_systems((style::add_dazed_effect, style::animate_dazed_effect));
    }
}
