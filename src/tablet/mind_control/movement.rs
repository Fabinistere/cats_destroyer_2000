use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{characters::movement::Speed, tablet::mind_control::MindControled};

/// The player input will act on the current MindControled entity
pub fn mind_control_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut mind_controled_query: Query<(&Speed, &mut Velocity), With<MindControled>>,
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
