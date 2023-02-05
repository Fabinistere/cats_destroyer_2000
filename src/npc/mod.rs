use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::character::{
        npc::{movement::BLACK_CAT_STARTING_POSITION, *},
        CHAR_HITBOX_HEIGHT, CHAR_HITBOX_WIDTH, CHAR_HITBOX_Y_OFFSET, CHAR_HITBOX_Z_OFFSET,
    },
    movement::{CharacterHitbox, MovementBundle, Speed},
    npc::movement::{npc_walk, NewDirectionEvent},
    spritesheet::{AnimState, AnimationTimer, CatSheet},
};

use self::movement::{give_new_direction_event, WalkBehavior};

pub mod movement;

#[derive(Default)]
pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_event::<NewDirectionEvent>()
            .add_startup_system(spawn_characters)
            .add_system(npc_walk)
            // event handler
            .add_system(give_new_direction_event)
            ;
    }
}

#[derive(Component, Inspectable)]
pub struct NPC;

fn spawn_characters(mut commands: Commands, cats: Res<CatSheet>) {
    // Black Cat
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: BLACK_CAT_STARTING_ANIM,
                    flip_x: true,
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
            Name::new("Blue Cat"),
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
                    angvel: 0.0,
                },
            },
            WalkBehavior {
                destination: Vec3::new(
                    BLACK_CAT_STARTING_POSITION.0,
                    BLACK_CAT_STARTING_POSITION.1 - 50.,
                    0.,
                ),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(CHAR_HITBOX_Z_OFFSET, CHAR_HITBOX_Y_OFFSET, 0.0),
                CharacterHitbox,
            ));
        });
}
