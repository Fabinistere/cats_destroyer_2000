use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{
    player::Player,
    tablet::{
        hack::HackPlugin,
        mind_control::{MindControlPlugin, MindControled},
    },
};

pub mod hack;
pub mod mind_control;

pub struct TabletPlugin;

impl Plugin for TabletPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MindControlPlugin)
            .add_plugin(HackPlugin)
            ;
    }
}

/// REFACTOR: stop checking if free by the Entity Player not being MindControled ?
fn run_if_tablet_is_free(
    player_query: Query<Entity, (With<MindControled>, With<Player>)>,
) -> ShouldRun {
    match player_query.get_single() {
        Ok(_) => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

fn run_if_tablet_is_mind_ctrl(
    player_query: Query<Entity, (With<MindControled>, With<Player>)>,
) -> ShouldRun {
    match player_query.get_single() {
        Ok(_) => ShouldRun::No,
        _ => ShouldRun::Yes,
    }
}
