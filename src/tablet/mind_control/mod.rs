//! All the stuffs that depend directly on the mechanics of mind control.

use std::time::Duration;

use bevy::prelude::*;

use crate::{
    characters::movement::Dazed,
    characters::{effects::style::DazeAnimation, npcs::NPC, player::Player},
    constants::character::effects::DAZE_TIMER,
    tablet::{
        mind_control::movement::mind_control_movement, run_if_tablet_is_free,
        run_if_tablet_is_mind_ctrl,
    },
};

mod movement;

pub struct MindControlPlugin;

impl Plugin for MindControlPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                .with_run_criteria(run_if_tablet_is_free)
                .with_system(mind_control_button.label("enter_mind_control"))
            )
            .add_system_set(
                SystemSet::new()
                .with_run_criteria(run_if_tablet_is_mind_ctrl)
                .with_system(exit_mind_control.label("exit_mind_control").after("enter_mind_control"))
            )
            .add_system(mind_control_movement.label("movement").after("enter_mind_control"))
            .add_system(camera_follow.after("movement"))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                daze_post_mind_control//.after("exit_mind_control")
            )
            .add_system(daze_cure_by_mind_control.before("exit_mind_control").after("enter_mind_control"))
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
            commands.entity(npc).insert(MindControled); // .remove::<Dazed>()
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
    npc_query: Query<(Entity, &Name), (With<NPC>, With<MindControled>)>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        // could be a single for now
        for (npc, _name) in npc_query.iter() {
            commands.entity(npc).remove::<MindControled>();
        }

        let player = player_query.single();
        commands.entity(player).insert(MindControled);
    }
}

fn daze_post_mind_control(
    mut commands: Commands,
    mind_controled_removals: RemovedComponents<MindControled>,

    player_query: Query<Entity, With<Player>>,
) {
    for entity in mind_controled_removals.iter() {
        match player_query.get(entity) {
            // This is prbly a npc
            Err(_) => {
                commands.entity(entity).insert(Dazed {
                    timer: Timer::new(Duration::from_secs(DAZE_TIMER), TimerMode::Once),
                });
            }
            Ok(_) => {
                // Will never be decreased (no system for it)
                // Only removed by adding MindControled back to the player
                // So the content of the timer is useless
                commands.entity(entity).insert(Dazed {
                    timer: Timer::new(Duration::from_secs(DAZE_TIMER), TimerMode::Repeating),
                });
            }
        }
    }
}

fn daze_cure_by_mind_control(
    mut commands: Commands,

    mind_controled_query: Query<(Entity, &Name, &Children), Added<MindControled>>,
    daze_effect_query: Query<Entity, With<DazeAnimation>>,
) {
    for (entity, _name, children) in mind_controled_query.iter() {
        commands.entity(entity).remove::<Dazed>();
        for child in children {
            match daze_effect_query.get(*child) {
                Err(_) => continue,
                Ok(daze_effect) => {
                    // Only works for player
                    // XXX: don't remove the link to their parent
                    commands.entity(daze_effect).despawn();
                }
            }
        }
    }
}
