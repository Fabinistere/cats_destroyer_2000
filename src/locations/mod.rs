use bevy::prelude::*;

use self::{
    cinematics::{
        animate_clouds,
        animate_free_cat,
        cinematic_camera,
        spawn_cinematic_final,
    },
    sensors::{
        button_event,
        location_event,
        win_trigger,
        win_event,
        WinTriggerEvent
    },
};

pub mod cinematics;
pub mod level_one;
pub mod sensors;

#[derive(Component, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Location {
    // /// The transition universe
    // /// Used when reset a Level
    // Void,
    // Start cinematic
    Level1000,
    LevelTwo,
    LevelOne,
    // LevelZero,
    // Exit cinematic
    OutDoor,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_event::<WinTriggerEvent>()

            .add_plugin(level_one::LevelOnePlugin)

            .add_state(Location::Level1000)

            .add_system(win_trigger)
            .add_system(win_event)
            .add_system(location_event)
            .add_system(button_event)
            
            .add_system_set(
                SystemSet::on_enter(Location::OutDoor)
                    .with_system(spawn_cinematic_final)
                    .with_system(cinematic_camera)
            )
            .add_system_set(
                SystemSet::on_update(Location::OutDoor)
                    .with_system(animate_clouds)
                    .with_system(animate_free_cat)
            )
            ;
    }
}
