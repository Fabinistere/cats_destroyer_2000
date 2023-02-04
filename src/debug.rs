use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::npc::NPC;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<NPC>()

                // UI
                ;
        }
    }
}
