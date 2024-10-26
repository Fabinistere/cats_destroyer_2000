use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};
use bevy_tweening::{
    lens::{TransformPositionLens, UiPositionLens},
    Animator, EaseFunction, Tween, TweenCompleted,
};
use std::time::Duration;

use crate::{
    constants::ui::tablet::{
        MINIMAP_ANIMATION_OFFSET, MINI_MAP_Z, TABLET_ANIMATION_OFFSET, TABLET_ANIMATION_TIME_MS,
    },
    locations::{
        level_one::doors::{Door, OpenDoorEvent},
        Location,
    },
    HudState, PlayerCamera,
};

pub struct HackPlugin;

impl Plugin for HackPlugin {
    // #[rustfmt::skip]
    fn build(&self, app: &mut App) {
            app.insert_resource(WinitSettings::game())
                .add_systems(
                    Update,
                    (create_tablet_on_key_press, despawn_tablet).run_if(in_state(Location::Level1000)),
                )
                .add_systems(OnEnter(HudState::Tablet), create_tablet)
                .add_systems(
                    Update,
                    click_to_hack
                        // .run_if(tablet_is_free)
                        .run_if(in_state(Location::Level1000))
                        .run_if(in_state(HudState::Tablet)),
                )
                .add_systems(OnExit(HudState::Tablet), close_tablet)
                .add_systems(OnExit(Location::Level1000), close_tablet);
        }
    }

/* ------------------------------- Components ------------------------------- */

#[derive(Component)]
pub struct Hackable;

#[derive(Component)]
#[allow(clippy::module_name_repetitions)]
pub struct HackButton;

#[derive(Component)]
pub struct Tablet;

#[derive(Component)]
pub struct MiniMap;

/* --------------------------------- Systems -------------------------------- */

pub fn create_tablet_on_key_press(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    tablet_query: Query<(Entity, &Animator<Style>, &Style), With<Tablet>>,

    mut next_game_state: ResMut<NextState<HudState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok((_entity, animator, _style)) = tablet_query.get_single() {
            if animator.tweenable().progress() >= 1. {
                next_game_state.set(HudState::Closed);
            }
        } else {
            next_game_state.set(HudState::Tablet);
        }
    }
}

pub fn close_tablet(
    mut commands: Commands,
    mut tablet_query: Query<(Entity, &mut Animator<Style>, &Style), With<Tablet>>,
    minimap_camera_query: Query<Entity, With<MiniMap>>,
) {
    if let Ok((entity, mut _animator, style)) = tablet_query.get_single_mut() {
        let tablet_tween = Tween::new(
            EaseFunction::ExponentialIn, // EaseFunction::CircularIn,
            Duration::from_millis(TABLET_ANIMATION_TIME_MS),
            UiPositionLens {
                start: UiRect {
                    left: style.left,
                    right: style.right,
                    top: style.top,
                    bottom: style.bottom,
                },
                end: UiRect {
                    top: Val::Auto,
                    right: Val::Auto,
                    left: Val::Px(TABLET_ANIMATION_OFFSET),
                    bottom: Val::Px(TABLET_ANIMATION_OFFSET),
                },
            },
        )
        .with_completed_event(0);

        commands
            .entity(entity)
            .remove::<Animator<Style>>()
            .insert(Animator::new(tablet_tween));

        // minimap
        let minimap = minimap_camera_query.single();
        let minimap_tween = Tween::new(
            EaseFunction::ExponentialIn, // EaseFunction::CircularIn,
            Duration::from_millis(TABLET_ANIMATION_TIME_MS),
            TransformPositionLens {
                start: Vec3::new(0., 0., MINI_MAP_Z),
                // left bottom
                end: MINIMAP_ANIMATION_OFFSET.into(),
            },
        )
        .with_completed_event(0);

        commands
            .entity(minimap)
            .remove::<Animator<Transform>>()
            .insert(Animator::new(minimap_tween));
    }
}

/// Despawn the tablet and its camera once the animation is done.
pub fn despawn_tablet(mut commands: Commands, mut completed_event: EventReader<TweenCompleted>) {
    for TweenCompleted { entity, user_data } in completed_event.read() {
        if *user_data == 0 {
            commands.entity(*entity).despawn_recursive();
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn create_tablet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    player_camera_query: Query<Entity, With<PlayerCamera>>,
) {
    // Tablet's motion
    let tablet_motion = (
        EaseFunction::ExponentialOut, // EaseFunction::CircularOut,
        Duration::from_millis(TABLET_ANIMATION_TIME_MS),
        UiPositionLens {
            start: UiRect {
                top: Val::Auto,
                right: Val::Auto,
                left: Val::Px(TABLET_ANIMATION_OFFSET),
                bottom: Val::Px(TABLET_ANIMATION_OFFSET),
            },
            end: UiRect {
                top: Val::Auto,
                right: Val::Auto,
                left: Val::Px(0.),
                bottom: Val::Px(0.),
            },
        },
    );
    let tablet_tween = Tween::new(tablet_motion.0, tablet_motion.1, tablet_motion.2);

    let player_camera = player_camera_query.single();
    let tablet = asset_server.load("textures/UI/Tablet.png");

    commands.spawn((
        ImageBundle {
            image: tablet.into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Relative,
                // top: Val::Px(0.),
                // left: Val::Px(TABLET_ANIMATION_OFFSET),
                // bottom: Val::Px(0.),
                margin: UiRect {
                    right: Val::Auto,
                    left: Val::Px(0.),
                    top: Val::Auto,
                    bottom: Val::Px(0.),
                },
                height: Val::Percent(100.),
                aspect_ratio: Some(16. / 9.),
                ..default()
            },
            ..default()
        },
        TargetCamera(player_camera),
        Tablet,
        Animator::new(tablet_tween),
        Name::new("Tablet"),
    ));

    // Minimap
    let minimap_tween = Tween::new(
        tablet_motion.0,
        tablet_motion.1,
        TransformPositionLens {
            // left bottom
            start: MINIMAP_ANIMATION_OFFSET.into(),
            end: Vec3::new(0., 0., MINI_MAP_Z),
        },
    );

    // BUG: Visual - Sometimes an asset (BlackCat) is not being drawn outside the PlayerCamera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after / on top of the main camera
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., MINI_MAP_Z)),
            projection: OrthographicProjection {
                scale: -0.2,
                // near: -1000.,
                ..default()
            },
            ..default()
        },
        Animator::new(minimap_tween),
        MiniMap,
        Name::new("Tablet Camera"),
    ));
}

/// Runs in [`main::HudState::Tablet`], track the cursor and all hackable object
///
/// ## Notes
///
/// IDEA: polish - bigger trigger, area around the object
fn click_to_hack(
    hackable_doors_query: Query<(Entity, &Transform, &Sprite), (With<Door>, With<Hackable>)>,
    tablet_camera_query: Query<(&Camera, &GlobalTransform), With<MiniMap>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut open_door_event: EventWriter<OpenDoorEvent>,
) {
    let (camera, camera_transform) = tablet_camera_query.single();
    let window = q_windows.single();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        if mouse_input.just_pressed(MouseButton::Left) {
            // info!("click at {cursor_position:#?}");
            for (door, transform, sprite) in hackable_doors_query.iter() {
                let sprite_size = sprite.custom_size.unwrap_or(Vec2::new(4., 14.));
                let entity_position = transform.translation.truncate();

                let half_size = sprite_size / 2.0;
                let min_bounds = entity_position - half_size;
                let max_bounds = entity_position + half_size;

                if cursor_position.x > min_bounds.x
                    && cursor_position.x < max_bounds.x
                    && cursor_position.y > min_bounds.y
                    && cursor_position.y < max_bounds.y
                {
                    open_door_event.send(OpenDoorEvent(door));
                    // sprite.color = Color::srgb(0., 1., 0.);
                }
            }
        }
    }
}
