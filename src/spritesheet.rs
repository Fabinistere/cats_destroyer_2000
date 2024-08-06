use bevy::prelude::*;

use crate::characters::{npcs::NPC, player::Player};

pub struct CatSpritePlugin;

impl Plugin for CatSpritePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CatSheet>()
            .init_resource::<DazeSheet>()
            .add_systems(Update, animate_sprite);
    }
}

#[derive(Clone, Resource)]
pub struct CatSheet(pub Handle<TextureAtlas>);

impl FromWorld for CatSheet {
    fn from_world(world: &mut World) -> Self {
        let image = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load("textures/character/character_sheet_v1.png");
        // warn!("You have to download the asset see in github releases");
        let atlas = TextureAtlas::from_grid(image, Vec2::splat(14.), 2, 2, None, None);

        let atlas_handle = world
            .get_resource_mut::<Assets<TextureAtlas>>()
            .unwrap()
            .add(atlas);

        CatSheet(atlas_handle)
    }
}

/// DOC: Rename it to EffectSheet
#[derive(Clone, Resource)]
pub struct DazeSheet(pub Handle<TextureAtlas>);

impl FromWorld for DazeSheet {
    fn from_world(world: &mut World) -> Self {
        let dazed_image = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load("textures/character/dazed.png");
        // warn!("You have to download the asset see in github releases");
        let dazed_atlas =
            TextureAtlas::from_grid(dazed_image, Vec2::from((35., 25.)), 12, 1, None, None);

        let dazed_atlas_handle = world
            .get_resource_mut::<Assets<TextureAtlas>>()
            .unwrap()
            .add(dazed_atlas);

        DazeSheet(dazed_atlas_handle)
    }
}
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimState {
    pub initial: usize,
    pub current: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    _texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut AnimState,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        Or<(With<Player>, With<NPC>)>,
    >,
) {
    for (mut timer, mut state, mut sprite, _texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            // let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // disco cats:
            // sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            sprite.index = if state.current == state.initial {
                sprite.index + 1
            } else {
                sprite.index - 1
            };
            state.current = sprite.index;
        }
    }
}
