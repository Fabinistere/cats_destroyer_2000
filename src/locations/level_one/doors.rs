use bevy::prelude::*;
use bevy_rapier2d::prelude::Sensor;

use crate::spritesheet::AnimationTimer;

/// Any door
#[derive(Component)]
pub struct Door {
    pub current_state: DoorState,
}

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

/// DOC
pub struct OpenDoorEvent(pub Entity);

/// Open the end of the level
pub fn open_door_event(
    mut commands: Commands,
    mut open_door_event: EventReader<OpenDoorEvent>,

    door_query: Query<Entity, With<Door>>,
) {
    for event in open_door_event.iter() {
        match door_query.get(event.0) {
            Err(e) => warn!("{:?}", e),
            Ok(door) => {
                // IDEA: play 'LA PORTE' sfx

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

pub fn animate_door(
    mut commands: Commands,

    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut Door,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Children,
    )>,
) {
    for (door_id, mut door, mut timer, mut sprite, texture_atlas_handle, children) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            if door.current_state == DoorState::Opening || door.current_state == DoorState::Closed {
                door.current_state = DoorState::Opening;

                let new_index = (sprite.index + 1) % texture_atlas.textures.len();
                // if last frame
                if new_index == 0 {
                    door.current_state = DoorState::Open;
                    // stop the animation
                    commands.entity(door_id).remove::<AnimationTimer>();

                    // We assume that a door has only hitbox as a child
                    // Or: Create and verify the DoorHitbox Component
                    for child in children.iter() {
                        commands.entity(*child).insert(Sensor);
                    }
                } else {
                    sprite.index = new_index;
                }
            } else if door.current_state == DoorState::Closing
                || door.current_state == DoorState::Open
            {
                door.current_state = DoorState::Closing;

                sprite.index = (sprite.index - 1) % texture_atlas.textures.len();
                // if first frame
                if sprite.index == 0 {
                    door.current_state = DoorState::Closed;
                    // stop the animation
                    commands.entity(door_id).remove::<AnimationTimer>();

                    for child in children.iter() {
                        commands.entity(*child).remove::<Sensor>();
                    }
                }
            }
        }
    }
}
