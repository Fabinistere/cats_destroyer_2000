use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::locations::{level_one::*, FLOOR_POSITION, LEVEL_POSITION, LEVEL_SCALE},
    locations::{
        level_one::doors::{animate_door, open_door_event, Door, ExitDoor, OpenDoorEvent},
        sensors::WinSensor,
        Location,
    },
    tablet::hack::Hackable,
};

use self::{
    button::{set_up_button, ButtonSensor},
    doors::DoorHitbox,
};

use super::sensors::LocationSensor;

pub mod button;
pub mod doors;

pub struct LevelOnePlugin;

impl Plugin for LevelOnePlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app .add_event::<OpenDoorEvent>()
            // .add_event::<ResetLevelOneEvent>()
            // .add_event::<EnterLevelOneEvent>()
            // .add_system(reset_level_one)
            // .add_system(enter_level_one)
            .add_system_set(
                SystemSet::on_enter(Location::LevelOne)
                    .with_system(setup_level_one)
                    .with_system(set_up_button)
            )
            .add_system_set(
                SystemSet::on_update(Location::LevelOne)
                    // .with_run_criteria(run_if_in_level_one)
                    .with_system(animate_door)
                    .with_system(open_door_event)
            )
            // .add_system_set(
            //     SystemSet::on_exit(Location::LevelOne)
            //         .with_system(despawn_level_one)
            // )
            ;
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy, Inspectable)]
pub enum LevelOneLocation {
    SpawnRoom,
    Corridor,
    Elevator,
}

#[derive(Component, Inspectable)]
pub struct CharacterLocation(pub LevelOneLocation);

// /// DOC
// pub struct ResetLevelOneEvent;

// /// DOC
// pub struct EnterLevelOneEvent;

// fn reset_level_one(
//     mut reset_level_event: EventReader<ResetLevelOneEvent>,
//     mut location: ResMut<State<Location>>,
//     mut enter_level_one_event: EventWriter<EnterLevelOneEvent>,
// ) {
//     for _ in reset_level_event.iter() {
//         if location.current() == &Location::LevelOne {
//             location.set(Location::Void).unwrap();
//             enter_level_one_event.send(EnterLevelOneEvent);
//         }
//     }
// }

// fn enter_level_one(
//     mut enter_level_event: EventReader<EnterLevelOneEvent>,
//     mut location: ResMut<State<Location>>,
// ) {
//     for _ in enter_level_event.iter() {
//         if location.current() != &Location::LevelOne {
//             location.set(Location::LevelOne).unwrap();
//         }
//     }
// }

// /// XXX: the filter on Name OUCH
// fn despawn_level_one(mut commands: Commands, query: Query<Entity, With<Name>>) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }

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
            parent.spawn((
                Collider::cuboid(1.5, 7.5),
                Transform::from_xyz(-29.5, 6., 0.),
                Name::new("Closet Left Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(12., 1.5),
                Transform::from_xyz(-16., 15., 0.),
                Name::new("Closet Top Hitbox"),
            ));
            parent.spawn((
                Collider::cuboid(12., 1.5),
                Transform::from_xyz(-16., -3., 0.),
                Name::new("Closet Bottom Hitbox"),
            ));
            // --- Closet Button Sensor ---
            parent.spawn((
                Collider::cuboid(2.5, 3.5),
                Transform::from_translation(BUTTON_SENSOR_POSITION.into()),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                ButtonSensor,
                // DOC: Better Naming
                Name::new("OneWay Button Sensor"),
            ));
        });

    // -- Doors --

    let horizontal_door = asset_server.load("textures/level_one/horizontal_door_anim.png");
    let horizontal_door_atlas =
        TextureAtlas::from_grid(horizontal_door, Vec2::new(12., 3.), 1, 7, None, None);

    let horizontal_door_atlas_handle = texture_atlases.add(horizontal_door_atlas);

    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(6., 1.5),
                Transform::from_xyz(5., -36., 0.),
                DoorHitbox,
                Name::new("Front Door Hitbox"),
            ));
            // --- Corridor Sensor ---
            parent.spawn((
                Collider::cuboid(6., 3.),
                Transform::from_xyz(5., -31.5, 0.),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                LocationSensor {
                    location: LevelOneLocation::Corridor,
                },
                Name::new("Corridor Sensor From Spawn"),
            ));
            // --- SpawnRoom Sensor ---
            parent.spawn((
                Collider::cuboid(6., 3.),
                Transform::from_xyz(5., -40.5, 0.),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                LocationSensor {
                    location: LevelOneLocation::SpawnRoom,
                },
                Name::new("SpawnRoom Sensor"),
            ));
        });

    // XXX: double load on the same sprite_sheet

    let out_horizontal_door = asset_server.load("textures/level_one/horizontal_door_anim.png");
    let out_horizontal_door_atlas =
        TextureAtlas::from_grid(out_horizontal_door, Vec2::new(12., 3.), 1, 7, None, None);

    let out_horizontal_door_atlas_handle = texture_atlases.add(out_horizontal_door_atlas);

    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(6., 1.5),
                Transform::from_xyz(5., 36., 0.),
                DoorHitbox,
                Name::new("Exit Door Hitbox"),
            ));
            // --- Win Sensor ---
            parent.spawn((
                Collider::segment(Vect::new(9., 48.5), Vect::new(1., 40.5)),
                Transform::default(),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                WinSensor,
                Name::new("Elevator Sensor"),
            ));
            // --- Corridor Sensor ---
            parent.spawn((
                Collider::cuboid(6., 3.),
                Transform::from_xyz(5., 31.5, 0.),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                LocationSensor {
                    location: LevelOneLocation::Corridor,
                },
                Name::new("Corridor Sensor From Exit"),
            ));
        });

    let vertical_door = asset_server.load("textures/level_one/vertical_door_anim.png");
    let vertical_door_atlas =
        TextureAtlas::from_grid(vertical_door, Vec2::new(3., 15.), 9, 1, None, None);

    let vertical_door_atlas_handle = texture_atlases.add(vertical_door_atlas);

    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(1.5, 7.5),
                Transform::from_xyz(-2.5, 6., 0.),
                DoorHitbox,
                Name::new("Side Door Hitbox"),
            ));
        });
}
