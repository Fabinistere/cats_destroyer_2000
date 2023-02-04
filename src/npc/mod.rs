use crate::constants::character::npc::{*, movement::BLUE_CAT_POSITION};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::spritesheet::CatSheet;

#[derive(Default)]
pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_characters)
            .add_system(animate_sprite)
            ;
    }
}

#[derive(Component, Inspectable)]
pub struct NPC;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn spawn_characters(mut commands: Commands, cats: Res<CatSheet>) {
    // Blue Cat
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: BLUE_CAT_STARTING_ANIM,
                flip_x: true,
                ..default()
            },
            texture_atlas: cats.0.clone(),
            transform: Transform {
                translation: Vec3::from(BLUE_CAT_POSITION),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        },
        Name::new("Blue Cat"),
        NPC,
        // -- Animation --
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
