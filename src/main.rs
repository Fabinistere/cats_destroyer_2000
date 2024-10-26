#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::pedantic)]
#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value, // required by Bevy systems' structure
    clippy::module_name_repetitions // could be removed but currently follwoing Bevy community examples
)]

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

use constants::{RESOLUTION, TILE_SIZE};

pub mod characters;
pub mod collisions;
pub mod constants;
mod debug;
pub mod locations;
mod spritesheet;
pub mod tablet;

fn main() {
    let height = 1080.;

    let mut app = App::new();
    app
        // Color::TEAL / AZURE
        .insert_resource(ClearColor(Color::srgb(0., 0.5, 0.5)))
        .insert_resource(Msaa::Off)
        // v-- Hitbox --v
        .insert_resource(
            // gravity: Vec2::ZERO,
            RapierConfiguration::new(0.),
        )
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(height * RESOLUTION, height),
                        title: "CatBeDoingTheLaundry".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            bevy_tweening::TweeningPlugin,
        ))
        .add_plugins((
            debug::DebugPlugin,
            spritesheet::CatSpritePlugin,
            locations::LocationsPlugin,
            tablet::TabletPlugin,
            characters::CharactersPlugin,
        ))
        .init_state::<GameState>()
        .add_sub_state::<HudState>()
        .add_systems(Startup, spawn_camera);

    app.run();
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum GameState {
    MainMenu,
    #[default]
    InGame,
}

#[derive(SubStates, PartialEq, Eq, Clone, Hash, Debug, Default)]
#[source(GameState = GameState::InGame)]
pub enum HudState {
    #[default]
    Closed,
    Tablet,
}

#[derive(Component)]
pub struct PlayerCamera;

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn((camera, PlayerCamera, Name::new("Player Camera")));
}
