use bevy::prelude::*;

use self::{
    cinematics::{animate_clouds, animate_free_cat, cinematic_camera, spawn_cinematic_final},
    sensors::WinTriggerEvent,
};

pub mod cinematics;
pub mod level_one;
pub mod sensors;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Component, States)]
pub enum Location {
    /// The transition universe
    /// Used when reset a Level
    Void,
    StartCinematic,
    #[default]
    Level1000,
    LevelTwo,
    LevelOne,
    // LevelZero,
    // Exit cinematic
    OutDoor,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WinTriggerEvent>()
            .add_state::<Location>()
            .add_plugin(level_one::LevelOnePlugin)
            .add_systems((
                sensors::win_trigger,
                sensors::win_event,
                sensors::location_event,
                sensors::button_event,
            ))
            .add_systems(
                (spawn_cinematic_final, cinematic_camera).in_schedule(OnEnter(Location::OutDoor)),
            )
            .add_systems((animate_clouds, animate_free_cat).in_set(OnUpdate(Location::OutDoor)));
    }
}
