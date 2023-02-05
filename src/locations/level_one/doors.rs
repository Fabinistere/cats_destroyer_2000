use bevy::prelude::*;

use crate::{npc::NPC, player::Player, spritesheet::AnimationTimer};

pub fn open_doors_event() {
    // commands.ent(...).insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Once)));
}

pub fn animate_doors(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        (Without<Player>, Without<NPC>),
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
