use crate::characters::effects::style::{add_dazed_effect, animate_dazed_effect};
use bevy::prelude::*;

pub mod style;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // -- Style --
            .add_system(add_dazed_effect)
            .add_system(animate_dazed_effect)
            ;
    }
}
