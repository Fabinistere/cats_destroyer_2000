use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::movement::{Dazed, Speed},
    tablet::mind_control::MindControlled,
};

/// The player input will act on the current MindControlled entity
pub fn mind_control_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut mind_controled_query: Query<(&Speed, &mut Velocity), With<MindControlled>>,
) {
    if let Ok((speed, mut rb_vel)) = mind_controled_query.get_single_mut() {
        let up = keyboard_input.pressed(KeyCode::Z)
            || keyboard_input.pressed(KeyCode::Up)
            || keyboard_input.pressed(KeyCode::W);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::Q)
            || keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

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
