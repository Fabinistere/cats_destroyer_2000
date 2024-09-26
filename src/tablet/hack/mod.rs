use bevy::{prelude::*, winit::WinitSettings};
use bevy_tweening::{lens::UiPositionLens, Animator, EaseFunction, Tween, TweenCompleted};
use std::time::Duration;

use crate::{
    constants::ui::tablet::{
        HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, TABLET_ANIMATION_OFFSET,
        TABLET_ANIMATION_TIME_MS,
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
                button_system
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
    mut query: Query<(Entity, &mut Animator<Style>, &Style), With<Tablet>>,
) {
    if let Ok((entity, mut _animator, style)) = query.get_single_mut() {
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
    }
}

pub fn despawn_tablet(
    mut commands: Commands,
    mut completed_event: EventReader<TweenCompleted>,
    minimap_camera_query: Query<Entity, With<MiniMap>>,
) {
    for TweenCompleted { entity, user_data } in completed_event.read() {
        if *user_data == 0 {
            commands.entity(*entity).despawn_recursive();
            let minimap = minimap_camera_query.single();
            commands.entity(minimap).despawn();
        }
    }
}

// #[allow(clippy::too_many_lines)]
pub fn create_tablet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    player_camera_query: Query<Entity, With<PlayerCamera>>,
) {
    // Tablet's motion
    let tablet_tween = Tween::new(
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

    let player_camera = player_camera_query.single();
    let tablet = asset_server.load("textures/UI/Tablet.png");

    commands
        .spawn((
            ImageBundle {
                image: tablet.into(),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    top: Val::Px(0.),
                    left: Val::Px(TABLET_ANIMATION_OFFSET),
                    bottom: Val::Px(0.),
                    margin: UiRect {
                        right: Val::Auto,
                        left: Val::Px(0.),
                        top: Val::Auto,
                        bottom: Val::Px(0.),
                    },
                    // width: Val::Percent(100.),
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
        ))
        .with_children(|parent| {
            // -- Hack/Open the ALT_DOOR --
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(180.),
                            height: Val::Px(65.),
                            // center button
                            margin: UiRect::all(Val::Auto),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            top: Val::Percent(30.),
                            right: Val::Percent(-39.),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    Name::new("Hack Button"),
                    HackButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "HACK",
                        TextStyle {
                            font: asset_server.load("fonts/dpcomic.ttf"),
                            font_size: 40.,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });

    // Minimap
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after / on top of the main camera
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 11.)),
            projection: OrthographicProjection {
                scale: -0.2,
                ..default()
            },
            ..default()
        },
        MiniMap,
        Name::new("Tablet Camera"),
    ));
}

/// # Note
///
/// Spam proof (cause of the timer being only 0.1s)
///
/// REFACTOR: seperate color/text management from action
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<HackButton>),
    >,

    mut text_query: Query<&mut Text>,

    hackable_door: Query<Entity, (With<Door>, With<Hackable>)>,
    mut open_door_event: EventWriter<OpenDoorEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // hack every hackable door
                for door in hackable_door.iter() {
                    open_door_event.send(OpenDoorEvent(door));
                }

                text.sections[0].value = String::from("HOCK");
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = String::from("HACK");
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = String::from("HACK");
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
