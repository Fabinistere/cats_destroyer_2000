#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;

use characters::CharactersPlugin;
use constants::{CLEAR, RESOLUTION, TILE_SIZE};
use debug::DebugPlugin;
use locations::LocationsPlugin;
use spritesheet::CatSpritePlugin;
use tablet::hack::HackPlugin;
use tablet::mind_control::MindControlPlugin;

pub mod characters;
pub mod collisions;
pub mod constants;
mod debug;
pub mod locations;
mod spritesheet;
pub mod tablet;

#[rustfmt::skip]
fn main() {
    let height = 720.;

    let mut app = App::new();
    app
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        // v-- Hitbox --v
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })

        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: height * RESOLUTION,
                        height,
                        title: "CatBeDoingTheLaundry".to_string(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        // .add_plugin(TweeningPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(CatSpritePlugin)
        .add_plugin(HackPlugin)
        .add_plugin(LocationsPlugin)
        .add_plugin(MindControlPlugin)
        .add_plugin(CharactersPlugin)
        .add_startup_system(spawn_camera);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 50. * TILE_SIZE;
    camera.projection.bottom = -50. * TILE_SIZE;

    camera.projection.left = 50. * TILE_SIZE * RESOLUTION;
    camera.projection.right = -50. * TILE_SIZE * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}
