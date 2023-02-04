//! All the stuffs that depend directly on the mechanics of mind control.

use bevy::prelude::*;

use crate::{mind_control::movement::mind_control_movement, npc::NPC, player::Player};

mod movement;

pub struct MindControlPlugin;

impl Plugin for MindControlPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_system(mind_control_button.label("enter_mind_control"))
            .add_system(exit_mind_control.label("exit_mind_control").after("enter_mind_control"))
            .add_system(mind_control_movement.label("movement"))
            .add_system(camera_follow.after("movement"))
            ;
    }
}

#[derive(Component)]
pub struct MindControled;

/// The camera follows the current Mind Controled entity
///
/// # Note
///
/// IDEA: gamefeel - smooth transition between mind control switch
fn camera_follow(
    mind_controled_query: Query<&Transform, With<MindControled>>,
    mut camera_query: Query<&mut Transform, (Without<MindControled>, With<Camera>)>,
) {
    let player_transform = mind_controled_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub fn mind_control_button(
    mut commands: Commands,

    keyboard_input: Res<Input<KeyCode>>,

    player_query: Query<Entity, With<Player>>,
    npc_query: Query<Entity, With<NPC>>,
) {
    if keyboard_input.pressed(KeyCode::M) {
        for npc in npc_query.iter() {
            commands.entity(npc).insert(MindControled);
            break;
        }
        let player = player_query.single();
        commands.entity(player).remove::<MindControled>();
    }
}

fn exit_mind_control(
    mut commands: Commands,

    keyboard_input: Res<Input<KeyCode>>,

    player_query: Query<Entity, With<Player>>,
    npc_query: Query<Entity, With<NPC>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        for npc in npc_query.iter() {
            commands.entity(npc).remove::<MindControled>();
            // break;
        }
        let player = player_query.single();
        commands.entity(player).insert(MindControled);
    }
}
