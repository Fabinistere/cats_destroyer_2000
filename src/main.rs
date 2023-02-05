use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;

use constants::{CLEAR, RESOLUTION, TILE_SIZE};
use debug::DebugPlugin;
use hack::HackPlugin;
use locations::LocationsPlugin;
use mind_control::MindControlPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;
use spritesheet::CatSpritePlugin;

pub mod constants;
mod debug;
mod hack;
pub mod locations;
mod mind_control;
mod movement;
mod npc;
mod player;
mod spritesheet;

#[rustfmt::skip]
fn main() {
    let height = 720.0;

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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        // .add_plugin(TweeningPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(CatSpritePlugin)
        .add_plugin(HackPlugin)
        .add_plugin(LocationsPlugin)
        .add_plugin(MindControlPlugin)
        .add_plugin(NPCPlugin)
        .add_plugin(PlayerPlugin)
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
