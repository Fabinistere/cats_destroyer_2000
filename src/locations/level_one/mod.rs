use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::Character,
    constants::{
        character::npcs::movement::BLACK_CAT_STARTING_POSITION,
        locations::{
            level_one::{
                ALT_DOOR_POSITION, BUTTON_SENSOR_POSITION, IN_DOOR_POSITION, OUT_DOOR_POSITION,
                WAYPOINT_BOT, WAYPOINT_TOP,
            },
            FLOOR_POSITION, LEVEL_POSITION, LEVEL_SCALE,
        },
    },
    locations::{
        level_one::{
            button::ButtonSensor,
            doors::{Door, DoorHitbox, ExitDoor, OpenDoorEvent},
        },
        sensors::{LocationSensor, WinSensor},
        Location,
    },
    tablet::hack::Hackable,
};

pub mod button;
pub mod doors;

pub struct LevelOnePlugin;

impl Plugin for LevelOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenDoorEvent>()
            .add_systems(
                OnEnter(Location::Level1000),
                (setup_level_one, button::set_up),
            )
            .add_systems(
                Update,
                (doors::animate_door, doors::open_door_event).run_if(in_state(Location::Level1000)),
            )
            .add_systems(OnExit(Location::Level1000), despawn_level_one);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy, Reflect)]
pub enum Level1000Location {
    SpawnRoom,
    Corridor,
    Elevator,
}

#[derive(Component, Reflect, PartialEq, Eq)]
pub struct CharacterLocation(pub Level1000Location);

#[derive(Component, PartialEq, Eq)]
pub enum WayPoint {
    Top,
    Bot,
}

/* --------------------------------- Systems -------------------------------- */

fn despawn_level_one(
    mut commands: Commands,
    level_1000_query: Query<(Entity, &Name), Or<(With<Character>, With<Location>)>>,
) {
    for (entity, name) in level_1000_query.iter() {
        info!("{name} despawn");
        commands.entity(entity).despawn_recursive();
        // NOTE: the warning that we'll get is due to the already despawned `DazeAnimation`; But the link with the parent remain.
    }
}

#[allow(clippy::too_many_lines)]
fn setup_level_one(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // -- WayPoints --
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    BLACK_CAT_STARTING_POSITION.0,
                    BLACK_CAT_STARTING_POSITION.1 - 50.,
                    0.,
                )),
                visibility: Visibility::Hidden,
                ..default()
            },
            Name::new("WayPoints"),
            Location::Level1000,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(WAYPOINT_TOP.into()),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::new("WayPoint Top"),
                WayPoint::Top,
            ));

            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(WAYPOINT_BOT.into()),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::new("WayPoint Bot"),
                WayPoint::Bot,
            ));
        });

    // -- Map --
    let walls = asset_server.load("textures/level_one/lab_wall.png");
    let floor = asset_server.load("textures/level_one/lab_floor.png");

    commands
        .spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            Location::Level1000,
            Name::new("LevelOne"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: floor.clone(),
                    transform: Transform {
                        translation: FLOOR_POSITION.into(),
                        scale: LEVEL_SCALE.into(),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Fixed,
                Name::new("floor"),
            ));

            parent
                .spawn((
                    SpriteBundle {
                        texture: walls.clone(),
                        transform: Transform {
                            translation: LEVEL_POSITION.into(),
                            scale: LEVEL_SCALE.into(),
                            ..default()
                        },
                        ..default()
                    },
                    RigidBody::Fixed,
                    Name::new("walls"),
                ))
                .with_children(|parent| {
                    // Collider draw by hand...
                    parent.spawn((
                        Collider::cuboid(21., 1.5),
                        Transform::from_xyz(-5., -57., 0.),
                        Name::new("Entry Lower Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(1.5, 12.),
                        Transform::from_xyz(-24.5, -46.5, 0.),
                        Name::new("Entry Left Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(1.5, 12.),
                        Transform::from_xyz(14.5, -46.5, 0.),
                        Name::new("Entry Right Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(7.5, 1.5),
                        Transform::from_xyz(-18.5, -36., 0.),
                        Name::new("Entry Top Left Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(7.5, 1.5),
                        Transform::from_xyz(8.5, -36., 0.),
                        Name::new("Entry Top Right Hitbox"),
                    ));
                    // --- Corridor ---
                    parent.spawn((
                        Collider::cuboid(1.5, 44.5),
                        Transform::from_xyz(-12.5, 10., 0.),
                        Name::new("Corridor Left Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(1.5, 16.5),
                        Transform::from_xyz(2.5, -18., 0.),
                        Name::new("Corridor Right Bottom Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(1.5, 20.5),
                        Transform::from_xyz(2.5, 34., 0.),
                        Name::new("Corridor Right Top Hitbox"),
                    ));
                    // --- Elevator ---
                    // Which is the two Exit and Front Door
                    parent.spawn((
                        Collider::cuboid(6., 1.5),
                        Transform::from_xyz(-5., 53., 0.),
                        Name::new("Elevator Top Hitbox"),
                    ));
                    // --- Broom Closet ---
                    parent.spawn((
                        Collider::cuboid(1.5, 7.5),
                        Transform::from_xyz(29.5, 6., 0.),
                        Name::new("Closet Left Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(12., 1.5),
                        Transform::from_xyz(16., 15., 0.),
                        Name::new("Closet Top Hitbox"),
                    ));
                    parent.spawn((
                        Collider::cuboid(12., 1.5),
                        Transform::from_xyz(16., -3., 0.),
                        Name::new("Closet Bottom Hitbox"),
                    ));
                    // --- Closet Button Sensor ---
                    parent.spawn((
                        Collider::cuboid(2.5, 3.5),
                        Transform::from_translation(BUTTON_SENSOR_POSITION.into()),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        ButtonSensor,
                        Name::new("Escape Button Sensor"),
                    ));
                });

            // -- Doors --
            let horizontal_door = asset_server.load("textures/level_one/horizontal_door_anim.png");
            let horizontal_door_atlas =
                TextureAtlasLayout::from_grid(UVec2::new(12, 3), 1, 7, None, None);

            let horizontal_door_atlas_handle = texture_atlases.add(horizontal_door_atlas);
            parent
                .spawn((
                    SpriteBundle {
                        texture: horizontal_door.clone(),

                        transform: Transform {
                            translation: IN_DOOR_POSITION.into(),
                            scale: LEVEL_SCALE.into(),
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: horizontal_door_atlas_handle.clone(),
                        index: 0,
                    },
                    Name::new("Front Door"),
                    RigidBody::Fixed,
                    Door {
                        current_state: doors::DoorState::Closed,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::cuboid(6., 1.5),
                        Transform::default(), // from_xyz(-5., -36., 0.),
                        DoorHitbox,
                        Name::new("Front Door Hitbox"),
                    ));
                    // --- Corridor Sensor ---
                    parent.spawn((
                        Collider::cuboid(6., 3.),
                        Transform::from_xyz(0., 4.5, 0.),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        LocationSensor {
                            location: Level1000Location::Corridor,
                        },
                        Name::new("Corridor Sensor From Spawn"),
                    ));
                    // --- SpawnRoom Sensor ---
                    parent.spawn((
                        Collider::cuboid(6., 3.),
                        Transform::from_xyz(0., -4.5, 0.),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        LocationSensor {
                            location: Level1000Location::SpawnRoom,
                        },
                        Name::new("SpawnRoom Sensor"),
                    ));
                });

            parent
                .spawn((
                    SpriteBundle {
                        texture: horizontal_door,

                        transform: Transform {
                            translation: OUT_DOOR_POSITION.into(),
                            scale: LEVEL_SCALE.into(),
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: horizontal_door_atlas_handle.clone(),
                        index: 0,
                    },
                    Name::new("Exit Door"),
                    RigidBody::Fixed,
                    ExitDoor,
                    Door {
                        current_state: doors::DoorState::Closed,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::cuboid(6., 1.5),
                        Transform::default(), // from_xyz(-5., 36., 0.),
                        DoorHitbox,
                        Name::new("Exit Door Hitbox"),
                    ));
                    // --- Win Sensor ---
                    parent.spawn((
                        Collider::cuboid(4., 4.), // segment(Vect::new(9., 48.5), Vect::new(1., 40.5)),
                        Transform::from_xyz(0., 8.5, 0.), // default(),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        WinSensor,
                        Name::new("Elevator Sensor"),
                    ));
                    // --- Corridor Sensor ---
                    parent.spawn((
                        Collider::cuboid(6., 3.),
                        Transform::from_xyz(0., -4.5, 0.),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        LocationSensor {
                            location: Level1000Location::Corridor,
                        },
                        Name::new("Corridor Sensor From Exit"),
                    ));
                });

            let vertical_door = asset_server.load("textures/level_one/vertical_door_anim.png");
            let vertical_door_atlas =
                TextureAtlasLayout::from_grid(UVec2::new(3, 15), 9, 1, None, None);

            let vertical_door_atlas_handle = texture_atlases.add(vertical_door_atlas);

            parent
                .spawn((
                    SpriteBundle {
                        texture: vertical_door,
                        transform: Transform {
                            translation: ALT_DOOR_POSITION.into(),
                            scale: LEVEL_SCALE.into(),
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: vertical_door_atlas_handle,
                        index: 0,
                    },
                    Name::new("Closet Door"),
                    RigidBody::Fixed,
                    Door {
                        current_state: doors::DoorState::Closed,
                    },
                    Hackable,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::cuboid(1.5, 7.5),
                        Transform::default(), // from_xyz(2.5, 6., 0.),
                        DoorHitbox,
                        Name::new("Side Door Hitbox"),
                    ));
                });
        });
}
