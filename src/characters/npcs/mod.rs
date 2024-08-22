use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        movement::{CharacterHitbox, MovementBundle, Speed},
        npcs::{
            aggression::{DetectionSensor, EngagePursuitEvent},
            movement::{NewWayPointEvent, ResetAggroEvent, Target, WalkBehavior},
        },
        Character,
    },
    constants::character::{
        npcs::{movement::BLACK_CAT_STARTING_POSITION, BLACK_CAT_STARTING_ANIM, NPC_SCALE},
        CHAR_HITBOX_HEIGHT, CHAR_HITBOX_WIDTH, CHAR_HITBOX_Y_OFFSET, CHAR_HITBOX_Z_OFFSET,
    },
    locations::{
        level_one::{CharacterLocation, Level1000Location},
        Location,
    },
    spritesheet::{AnimState, AnimationTimer, CatSheet},
};

mod aggression;
pub mod movement;

#[derive(Default)]
pub struct NPCsPlugin;

impl Plugin for NPCsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewWayPointEvent>()
            .add_event::<EngagePursuitEvent>()
            .add_event::<ResetAggroEvent>()
            .add_systems(OnEnter(Location::Level1000), spawn_characters)
            .add_systems(
                Update,
                (
                    aggression::player_detection,
                    aggression::add_pursuit_urge,
                    movement::npc_walk,
                    movement::npc_chase,
                    movement::npc_walk_to,
                    movement::daze_wait,
                    aggression::reset_aggro,
                    movement::give_new_way_point_event,
                )
                    .chain()
                    .run_if(in_state(Location::Level1000)),
            );
    }
}

#[derive(Component, Reflect)]
pub struct NPC;

fn spawn_characters(mut commands: Commands, cats: Res<CatSheet>) {
    // Black Cat
    commands
        .spawn((
            SpriteBundle {
                texture: cats.texture.clone(),
                transform: Transform {
                    translation: Vec3::from(BLACK_CAT_STARTING_POSITION),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: cats.atlas_handle.clone(),
                index: BLACK_CAT_STARTING_ANIM,
            },
            Name::new("Black Cat"),
            NPC,
            Character,
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
            Target(None),
            CharacterLocation(Level1000Location::Corridor),
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
