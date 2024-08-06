use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::movement::{CharacterHitbox, MovementBundle, Speed},
    characters::npcs::{
        aggression::{DetectionSensor, EngagePursuitEvent},
        movement::{NewDirectionEvent, ResetAggroEvent, WalkBehavior},
    },
    constants::character::{
        npc::{movement::BLACK_CAT_STARTING_POSITION, *},
        CHAR_HITBOX_HEIGHT, CHAR_HITBOX_WIDTH, CHAR_HITBOX_Y_OFFSET, CHAR_HITBOX_Z_OFFSET,
    },
    locations::level_one::{CharacterLocation, LevelOneLocation},
    spritesheet::{AnimState, AnimationTimer, CatSheet},
};

use self::movement::Target;

mod aggression;
pub mod movement;

#[derive(Default)]
pub struct NPCsPlugin;

impl Plugin for NPCsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewDirectionEvent>()
            .add_event::<EngagePursuitEvent>()
            .add_event::<ResetAggroEvent>()
            .add_systems(Startup, spawn_characters)
            .add_systems(
                Update,
                (
                    // -- Movement --
                    movement::npc_walk_to,
                    movement::npc_walk,
                    movement::npc_chase,
                    movement::daze_wait,
                    movement::give_new_direction_event,
                    // -- Aggression --
                    aggression::player_detection,
                    aggression::add_pursuit_urge,
                    aggression::reset_aggro,
                ),
            );
    }
}

#[derive(Component, Reflect)]
pub struct NPC;

fn spawn_characters(mut commands: Commands, cats: Res<CatSheet>) {
    // initial target
    let way_point = commands
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
            Name::new("WayPoint for Black Cat"),
        ))
        .id();

    // Black Cat
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: BLACK_CAT_STARTING_ANIM,
                    ..default()
                },
                texture_atlas: cats.0.clone(),
                transform: Transform {
                    translation: Vec3::from(BLACK_CAT_STARTING_POSITION),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("Black Cat"),
            NPC,
            // -- Animation --
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            AnimState {
                initial: BLACK_CAT_STARTING_ANIM,
                current: BLACK_CAT_STARTING_ANIM,
            },
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            // -- Movement --
            MovementBundle {
                speed: Speed::default(),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.,
                },
            },
            WalkBehavior,
            Target(Some(way_point)),
            CharacterLocation(LevelOneLocation::Corridor),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(CHAR_HITBOX_Z_OFFSET, CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
            ));
            parent.spawn((
                Collider::ball(40.),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                DetectionSensor,
                Name::new("Detection Range"),
            ));
        });
}
