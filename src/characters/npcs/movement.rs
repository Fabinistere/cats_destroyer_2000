use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::{
        effects::style::DazeAnimation,
        movement::{Dazed, Speed},
    },
    characters::{npcs::NPC, player::Player},
    constants::character::npc::movement::BLACK_CAT_STARTING_POSITION,
    tablet::mind_control::MindControled,
};

#[derive(Component)]
pub struct WalkBehavior;

#[derive(Component)]
pub struct ChaseBehavior;

/// DOC
// pub struct FreezeEvent;

/// DOC
pub struct NewDirectionEvent(pub Entity);

/// Happens in
///   - npc::movement::npc_chase
///     - target in ChaseBehavior is not a player
///     - target is reached
/// Read in
///   - npc::movement::reset_aggro
///     - Remove ChaseBehavior
///     Insert WalkBehavior
///     Ask for a new destination
pub struct ResetAggroEvent {
    pub npc: Entity,
}

#[derive(Clone, Copy, Component)]
pub struct Target(pub Option<Entity>);

impl Default for Target {
    fn default() -> Self {
        Target { 0: None }
    }
}

/// # Note
///
/// TODO: feature - Without MindControled
pub fn npc_walk(
    // mut commands: Commands,
    mut npc_query: Query<
        (Entity, &Transform, &Target, &Name),
        (
            With<NPC>,
            With<WalkBehavior>,
            Without<MindControled>,
            Without<Dazed>,
        ),
    >,
    target_query: Query<(Entity, &Transform)>,

    mut new_direction_event: EventWriter<NewDirectionEvent>,
) {
    for (npc, npc_transform, target, _npc_name) in npc_query.iter_mut() {
        match target_query.get(target.0.unwrap()) {
            Err(_) => new_direction_event.send(NewDirectionEvent(npc)),
            Ok((_, target_transform)) => {
                let direction: Vec3 = target_transform.translation;

                let close_range_width = npc_transform.scale.x * 10.;
                let close_range_height = npc_transform.scale.y * 10.;

                // The npc reached destination
                if direction.x - close_range_width < npc_transform.translation.x
                    && direction.x + close_range_width > npc_transform.translation.x
                    && direction.y - close_range_height < npc_transform.translation.y
                    && direction.y + close_range_height > npc_transform.translation.y
                {
                    // info!("{} reached destination", npc_name);
                    new_direction_event.send(NewDirectionEvent(npc));
                    // commands.entity(npc).insert(Clicked);
                } else {
                    // The npc has to walk
                    // Managed by npc::movement::npc_walk_to
                }
            }
        }
    }
}

/// # Note
///
/// TODO: feature - Without MindControled
pub fn npc_chase(
    mut npc_query: Query<
        (Entity, &Transform, &Target, &Name),
        (
            With<NPC>,
            With<ChaseBehavior>,
            Without<MindControled>,
            Without<Dazed>,
        ),
    >,
    player_query: Query<(Entity, &Transform, &Name), With<Player>>,

    mut reset_aggro_event: EventWriter<ResetAggroEvent>,
    // mut reset_level_event: EventWriter<ResetLevelOneEvent>,
) {
    for (npc, npc_transform, target, npc_name) in npc_query.iter_mut() {
        match player_query.get(target.0.unwrap()) {
            Err(e) => {
                warn!("target is not a player: {:?}", e);
                reset_aggro_event.send(ResetAggroEvent { npc });
            }
            Ok((_player, player_transform, player_name)) => {
                let direction = player_transform.translation;

                let close_range_width = npc_transform.scale.x * 10.;
                let close_range_height = npc_transform.scale.y * 10.;

                // TODO: feature - Cancel aggro when no longer in the same area
                // and insert Dazed ------^^^^^
                // TODO: feature ? - Confine NPC within certain area

                // The npc reached destination
                if direction.x - close_range_width < npc_transform.translation.x
                    && direction.x + close_range_width > npc_transform.translation.x
                    && direction.y - close_range_height < npc_transform.translation.y
                    && direction.y + close_range_height > npc_transform.translation.y
                {
                    info!("{}: Back to Horny Jail by {}", player_name, npc_name);
                    reset_aggro_event.send(ResetAggroEvent { npc });
                    // TODO: BAKC TO THE START with event
                    // reset_level_event.send(ResetLevelOneEvent);
                } else {
                    // The npc has to walk
                    // Managed by npc::movement::npc_walk_to
                }
            }
        }
    }
}

pub fn npc_walk_to(
    mut npc_query: Query<
        (Entity, &Transform, &Speed, &mut Velocity, &Target, &Name),
        (
            With<NPC>,
            Or<(With<ChaseBehavior>, With<WalkBehavior>)>,
            Without<MindControled>,
            Without<Dazed>,
        ),
    >,
    target_query: Query<(Entity, &Transform)>,
) {
    for (_npc, npc_transform, speed, mut rb_vel, target, _npc_name) in npc_query.iter_mut() {
        match target_query.get(target.0.unwrap()) {
            Err(_) => {}
            Ok((_target_entity, target_transform)) => {
                let destination = target_transform.translation;

                let up = destination.y > npc_transform.translation.y;
                let down = destination.y < npc_transform.translation.y;
                let left = destination.x < npc_transform.translation.x;
                let right = destination.x > npc_transform.translation.x;

                let x_axis = -(left as i8) + right as i8;
                let y_axis = -(down as i8) + up as i8;

                // println!("x: {}, y: {}", x_axis, y_axis);

                let mut vel_x = x_axis as f32 * **speed;
                let mut vel_y = y_axis as f32 * **speed;

                if x_axis != 0 && y_axis != 0 {
                    vel_x *= (std::f32::consts::PI / 4.).cos();
                    vel_y *= (std::f32::consts::PI / 4.).cos();
                }

                // TODO: gamefeel - make sure that the npc stop skape when approximate his position

                rb_vel.linvel.x = vel_x;
                rb_vel.linvel.y = vel_y;
            }
        }
    }
}

/// Event Handler of NewDirectionEvent
pub fn give_new_direction_event(
    mut commands: Commands,
    mut new_direction_event: EventReader<NewDirectionEvent>,

    mut npc_query: Query<(Entity, &Transform, &mut Target, &Name), (With<NPC>, With<WalkBehavior>)>,
    // REFACTOR: FOR NOW target can't be NPC - conflictual queries
    mut target_query: Query<(Entity, &mut Transform), (Without<Player>, Without<NPC>)>,
) {
    for event in new_direction_event.iter() {
        match npc_query.get_mut(event.0) {
            Err(e) => warn!("{:?}", e),
            Ok((_, npc_transform, mut target, name)) => {
                // creation of a Waypoint
                match target_query.get_mut(target.0.unwrap()) {
                    Err(e) => {
                        // resetAggro ?
                        warn!("None ?! {:?}", e);
                        // new_way_point spawns in the world.
                        let new_way_point = commands
                            .spawn((
                                SpatialBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        BLACK_CAT_STARTING_POSITION.0,
                                        BLACK_CAT_STARTING_POSITION.1 - 50.,
                                        0.,
                                    )),
                                    visibility: Visibility::Hidden,
                                    ..default()
                                },
                                Name::new(format!("WayPoint for {}", name)),
                            ))
                            .id();
                        target.0 = Some(new_way_point);
                    }
                    Ok((_, mut target_transform)) => {
                        // simple turn back: up and down
                        target_transform.translation = Vec3::new(
                            npc_transform.translation.x,
                            -target_transform.translation.y,
                            0.,
                        )
                    }
                }
            }
        }
    }
}

/// Decrement the daze Timer
pub fn daze_wait(
    mut commands: Commands,

    time: Res<Time>,
    mut npc_query: Query<
        (Entity, &mut Dazed, &mut Velocity, &Children, &Name),
        (With<NPC>, Without<Player>),
    >,
    daze_effect_query: Query<Entity, With<DazeAnimation>>,
) {
    for (npc, mut daze_timer, mut _rb_vel, children, name) in npc_query.iter_mut() {
        daze_timer.timer.tick(time.delta());

        // not required to control velocity because it is managed elsewhere

        if daze_timer.timer.finished() {
            info!("{:?}, {} can now aggro", npc, name);

            // REFACTOR: Abstract Daze Cure by event (also in daze_cure_by_mind_control())
            commands.entity(npc).remove::<Dazed>();
            for child in children {
                match daze_effect_query.get(*child) {
                    Err(_) => continue,
                    Ok(daze_effect) => {
                        commands.entity(daze_effect).despawn();
                    }
                }
            }
        }
    }
}
