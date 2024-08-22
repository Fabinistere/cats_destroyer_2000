use bevy::{prelude::*, winit::WinitSettings};

use crate::{
    constants::ui::tablet::{
        HOVERED_BUTTON, HOVERED_INACTIVE_BUTTON, INACTIVE_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
    },
    locations::{
        level_one::doors::{Door, OpenDoorEvent},
        Location,
    },
    tablet::{tablet_is_free, tablet_is_mind_ctrl},
};

pub struct HackPlugin;

impl Plugin for HackPlugin {
    // #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())
            .add_systems(OnEnter(Location::Level1000), setup_tablet_button)
            .add_systems(
                Update,
                (
                    button_system.run_if(tablet_is_free),
                    place_holder_while_in_mind_control.run_if(tablet_is_mind_ctrl),
                )
                    .run_if(in_state(Location::Level1000)),
            )
            .add_systems(OnExit(Location::Level1000), remove_tablet_button);
    }
}

#[derive(Component)]
pub struct Hackable;

#[derive(Component)]
#[allow(clippy::module_name_repetitions)]
pub struct HackButton;

fn remove_tablet_button(mut commands: Commands, tablet_query: Query<Entity, With<HackButton>>) {
    for button in tablet_query.iter() {
        commands.entity(button).despawn_recursive();
    }
}

pub fn setup_tablet_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 'Hack'/Open the ALT_DOOR
    commands
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
}

// pub fn hack_door(
//     mut open_doors_alt_event: EventWriter<OpenAltDoorsEvent>,
// ) {
// }

/// # Note
///
/// Spamming should not work (cause of the timer being only 0.1s)
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

/// # Panics
///
/// Will panic if the button doesn't have as first child a `Text`
pub fn place_holder_while_in_mind_control(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<HackButton>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_INACTIVE_BUTTON.into();
                text.sections[0].value = String::from("In MindCtrl");
            }
            Interaction::None => {
                *color = INACTIVE_BUTTON.into();
                text.sections[0].value = String::from("In MindCtrl");
            }
        }
    }
}
