use crate::{
    characters::movement::{CharacterHitbox, MovementBundle, Speed},
    constants::character::{
        npcs::{movement::BLUE_CAT_STARTING_POSITION, *},
        CHAR_HITBOX_HEIGHT, CHAR_HITBOX_WIDTH, CHAR_HITBOX_Y_OFFSET, CHAR_HITBOX_Z_OFFSET,
    },
    locations::{
        level_one::{CharacterLocation, Level1000Location},
        Location,
    },
    spritesheet::{AnimState, AnimationTimer, CatSheet},
    tablet::mind_control::MindControlled,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Character;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Location::Level1000), spawn_player)
            .add_systems(Update, player_idle.run_if(in_state(Location::Level1000)));
    }
}

#[derive(Component, Reflect)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHitbox;

/// # Note
///
/// Player's velocity = 0 if not self MindControlled to avoid being lauched
fn player_idle(mut player_query: Query<&mut Velocity, (With<Player>, Without<MindControlled>)>) {
    if let Ok(mut rb_vel) = player_query.get_single_mut() {
        rb_vel.linvel.x = 0.;
        rb_vel.linvel.y = 0.;
    }
}

fn spawn_player(mut commands: Commands, cats: Res<CatSheet>) {
    // Blue Cat
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: BLUE_CAT_STARTING_ANIM,
                    ..default()
                },
                texture_atlas: cats.0.clone(),
                transform: Transform {
                    translation: Vec3::from(BLUE_CAT_STARTING_POSITION),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("Player: Blue Cat"),
            Player,
            MindControlled,
            // -- Animation --
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            AnimState {
                initial: BLUE_CAT_STARTING_ANIM,
                current: BLUE_CAT_STARTING_ANIM,
            },
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            MovementBundle {
                speed: Speed::default(),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.,
                },
            },
            Character,
            CharacterLocation(Level1000Location::SpawnRoom),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(CHAR_HITBOX_Z_OFFSET, CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
                PlayerHitbox,
            ));
        });
}
