use bevy::prelude::*;
use bevy_rapier2d::prelude::Sensor;

use crate::spritesheet::AnimationTimer;

/// Any door
#[derive(Component)]
pub struct Door {
    pub current_state: DoorState,
}

#[derive(Component)]
pub struct DoorHitbox;

#[derive(PartialEq)]
pub enum DoorState {
    Open,
    Closed,
    Opening,
    Closing,
}

/// The door which leads to the exit elevator
#[derive(Component)]
pub struct ExitDoor;

/// Happens when:
/// - [`location::sensors::button_event`]
///   - A character pushed the closet button
///     which will open all secured door.
/// - [`tablet::hack::button_event`]
///   - Hack button clicked, open the closet.
/// - [`tablet::hack::click_to_hack`]
///   - Hackable door clicked in the tablet interface,
///     open the clicked door.
///
/// Read in:
/// - [`location::level_one::opens::open_door_event`]
///   - Change the state of the given door
#[derive(Event)]
pub struct OpenDoorEvent(pub Entity);

/// Open the end of the level
pub fn open_door_event(
    mut commands: Commands,
    mut open_door_event: EventReader<OpenDoorEvent>,

    mut door_query: Query<(Entity, &mut Door)>,
) {
    for event in open_door_event.read() {
        match door_query.get_mut(event.0) {
            Err(e) => warn!("{:?}", e),
            Ok((door, mut door_state)) => {
                // IDEA: play 'LA PORTE' sfx

                // Reverse Action
                if door_state.current_state == DoorState::Opening {
                    door_state.current_state = DoorState::Closing;
                } else if door_state.current_state == DoorState::Closing {
                    door_state.current_state = DoorState::Opening;
                }
                commands
                    .entity(door)
                    .insert(AnimationTimer(Timer::from_seconds(
                        0.1,
                        TimerMode::Repeating,
                    )));
                // to click multiple time on Hack to open the door
                // AnimationTimer(Timer::from_seconds(0.1, TimerMode::Once))
            }
        }
    }
}

/// # Panics
///
/// Will panic if the door spritesheet has not been loaded.
pub fn animate_door(
    mut commands: Commands,

    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut door_query: Query<(
        Entity,
        &mut Door,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &Children,
    )>,

    door_hitbox_query: Query<Entity, With<DoorHitbox>>,
) {
    for (door_id, mut door, mut timer, mut atlas, children) in &mut door_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let atlas_layout_len = texture_atlases.get(&atlas.layout).unwrap().textures.len();

            if door.current_state == DoorState::Opening || door.current_state == DoorState::Closed {
                door.current_state = DoorState::Opening;

                let new_index = (atlas.index + 1) % atlas_layout_len;
                // if last frame
                if new_index == 0 {
                    door.current_state = DoorState::Open;
                    // stop the animation
                    commands.entity(door_id).remove::<AnimationTimer>();

                    // We assume that a door has only hitbox as a child
                    // Or: Create and verify the DoorHitbox Component
                    for child in children {
                        // can be a LocationSensor or somethign else
                        if door_hitbox_query.get(*child).is_ok() {
                            commands.entity(*child).insert(Sensor);
                        }
                    }
                } else {
                    atlas.index = new_index;
                }
            } else if door.current_state == DoorState::Closing
                || door.current_state == DoorState::Open
            {
                door.current_state = DoorState::Closing;

                atlas.index = (atlas.index - 1) % atlas_layout_len;
                // if first frame
                if atlas.index == 0 {
                    door.current_state = DoorState::Closed;
                    // stop the animation
                    commands.entity(door_id).remove::<AnimationTimer>();

                    // can be a LocationSensor or somethign else
                    for child in children {
                        if door_hitbox_query.get(*child).is_ok() {
                            commands.entity(*child).remove::<Sensor>();
                        }
                    }
                }
            }
        }
    }
}
