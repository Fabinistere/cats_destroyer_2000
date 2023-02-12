use bevy::prelude::*;

use self::{effects::EffectsPlugin, npcs::NPCsPlugin, player::PlayerPlugin};

pub mod effects;
pub mod movement;
pub mod npcs;
pub mod player;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_plugin(NPCsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EffectsPlugin)
            ;
    }
}
