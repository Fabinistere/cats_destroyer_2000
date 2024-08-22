//! This is a speedrun to have an cinematic in this game-mockup
//! Do not use it as example (not a good one at least)

use bevy::prelude::*;

use crate::{
    constants::{
        cinematics::{CLOUDS_LIMIT, CLOUDS_RESET, SECOND_CLOUDS_INIT},
        locations::LEVEL_SCALE,
        CLOUD_FRAME_TIME, FRAME_TIME,
    },
    spritesheet::AnimationTimer,
    tablet::mind_control::CurrentlyMindControlled,
};

/* -------------------------------------------------------------------------- */
/*                                    Final                                   */
/* -------------------------------------------------------------------------- */

/// Marker for the final cat sprite anim
#[derive(Component)]
pub struct Clouds;

/// Marker for the final cat sprite anim
#[derive(Component)]
pub struct PlayerHusk;

pub fn cinematic_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = 0.;
    camera_transform.translation.y = 3.;

    camera_transform.scale = Vec3::new(1.3, 1.3, 1.3);
    // TODO: adpat the camera to be centered on the endcinematic
}

pub fn spawn_cinematic_final(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut clear_color: ResMut<ClearColor>,

    currently_mind_controlled: Res<CurrentlyMindControlled>,
) {
    info!("Spawn Cinematic");

    clear_color.0 = Color::srgb(0.753, 0.126, 0.158);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/cinematics/final/FullClouds.png"),
            transform: Transform {
                translation: Vec3::from((CLOUDS_RESET, 0., 1.)),
                scale: Vec3::from(LEVEL_SCALE),
                ..default()
            },
            ..default()
        },
        Name::new("Cinematics - Clouds"),
        Clouds,
        // -- Animation --
        AnimationTimer(Timer::from_seconds(CLOUD_FRAME_TIME, TimerMode::Repeating)),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/cinematics/final/FullClouds.png"),
            transform: Transform {
                translation: Vec3::from((SECOND_CLOUDS_INIT, 0., 1.)),
                scale: Vec3::from(LEVEL_SCALE),
                ..default()
            },
            ..default()
        },
        Name::new("Cinematics - Clouds"),
        Clouds,
        // -- Animation --
        AnimationTimer(Timer::from_seconds(CLOUD_FRAME_TIME, TimerMode::Repeating)),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/cinematics/final/evasion-mountain-lab.png"),
            transform: Transform {
                translation: Vec3::from((0., 0., 2.)),
                scale: Vec3::from(LEVEL_SCALE),
                ..default()
            },
            ..default()
        },
        Name::new("Cinematics - Mountain Lab"),
    ));

    let cat_escape_image = if CurrentlyMindControlled::BlueCat == *currently_mind_controlled {
        info!("NormalEnd: Blue Cat Escaped");
        asset_server.load("textures/cinematics/final/blue-cat-sheet.png")
    } else {
        info!("EasterEgg: Black Cat Escaped");
        asset_server.load("textures/cinematics/final/black-cat-sheet.png")
    };

    let cat_escape_atlas = TextureAtlasLayout::from_grid(UVec2::from((19, 17)), 14, 1, None, None);
    let cat_escape_atlas_handle = texture_atlases.add(cat_escape_atlas);

    commands.spawn((
        SpriteBundle {
            texture: cat_escape_image,
            transform: Transform {
                translation: Vec3::from((-12., -12., 9.)),
                scale: Vec3::from(LEVEL_SCALE),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: cat_escape_atlas_handle,
            index: 0,
        },
        Name::new("Cinematics - Cat"),
        PlayerHusk,
        // -- Animation --
        AnimationTimer(Timer::from_seconds(FRAME_TIME, TimerMode::Repeating)),
    ));

    info!("Cinematic Spawned");
}

// TODO: spawn a Quit button after x seconds
// IDEA: when leaving with an enemy, scroll through all level with the enemy with a jazzy/chill song

/// Infinitly loop the clouds
pub fn animate_clouds(
    time: Res<Time>,
    mut clouds_query: Query<(&mut AnimationTimer, &mut Transform), With<Clouds>>,
) {
    for (mut timer, mut transform) in &mut clouds_query {
        timer.tick(time.delta());

        if timer.just_finished() {
            transform.translation.x = if transform.translation.x == CLOUDS_LIMIT {
                CLOUDS_RESET
            } else {
                transform.translation.x - 0.1
            };
        }
    }
}

/// # Panics
///
/// Will panic if the cat's spritesheet has not been loaded.
pub fn animate_free_cat(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut running_cat: Query<(&mut AnimationTimer, &mut TextureAtlas), With<PlayerHusk>>,
) {
    for (mut timer, mut atlas) in &mut running_cat {
        timer.tick(time.delta());

        if timer.just_finished() {
            let layout_len = texture_atlases.get(&atlas.layout).unwrap().textures.len();

            atlas.index = if atlas.index + 1 < layout_len {
                atlas.index + 1
            } else if atlas.index == layout_len - 1 {
                layout_len - 2
            } else {
                warn!("Overflow CatAnim FinalCinematic");
                layout_len - 2
            };
        }
    }
}
