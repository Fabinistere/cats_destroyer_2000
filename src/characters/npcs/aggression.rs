//! Scientifics Cats are pretty tough

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        npcs::movement::{ChaseBehavior, Target, WalkBehavior},
        player::{Player, PlayerHitbox},
    },
    collisions::CollisionEventExt,
    locations::level_one::CharacterLocation,
    tablet::mind_control::MindControled,
};

use super::{
    movement::{NewDirectionEvent, ResetAggroEvent},
    NPC,
};

// :0

// Happens when:
//   - ??? (npc::movement::pursue)
//     - target is not found/exist
//     - target is reach
// Read in ??? (npc::aggression::remove_pursuit_urge)
// pub struct StopChaseEvent {
//     pub npc_entity: Entity,
// }

/// Happens when:
///   - npc::aggression::player_detection
///     - An npc detected a enemy
///       in the same Area
///
/// Read in
///   - npc::aggression::add_pursuit_urge
///     - remove DetectionBehavior from the entity
///     - insert PursuitBehavior into the entity
///     - insert the Target into the entity
#[derive(Event)]
pub struct EngagePursuitEvent {
    npc_entity: Entity,
    target_entity: Entity,
}

#[derive(Component)]
pub struct DetectionSensor;

/// Pursuit Management
///
///   - Engagement
///     - targeting after the detection event
///   - Disengagement
///     - If the target outran the chaser remove the PursuitBehavior
pub fn player_detection(
    mut collision_events: EventReader<CollisionEvent>,

    rapier_context: Res<RapierContext>,

    detection_sensor_query: Query<
        (Entity, &Parent),
        (With<Collider>, With<Sensor>, With<DetectionSensor>),
    >,
    player_hitbox_query: Query<(Entity, &Parent), (With<Collider>, With<PlayerHitbox>)>,

    character_query: Query<(Entity, &CharacterLocation), Or<(With<Player>, With<NPC>)>>,

    mut ev_engage_pursuit: EventWriter<EngagePursuitEvent>,
) {
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
            // DEBUG: info!(target: "Collision Event with a sensor involved", "{:?} and {:?}", entity_1, entity_2);
            // check if the sensor is a DetectionSensor
            match (
                detection_sensor_query.get(entity_1),
                detection_sensor_query.get(entity_2),
                player_hitbox_query.get(entity_1),
                player_hitbox_query.get(entity_2),
            ) {
                // only one of them contains DetectionSensor: detection_sensor
                // and the other one is a player_hitbox
                (Ok(detection_sensor), Err(_e1), Err(_e2), Ok(player_hitbox))
                | (Err(_e1), Ok(detection_sensor), Ok(player_hitbox), Err(_e2)) => {
                    // DEBUG: info!(target: "Collision with a sensor and a player hitbox", "{:?} and {:?}", detection_sensor, player_hitbox);

                    match character_query.get_many([**detection_sensor.1, **player_hitbox.1]) {
                        Err(e) => warn!("{:?}", e),
                        Ok([(npc, npc_location), (player, player_location)]) => {
                            if npc_location.0 == player_location.0 {
                                // [detection_sensor, player_hitbox].1 returns the Parent Entity
                                // Only Bad Cats have a DetectionSensor
                                // Only the Player have a PlayerHitbox
                                ev_engage_pursuit.send(EngagePursuitEvent {
                                    npc_entity: npc,
                                    target_entity: player,
                                });
                            }
                        }
                    }
                }
                // two are sensors
                // or
                // two are errors
                _ => continue,
            }
        }
    }
}

/// Insert the new target into the npc
pub fn add_pursuit_urge(
    mut commands: Commands,
    mut ev_engage_pursuit: EventReader<EngagePursuitEvent>,
    mut npc_query: Query<(Entity, &mut Target, &Name), (With<NPC>, Without<MindControled>)>,
) {
    for ev in ev_engage_pursuit.iter() {
        match npc_query.get_mut(ev.npc_entity) {
            Err(_) => continue,
            Ok((npc_entity, mut target, npc_name)) => {
                info!("add pursuit urge to {}", npc_name);
                commands
                    .entity(npc_entity)
                    .insert(ChaseBehavior)
                    .remove::<WalkBehavior>();
                target.0 = Some(ev.target_entity);
            }
        }
    }
}

/// The npc returns to walk peacefully
///
/// - Remove ChaseBehavior
/// - Insert WalkBehavior
/// - Ask for a new destination
pub fn reset_aggro(
    mut commands: Commands,
    mut reset_aggro_event: EventReader<ResetAggroEvent>,

    mut new_direction_event: EventWriter<NewDirectionEvent>,
) {
    for ResetAggroEvent { npc } in reset_aggro_event.iter() {
        commands.entity(*npc).remove::<ChaseBehavior>();
        commands.entity(*npc).insert(WalkBehavior);
        new_direction_event.send(NewDirectionEvent(*npc));
    }
}
