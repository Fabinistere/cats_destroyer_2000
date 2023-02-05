use bevy::prelude::*;

pub mod level_one;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Location {
    LevelOne,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_plugin(level_one::LevelOnePlugin)
            .add_state(Location::LevelOne);
    }
}
