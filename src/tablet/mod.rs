use bevy::prelude::*;

use crate::{characters::player::Player, tablet::mind_control::MindControlled};

pub mod hack;
pub mod mind_control;

pub struct TabletPlugin;

impl Plugin for TabletPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((mind_control::MindControlPlugin, hack::HackPlugin));
    }
}

/// REFACTOR: stop checking if free by the Entity Player not being `MindControlled` ?
fn tablet_is_free(player_query: Query<Entity, (With<MindControlled>, With<Player>)>) -> bool {
    player_query.get_single().is_ok()
}

fn tablet_is_mind_ctrl(player_query: Query<Entity, (With<MindControlled>, With<Player>)>) -> bool {
    player_query.get_single().is_err()
}
