use bevy::prelude::*;

pub struct CatSpritePlugin;

#[derive(Clone, Resource)]
pub struct CatSheet(pub Handle<TextureAtlas>);

impl Plugin for CatSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_character_spritesheet);
    }
}

fn load_character_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("textures/character/blue_cat.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(11.), 2, 1, None, None);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(CatSheet(atlas_handle));
}
