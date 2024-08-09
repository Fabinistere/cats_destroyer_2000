use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::movement::{Dazed, Speed},
    tablet::mind_control::MindControlled,
};

/// The player input will act on the current MindControlled entity
pub fn mind_control_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mind_controled_query: Query<(&Speed, &mut Velocity), With<MindControlled>>,
) {
    if let Ok((speed, mut rb_vel)) = mind_controled_query.get_single_mut() {
        let up = keyboard_input.pressed(KeyCode::KeyZ)
            || keyboard_input.pressed(KeyCode::ArrowUp)
            || keyboard_input.pressed(KeyCode::KeyW);
        let down =
            keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
        let left = keyboard_input.pressed(KeyCode::KeyQ)
            || keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::KeyA);
        let right =
            keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

        let x_axis = -(left as i8) + right as i8;
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

pub fn freeze_dazed_character(mut dazed_cats_query: Query<&mut Velocity, With<Dazed>>) {
    for mut dazed_cat_velocity in dazed_cats_query.iter_mut() {
        dazed_cat_velocity.linvel.x = 0.;
        dazed_cat_velocity.linvel.y = 0.;
    }
}
