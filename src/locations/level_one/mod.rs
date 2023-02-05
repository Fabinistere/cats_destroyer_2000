use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    constants::locations::{level_one::*, FLOOR_POSITION, LEVEL_POSITION, LEVEL_SCALE},
    locations::{
        level_one::doors::{animate_door, open_door_event, Door, ExitDoor, OpenDoorEvent},
        Location,
    },
    tablet::hack::Hackable,
};

pub mod doors;

pub struct LevelOnePlugin;

impl Plugin for LevelOnePlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_state(PlayerLocation::LevelOne)
            .add_event::<OpenDoorEvent>()
            .add_system_set(
                SystemSet::on_enter(Location::LevelOne)
                    .with_system(setup_level_one)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_level_one)
                    .with_system(animate_door)
                    .with_system(open_door_event)
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
    // -- Map --

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

    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            // Collider draw by hand...
            parent.spawn((
                Collider::cuboid(21., 1.5),
                Transform::from_xyz(5., -57., 0.),
                Name::new("Entry Lower Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(1.5, 12.),
                Transform::from_xyz(24.5, -46.5, 0.),
                Name::new("Entry Left Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(1.5, 12.),
                Transform::from_xyz(-14.5, -46.5, 0.),
                Name::new("Entry Right Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(7.5, 1.5),
                Transform::from_xyz(18.5, -36., 0.),
                Name::new("Entry Top Left Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(7.5, 1.5),
                Transform::from_xyz(-8.5, -36., 0.),
                Name::new("Entry Top Right Hitbox"),
            ));
            // --- Corridor ---
            parent.spawn((
                Collider::cuboid(1.5, 44.5),
                Transform::from_xyz(12.5, 10., 0.),
                Name::new("Corridor Left Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(1.5, 16.5),
                Transform::from_xyz(-2.5, -18., 0.),
                Name::new("Corridor Right Bottom Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(1.5, 20.5),
                Transform::from_xyz(-2.5, 34., 0.),
                Name::new("Corridor Right Top Hitbox"),
            ));
            // --- Elevator ---
            // Which is the two Exit and Front Door
            parent.spawn((
                Collider::cuboid(6., 1.5),
                Transform::from_xyz(5., 53., 0.),
                Name::new("Elevator Top Hitbox"),
            ));
            // --- Broom Closet ---
        });

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
        Name::new("Front Door"),
        Door {
            current_state: doors::DoorState::Closed,
        },
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
        Name::new("Exit Door"),
        ExitDoor,
        Door {
            current_state: doors::DoorState::Closed,
        },
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
        Name::new("Closet Door"),
        Door {
            current_state: doors::DoorState::Closed,
        },
        Hackable,
    ));
}
