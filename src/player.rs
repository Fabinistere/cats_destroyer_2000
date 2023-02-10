use crate::{
    constants::character::{
        npc::{movement::BLUE_CAT_STARTING_POSITION, *},
        CHAR_HITBOX_HEIGHT, CHAR_HITBOX_WIDTH, CHAR_HITBOX_Y_OFFSET, CHAR_HITBOX_Z_OFFSET,
    },
    locations::level_one::{CharacterLocation, LevelOneLocation},
    movement::{CharacterHitbox, MovementBundle, Speed},
    spritesheet::{AnimState, AnimationTimer, CatSheet},
    tablet::mind_control::MindControled,
};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement.label("movement"))
            .add_system(player_idle.after("movement"))
            ;
    }
}

#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHitbox;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Speed, &mut Velocity), (With<Player>, With<MindControled>)>,
) {
    if let Ok((speed, mut rb_vel)) = player_query.get_single_mut() {
        let up = keyboard_input.pressed(KeyCode::Z);
        let down = keyboard_input.pressed(KeyCode::S);
        let left = keyboard_input.pressed(KeyCode::Q);
        let right = keyboard_input.pressed(KeyCode::D);

        let x_axis = -(right as i8) + left as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut vel_x = x_axis as f32 * **speed;
        let mut vel_y = y_axis as f32 * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.).cos();
            vel_y *= (std::f32::consts::PI / 4.).cos();
        }

        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;
    }
}

/// # Note
///
/// Player's velocity = 0 if not self MindControled to avoid being lauched
fn player_idle(mut player_query: Query<&mut Velocity, (With<Player>, Without<MindControled>)>) {
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
                    flip_x: true,
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
            MindControled,
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
            CharacterLocation(LevelOneLocation::SpawnRoom),
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
