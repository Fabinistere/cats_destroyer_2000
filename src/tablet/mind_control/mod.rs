//! All the stuffs that depend directly on the mechanics of mind control.

use std::time::Duration;

use bevy::prelude::*;

use crate::{
    characters::movement::Dazed,
    characters::{effects::style::DazeAnimation, npcs::NPC, player::Player},
    constants::character::effects::DAZE_TIMER,
    locations::Location,
    tablet::{tablet_is_free, tablet_is_mind_ctrl},
};

mod movement;

pub struct MindControlPlugin;

impl Plugin for MindControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentlyMindControlled>().add_systems(
            Update,
            (
                mind_control_button.run_if(tablet_is_free),
                exit_mind_control.run_if(tablet_is_mind_ctrl),
                daze_cure_by_mind_control,
                daze_post_mind_control,
                movement::mind_control_movement,
                camera_follow,
                movement::freeze_dazed_character,
            )
                .chain()
                .run_if(in_state(Location::Level1000)),
        );
    }
}

#[derive(Component)]
pub struct MindControlled;

/// Used to choose which sprite to animate at the end cinematic
/// When we despawned every characters.
#[derive(Resource, PartialEq, Eq, Default)]
pub enum CurrentlyMindControlled {
    BlackCat,
    #[default]
    BlueCat,
}

/* --------------------------------- Systems -------------------------------- */

/// The camera follows the current Mind Controled entity
///
/// # Note
///
/// IDEA: gamefeel - smooth transition between mind control switch
fn camera_follow(
    mind_controled_query: Query<&Transform, With<MindControlled>>,
    mut camera_query: Query<&mut Transform, (Without<MindControlled>, With<Camera>)>,
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
    mut currently_mind_controlled: ResMut<CurrentlyMindControlled>,
) {
    if keyboard_input.pressed(KeyCode::M) || keyboard_input.pressed(KeyCode::Colon) {
        if let Some(npc) = npc_query.iter().next() {
            commands.entity(npc).insert(MindControlled); // .remove::<Dazed>()
            let player = player_query.single();
            commands.entity(player).remove::<MindControlled>();

            *currently_mind_controlled = CurrentlyMindControlled::BlackCat;
        }
    }
}

fn exit_mind_control(
    mut commands: Commands,

    keyboard_input: Res<Input<KeyCode>>,

    player_query: Query<Entity, With<Player>>,
    npc_query: Query<(Entity, &Name), (With<NPC>, With<MindControlled>)>,
    mut currently_mind_controlled: ResMut<CurrentlyMindControlled>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        for (npc, _name) in npc_query.iter() {
            commands.entity(npc).remove::<MindControlled>();
        }

        let player = player_query.single();
        commands.entity(player).insert(MindControlled);
        *currently_mind_controlled = CurrentlyMindControlled::BlueCat;
    }
}

fn daze_post_mind_control(
    mut commands: Commands,
    mut mind_controled_removals: RemovedComponents<MindControlled>,

    player_query: Query<Entity, With<Player>>,
    npcs_query: Query<Entity, With<NPC>>,
) {
    for entity in mind_controled_removals.read() {
        if player_query.get(entity).is_ok() {
            // Will never be decreased (no system for it)
            // Only removed by adding MindControlled back to the player
            // So the content of the timer is useless
            commands.entity(entity).insert(Dazed {
                timer: Timer::new(Duration::from_secs(DAZE_TIMER), TimerMode::Repeating),
            });
        } else if npcs_query.get(entity).is_ok() {
            commands.entity(entity).insert(Dazed {
                timer: Timer::new(Duration::from_secs(DAZE_TIMER), TimerMode::Once),
            });
        }
    }
}

fn daze_cure_by_mind_control(
    mut commands: Commands,

    mind_controled_query: Query<(Entity, &Name, &Children), Added<MindControlled>>,
    daze_effect_query: Query<Entity, With<DazeAnimation>>,
) {
    for (entity, _name, children) in mind_controled_query.iter() {
        commands.entity(entity).remove::<Dazed>();
        for child in children {
            match daze_effect_query.get(*child) {
                Err(_) => continue,
                Ok(daze_effect) => {
                    // XXX: it doesn't remove the link to their parent
                    commands.entity(daze_effect).despawn();
                }
            }
        }
    }
}
