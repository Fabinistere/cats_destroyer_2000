//! Particules and polished stuffs

// TODO: Rename Style module ?

use bevy::prelude::*;

use super::movement::Dazed;

pub fn dazed_effect(dazed_npc_query: Query<Entity, With<Dazed>>) {
    for _npc in dazed_npc_query.iter() {
        // TODO: polish - floating Stars above their head
    }
}
