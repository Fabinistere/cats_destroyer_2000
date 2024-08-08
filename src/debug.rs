use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    characters::npcs::NPC,
    locations::level_one::{CharacterLocation, Level1000Location},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins(WorldInspectorPlugin::new())
                .register_type::<NPC>()
                .register_type::<CharacterLocation>()
                .register_type::<Level1000Location>()

                // UI
                ;
        }
    }
}
