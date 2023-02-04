use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{movement::Speed, npc::NPC};

#[derive(Component)]
pub struct WalkBehavior {
    pub destination: Vec3,
}

/// DOC
pub struct FreezeEvent;

/// DOC
pub struct NewDirectionEvent(pub Entity);

/// # Note
///
/// TODO: feature - Without MindControled
pub fn npc_walk(
    // mut commands: Commands,

    mut npc_query: Query<
        (
            Entity,
            &Transform,
            &Speed,
            &mut Velocity,
            &WalkBehavior,
            &Name,
        ),
        With<NPC>,
    >,

    mut new_drection_event: EventWriter<NewDirectionEvent>,
) {
    for (npc, npc_transform, speed, mut rb_vel, walk_behavior, _npc_name) in npc_query.iter_mut() {
        let direction: Vec3 = walk_behavior.destination;

        let close_range_width = npc_transform.scale.x * 10.;
        let close_range_height = npc_transform.scale.y * 10.;

        if direction.x - close_range_width < npc_transform.translation.x
            && direction.x + close_range_width > npc_transform.translation.x
            && direction.y - close_range_height < npc_transform.translation.y
            && direction.y + close_range_height > npc_transform.translation.y
        {
            // info!("{} reached destination", npc_name);
            new_drection_event.send(NewDirectionEvent(npc));
            // commands.entity(npc).insert(Clicked);
        } else {
            let up = direction.y > npc_transform.translation.y;
            let down = direction.y < npc_transform.translation.y;
            let left = direction.x < npc_transform.translation.x;
            let right = direction.x > npc_transform.translation.x;

            let x_axis = -(left as i8) + right as i8;
            let y_axis = -(down as i8) + up as i8;

            // println!("x: {}, y: {}", x_axis, y_axis);

            let mut vel_x = x_axis as f32 * **speed;
            let mut vel_y = y_axis as f32 * **speed;

            if x_axis != 0 && y_axis != 0 {
                vel_x *= (std::f32::consts::PI / 4.0).cos();
                vel_y *= (std::f32::consts::PI / 4.0).cos();
            }

            rb_vel.linvel.x = vel_x;
            rb_vel.linvel.y = vel_y;
        }
    }
}

/// Event Handler of NewDirectionEvent
pub fn give_new_direction_event(
    mut new_direction_event: EventReader<NewDirectionEvent>,
    mut npc_query: Query<(Entity, &Transform, &mut WalkBehavior, &Name), With<NPC>>,
) {
    for event in new_direction_event.iter() {
        match npc_query.get_mut(event.0) {
            Err(e) => warn!("{:?}", e),
            Ok((_npc, npc_transform, mut walk_behavior, _name)) => {
                // simple turn back: up and down
                walk_behavior.destination = Vec3::new(
                    npc_transform.translation.x,
                    -walk_behavior.destination.y,
                    0.,
                )
            }
        }
        // WalkBehavior{ destination: Vec3::new(BLACK_CAT_POSITION.0, BLACK_CAT_POSITION.1 - 50., 0.) }
    }
}
