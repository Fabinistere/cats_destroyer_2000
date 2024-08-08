#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity, clippy::too_many_arguments, clippy::pedantic)]
// #![warn(missing_docs)]

use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, window::WindowResolution};
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
        .insert_resource(ClearColor(Color::TEAL))
        .insert_resource(Msaa::Off)
        // v-- Hitbox --v
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
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
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    // This tells the AssetServer to watch for changes to assets.
                    // It enables our scenes to automatically reload in game when we modify their files.
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            // TweeningPlugin,
        ))
        .add_plugins((
            debug::DebugPlugin,
            spritesheet::CatSpritePlugin,
            locations::LocationsPlugin,
            tablet::TabletPlugin,
            characters::CharactersPlugin,
        ))
        .add_systems(Startup, spawn_camera);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}
