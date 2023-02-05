use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    constants::locations::{level_one::*, FLOOR_POSITION, LEVEL_POSITION, LEVEL_SCALE},
    locations::{level_one::doors::animate_doors, Location},
    spritesheet::AnimationTimer,
};

mod doors;

pub struct LevelOnePlugin;

impl Plugin for LevelOnePlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app.add_state(PlayerLocation::LevelOne)
            .add_system_set(
                SystemSet::on_enter(Location::LevelOne)
                    .with_system(setup_level_one)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_level_one)
                    .with_system(animate_doors)
            )
            ;
    }
}

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerLocation {
    LevelOne,
}

fn run_if_in_level_one(location: Res<State<Location>>) -> ShouldRun {
    if location.current() == &Location::LevelOne {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn setup_level_one(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let walls = asset_server.load("textures/level_one/lab_wall.png");
    let floor = asset_server.load("textures/level_one/lab_floor.png");

    commands.spawn((
        SpriteBundle {
            texture: floor.clone(),
            transform: Transform {
                translation: FLOOR_POSITION.into(),
                scale: LEVEL_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        },
        RigidBody::Fixed,
        Name::new("floor"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: walls.clone(),
            transform: Transform {
                translation: LEVEL_POSITION.into(),
                scale: LEVEL_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        },
        RigidBody::Fixed,
        Name::new("walls"),
    ));

    // -- Doors --

    let horizontal_door = asset_server.load("textures/level_one/horizontal_door_anim.png");
    let horizontal_door_atlas =
        TextureAtlas::from_grid(horizontal_door, Vec2::new(12., 3.), 1, 7, None, None);

    let horizontal_door_atlas_handle = texture_atlases.add(horizontal_door_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: horizontal_door_atlas_handle,
            transform: Transform {
                translation: IN_DOOR_POSITION.into(),
                scale: LEVEL_SCALE.into(),
                ..default()
            },
            ..default()
        },
        Name::new("IN_DOOR"),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    // XXX: double load on the same sprite_sheet

    let out_horizontal_door = asset_server.load("textures/level_one/horizontal_door_anim.png");
    let out_horizontal_door_atlas =
        TextureAtlas::from_grid(out_horizontal_door, Vec2::new(12., 3.), 1, 7, None, None);

    let out_horizontal_door_atlas_handle = texture_atlases.add(out_horizontal_door_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: out_horizontal_door_atlas_handle,
            transform: Transform {
                translation: OUT_DOOR_POSITION.into(),
                scale: LEVEL_SCALE.into(),
                ..default()
            },
            ..default()
        },
        Name::new("OUT_DOOR"),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    let vertical_door = asset_server.load("textures/level_one/vertical_door_anim.png");
    let vertical_door_atlas =
        TextureAtlas::from_grid(vertical_door, Vec2::new(3., 15.), 9, 1, None, None);

    let vertical_door_atlas_handle = texture_atlases.add(vertical_door_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: vertical_door_atlas_handle,
            transform: Transform {
                translation: ALT_DOOR_POSITION.into(),
                scale: LEVEL_SCALE.into(),
                ..default()
            },
            ..default()
        },
        Name::new("ALT_DOOR"),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
