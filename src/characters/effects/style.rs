//! Particules and polished stuffs

// DOC: Rename Style module ?

use bevy::prelude::*;

use crate::{
    characters::movement::Dazed,
    constants::character::effects::{DAZE_STARTING_ANIM, DAZE_Y_OFFSET},
    spritesheet::{AnimationTimer, DazeSheet},
};

#[derive(Component)]
pub struct DazeAnimation;

/// Polish - floating Stars above their head
pub fn add_dazed_effect(
    mut commands: Commands,
    dazed_character_query: Query<Entity, Added<Dazed>>,
    effects_spritesheet: Res<DazeSheet>,
) {
    for entity in dazed_character_query.iter() {
        // whatever the entity
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: DAZE_STARTING_ANIM,
                        ..default()
                    },
                    texture_atlas: effects_spritesheet.0.clone(),
                    transform: Transform {
                        translation: Vec3::from(DAZE_Y_OFFSET),
                        scale: Vec3::splat(0.5),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Daze Anim"),
                DazeAnimation,
                // -- Animation --
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                // AnimState {
                //     initial: DAZE_STARTING_ANIM,
                //     current: DAZE_STARTING_ANIM,
                // },
            ));
        });
    }
}

// pub fn remove_daze_effect(daze_removal: RemovedComponents<Dazed>) {}

pub fn animate_dazed_effect(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut daze_effect_query: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<DazeAnimation>,
    >,
) {
    for (_daze_id, mut timer, mut sprite, texture_atlas_handle) in &mut daze_effect_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
