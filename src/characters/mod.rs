use bevy::prelude::*;

pub mod effects;
pub mod movement;
pub mod npcs;
pub mod player;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            npcs::NPCsPlugin,
            player::PlayerPlugin,
            effects::EffectsPlugin,
        ));
    }
}
