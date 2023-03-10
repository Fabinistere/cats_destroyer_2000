use bevy::prelude::*;

use self::sensors::{button_event, location_event, win_event, win_trigger, WinTriggerEvent};

pub mod level_one;
pub mod sensors;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Location {
    /// The transition universe
    /// Used when reset a Level
    Void,
    // Start cinematic
    // LevelZero,
    LevelOne,
    // Exit cinematic
    // Level1000,
}

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerLocation {
    LevelOne,
    LevelTwo,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_event::<WinTriggerEvent>()
            .add_plugin(level_one::LevelOnePlugin)
            .add_state(Location::LevelOne)
            .add_state(PlayerLocation::LevelOne)
            .add_system(win_event)
            .add_system(win_trigger)
            .add_system(location_event)
            .add_system(button_event)
            ;
    }
}
