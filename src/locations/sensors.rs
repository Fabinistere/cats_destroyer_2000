use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{locations::PlayerLocation, player::PlayerHitbox};

pub struct WinTriggerEvent;

#[derive(Component)]
pub struct ElevatorSensor;

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
