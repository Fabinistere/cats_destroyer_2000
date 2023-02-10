use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    locations::{
        level_one::{CharacterLocation, LevelOneLocation},
        PlayerLocation,
    },
    movement::CharacterHitbox,
    npc::NPC,
    player::{Player, PlayerHitbox},
};

pub struct WinTriggerEvent;

#[derive(Component)]
pub struct ElevatorSensor;

#[derive(Component)]
pub struct LocationSensor {
    pub location: LevelOneLocation,
}
pub fn elevator_events(
    mut collision_events: EventReader<CollisionEvent>,

    elevator_sensor_query: Query<Entity, With<ElevatorSensor>>,
    player_hitbox_query: Query<Entity, With<PlayerHitbox>>,

    mut win_trigger_event: EventWriter<WinTriggerEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let elevator_sensor = elevator_sensor_query.single();
                let player_hitbox = player_hitbox_query.single();

                if (*e1 == elevator_sensor && *e2 == player_hitbox)
                    || (*e1 == player_hitbox && *e2 == elevator_sensor)
                {
                    win_trigger_event.send(WinTriggerEvent);
                }
            }
            CollisionEvent::Stopped(_e1, _e2, _) => {
                // let elevator_sensor = elevator_sensor_query.single();

                // if *e1 == elevator_sensor || *e2 == elevator_sensor {
                //     win_trigger_event.send(WinTriggerEvent);
                // }
            }
        }
    }
}

pub fn win_trigger(
    // player_query: Query<&Transform, With<Player>>,
    mut player_location: ResMut<State<PlayerLocation>>,
    mut win_trigger_event: EventReader<WinTriggerEvent>,
) {
    for _event in win_trigger_event.iter() {
        println!("BIEN JOUE !");
        // TODO: 'increment" the level
        if *player_location.current() == PlayerLocation::LevelOne {
            player_location.set(PlayerLocation::LevelTwo).unwrap();
        }
    }
}

/// Manage where characters are
pub fn location_events(
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
