use bevy::prelude::*;

use crate::characters::{npcs::NPC, player::Player};

pub struct CatSpritePlugin;

impl Plugin for CatSpritePlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_character_spritesheet.in_base_set(StartupSet::PreStartup))
            .add_system(animate_sprite)
            ;
    }
}

#[derive(Clone, Resource)]
pub struct CatSheet(pub Handle<TextureAtlas>);

/// DOC: Rename it to EffectSheet
#[derive(Clone, Resource)]
pub struct DazeSheet(pub Handle<TextureAtlas>);

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

fn load_character_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("textures/character/character_sheet_v1.png");
    // warn!("You have to download the asset see in github releases");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(14.), 2, 2, None, None);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(CatSheet(atlas_handle));

    let dazed_image = assets.load("textures/character/dazed.png");
    let dazed_atlas =
        TextureAtlas::from_grid(dazed_image, Vec2::from((35., 25.)), 12, 1, None, None);

    let dazed_atlas_handle = texture_atlases.add(dazed_atlas);

    commands.insert_resource(DazeSheet(dazed_atlas_handle));
}
