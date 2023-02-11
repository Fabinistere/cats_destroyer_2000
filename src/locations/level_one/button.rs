use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::locations::{
    level_one::{BUTTON_HITBOX_X_OFFSET, BUTTON_POSITION},
    LEVEL_SCALE,
};

/// # Note
///
/// Instead of creating struct to differenciate Doors,
/// Link Doors and Button:
/// Button { pub linked_doors: Vec<Entity> }
#[derive(Component)]
pub struct Button;

#[derive(Component)]
pub struct ButtonSensor;

// #[derive(PartialEq)]
// pub enum ButtonState {
//     Idle,
//     Pushed,
//     Pushing,
//     Releasing,
// }

pub fn set_up_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // -- Doors --

    let button = asset_server.load("textures/level_one/button-press.png");
    let button_atlas = TextureAtlas::from_grid(button, Vec2::new(8., 7.), 1, 7, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: button_atlas_handle,
                transform: Transform {
                    translation: BUTTON_POSITION.into(),
                    scale: LEVEL_SCALE.into(),
                    ..default()
                },
                ..default()
            },
            Name::new("OneWay Button"),
            Button,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(1., 3.5),
                Transform::from_translation(BUTTON_HITBOX_X_OFFSET.into()),
                Name::new("Button Hitbox"),
            ));
        });
}
