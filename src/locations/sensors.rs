use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    locations::{
        level_one::{
            button::ButtonSensor,
            doors::{Door, OpenDoorEvent},
            CharacterLocation, LevelOneLocation,
        },
        PlayerLocation,
    },
    movement::CharacterHitbox,
    npc::NPC,
    player::Player,
    tablet::hack::Hackable,
};

/// Happens when
///   -
/// Read in
///   -
/// DOC
pub struct WinTriggerEvent {
    /// The Entity which triggered the WinEvent
    pub entity: Entity,
}

#[derive(Component)]
pub struct WinSensor;

#[derive(Component)]
pub struct LocationSensor {
    pub location: LevelOneLocation,
}

/// Enter the elevator to trigger the win
pub fn win_event(
    mut collision_events: EventReader<CollisionEvent>,

    win_sensor_query: Query<Entity, With<WinSensor>>,
    character_hitbox_query: Query<(Entity, &Parent), With<CharacterHitbox>>,

    mut win_trigger_event: EventWriter<WinTriggerEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let win_sensor = win_sensor_query.single();
                if *e1 == win_sensor || *e2 == win_sensor {
                    // if this is a NPC AltWin with a the npc leaving the building happy
                    // and the character still in.
                    match (
                        character_hitbox_query.get(*e1),
                        character_hitbox_query.get(*e2),
                    ) {
                        (Err(_), Ok((_, character))) | (Ok((_, character)), Err(_)) => {
                            win_trigger_event.send(WinTriggerEvent {
                                entity: **character,
                            });
                        }
                        _ => continue, // warn!("Neither {:?} or {:?} is a CharacterHitbox", *e1, *e2),
                    }
                }
            }
            _ => continue,
        }
    }
}

pub fn win_trigger(
    mut win_trigger_event: EventReader<WinTriggerEvent>,

    character_query: Query<&Name, Or<(With<Player>, With<NPC>)>>,
    mut player_location: ResMut<State<PlayerLocation>>,
) {
    for event in win_trigger_event.iter() {
        match character_query.get(event.entity) {
            Err(e) => warn!("The Winner is neither a NPC or a Player... {:?}", e),
            Ok(name) => {
                let congrats = format!("BIEN JOUE {}!", name);
                println!("{}", congrats);
            }
        }
        // TODO: 'increment" the level
        if *player_location.current() == PlayerLocation::LevelOne {
            player_location.set(PlayerLocation::LevelTwo).unwrap();
        }
    }
}

/// Manage where characters are
pub fn location_event(
    mut collision_events: EventReader<CollisionEvent>,

    location_sensor_query: Query<(Entity, &LocationSensor)>,
    character_hitbox_query: Query<(Entity, &Parent), With<CharacterHitbox>>,

    mut character_location_query: Query<
        (Entity, &mut CharacterLocation),
        Or<(With<Player>, With<NPC>)>,
    >,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                match (
                    character_hitbox_query.get(*e1),
                    character_hitbox_query.get(*e2),
                    location_sensor_query.get(*e1),
                    location_sensor_query.get(*e2),
                ) {
                    (
                        Ok((character_hitbox, character)),
                        Err(_),
                        Err(_),
                        Ok((location_sensor, location_point)),
                    )
                    | (
                        Err(_),
                        Ok((character_hitbox, character)),
                        Ok((location_sensor, location_point)),
                        Err(_),
                    ) => {
                        if (*e1 == location_sensor && *e2 == character_hitbox)
                            || (*e1 == character_hitbox && *e2 == location_sensor)
                        {
                            match character_location_query.get_mut(**character) {
                                Err(e) => warn!("Lost Character Hitbox {:?}", e),
                                // Updates the location of the character who cross the sensor
                                Ok((_, mut location)) => location.0 = location_point.location,
                            }
                            break;
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
}

/// Enter the button to trigger the win
pub fn button_event(
    mut collision_events: EventReader<CollisionEvent>,

    button_sensor_query: Query<Entity, With<ButtonSensor>>,
    // can be a npc or player
    character_hitbox_query: Query<Entity, With<CharacterHitbox>>,

    secured_door_query: Query<Entity, (With<Door>, Without<Hackable>)>,
    mut open_door_event: EventWriter<OpenDoorEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let button_sensor = button_sensor_query.single();
                // for the LevelOne: could be a single
                for character_hitbox in character_hitbox_query.iter() {
                    if (*e1 == button_sensor && *e2 == character_hitbox)
                        || (*e1 == character_hitbox && *e2 == button_sensor)
                    {
                        for door in secured_door_query.iter() {
                            open_door_event.send(OpenDoorEvent(door));
                        }

                        // The character hitbox has been found
                        break;
                    }
                }
            }
            _ => continue,
        }
    }
}
