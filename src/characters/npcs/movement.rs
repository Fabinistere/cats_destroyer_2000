use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    characters::{
        effects::style::DazeAnimation,
        movement::{Dazed, Speed},
        npcs::NPC,
        player::Player,
    },
    constants::character::effects::DAZE_TIMER,
    locations::{
        level_one::{CharacterLocation, WayPoint},
        Location,
    },
    tablet::mind_control::MindControlled,
};

#[derive(Component)]
pub struct WalkBehavior;

#[derive(Component)]
pub struct ChaseBehavior;

/// Happens when:
/// - [`characters::npcs::aggression::reset_aggro`]
///   - A NPC returns to walk peacefully.
/// - [`characters::npcs::movement::npc_walk_to`]
///   - The NPC don't have a target.
/// - [`characters::npcs::movement::npc_walk`]
///   - The NPC don't have a target.
///   - The NPC reached the way point.
///
/// Read in:
/// - [`characters::npcs::movement::give_new_way_point_event`]
///   - Find another waypoint for the npc to walk to.
#[derive(Event)]
pub struct NewWayPointEvent(pub Entity);

/// Happens in
///   - `npcs::movement::npc_chase`
///     - target in `ChaseBehavior` is not a player
///     - target is reached
///
/// Read in
///   - `npcs::aggresion::reset_aggro`
///     - Remove `ChaseBehavior`
///       Insert `WalkBehavior`
///       Ask for a new destination
#[derive(Event)]
pub struct ResetAggroEvent {
    pub npc: Entity,
}

#[derive(Clone, Copy, Default, Debug, Component)]
pub struct Target(pub Option<Entity>);

/// Control the npcs' movement
///
/// # Panics
///
/// `target.0.unwrap()` won't panic as `is_none` is tested just before.
///
/// Morever, it will panic if the target doesn't have the `Transform` component.
pub fn npc_walk_to(
    mut npc_query: Query<
        (Entity, &Transform, &Speed, &mut Velocity, &Target, &Name),
        (
            With<NPC>,
            Or<(With<ChaseBehavior>, With<WalkBehavior>)>,
            Without<MindControlled>,
            Without<Dazed>,
        ),
    >,
    transforms_query: Query<&Transform>,
    mut new_direction_event: EventWriter<NewWayPointEvent>,
) {
    for (npc, npc_transform, speed, mut rb_vel, target, _npc_name) in &mut npc_query {
        if target.0.is_none() {
            new_direction_event.send(NewWayPointEvent(npc));
            return;
        }

        let target_transform = transforms_query.get(target.0.unwrap()).unwrap();
        let destination = target_transform.translation;

        let up = destination.y > npc_transform.translation.y;
        let down = destination.y < npc_transform.translation.y;
        let left = destination.x < npc_transform.translation.x;
        let right = destination.x > npc_transform.translation.x;

        let x_axis = -i8::from(left) + i8::from(right);
        let y_axis = -i8::from(down) + i8::from(up);

        // println!("x: {}, y: {}", x_axis, y_axis);

        let mut vel_x = f32::from(x_axis) * **speed;
        let mut vel_y = f32::from(y_axis) * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.).cos();
            vel_y *= (std::f32::consts::PI / 4.).cos();
        }

        // TODO: gamefeel - make sure that the npc stop skape when approximate his position

        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;
    }
}

/// For all npcs with the `WalkBehavior`
///
/// # Panics
///
/// `target.0.unwrap()` won't panic as `is_none` is tested just before.
///
/// Morever, it will panic if the target doesn't have the `Transform` component.
pub fn npc_walk(
    mut npc_query: Query<
        (Entity, &Transform, &mut Target, &Name),
        (
            With<NPC>,
            With<WalkBehavior>,
            Without<MindControlled>,
            Without<Dazed>,
        ),
    >,
    transforms_query: Query<&Transform>,

    mut new_direction_event: EventWriter<NewWayPointEvent>,
) {
    for (npc, npc_transform, target, _npc_name) in &mut npc_query {
        if target.0.is_none() {
            new_direction_event.send(NewWayPointEvent(npc));
            return;
        }

        let target_transform = transforms_query.get(target.0.unwrap()).unwrap();
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
            new_direction_event.send(NewWayPointEvent(npc));
        } else {
            // The npc has to walk
            // Managed by npcs::movement::npc_walk_to
        }
    }
}

/// For all npcs with the `ChaseBehavior`
///
/// # Panics
///
/// `target.0.unwrap()` won't panic as `is_none` is tested just before.
///
/// Morever, it will panic if the target doesn't have the `Transform` component.
pub fn npc_chase(
    mut commands: Commands,
    mut npc_query: Query<
        (Entity, &Transform, &Target, &Name, &CharacterLocation),
        (
            With<NPC>,
            With<ChaseBehavior>,
            Without<MindControlled>,
            Without<Dazed>,
        ),
    >,
    player_query: Query<(&Transform, &Name, &CharacterLocation), With<Player>>,

    mut reset_aggro_event: EventWriter<ResetAggroEvent>,
    mut next_location: ResMut<NextState<Location>>,
) {
    for (npc, npc_transform, target, npc_name, npc_location) in &mut npc_query {
        if target.0.is_none() {
            reset_aggro_event.send(ResetAggroEvent { npc });
            return;
        }

        let (player_transform, player_name, player_location) =
            player_query.get(target.0.unwrap()).unwrap();

        let direction = player_transform.translation;

        let close_range_width = npc_transform.scale.x * 10.;
        let close_range_height = npc_transform.scale.y * 10.;

        // The npc reached destination
        if direction.x - close_range_width < npc_transform.translation.x
            && direction.x + close_range_width > npc_transform.translation.x
            && direction.y - close_range_height < npc_transform.translation.y
            && direction.y + close_range_height > npc_transform.translation.y
        {
            info!("{}: Back to the Horny Jail by {}", player_name, npc_name);
            reset_aggro_event.send(ResetAggroEvent { npc });
            next_location.set(Location::Void);

            // IDEA: feature - Cinematic flash: bandeau with the two characters + animation falling bars, lock up
        } else if npc_location != player_location {
            // info!("{}: Unreachable target - chaser: {}", player_name, npc_name);
            reset_aggro_event.send(ResetAggroEvent { npc });
            commands.entity(npc).insert(Dazed {
                timer: Timer::new(Duration::from_secs(DAZE_TIMER), TimerMode::Once),
            });
        } else {
            // The npc has to walk
            // Managed by npcs::movement::npc_walk_to
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                Event Handler                               */
/* -------------------------------------------------------------------------- */

/// Event Handler of `NewWayPointEvent`.
///
/// If the requested npc's target was already a way point give its a different one
/// Else give its a random way point
///
/// # Panics
///
/// It will panic if there is no waypoints registered (with the `WayPoint` component),
/// or if the current waypoint doesn't have the `Transfrom` component.
pub fn give_new_way_point_event(
    mut new_way_point_event: EventReader<NewWayPointEvent>,

    mut npc_query: Query<(&mut Target, &Name), (With<NPC>, With<WalkBehavior>)>,
    way_points_query: Query<Entity, With<WayPoint>>,
) {
    for NewWayPointEvent(npc) in new_way_point_event.read() {
        // The entity could have been despawned after a level change
        if let Ok((mut target, _name)) = npc_query.get_mut(*npc) {
            if target.0.is_none() {
                let mut rng = rand::thread_rng();
                target.0 = Some(way_points_query.iter().choose(&mut rng).unwrap());
            } else if let Ok(original_way_point) = way_points_query.get(target.0.unwrap()) {
                for way_point in way_points_query.iter() {
                    if way_point != original_way_point {
                        target.0 = Some(way_point);
                    }
                }
                // if no other waypoint is found, the npc will be stuck in the current
            } else {
                // the target is not a way point
                let mut rng = rand::thread_rng();
                target.0 = Some(way_points_query.iter().choose(&mut rng).unwrap());
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
    for (npc, mut daze_timer, mut _rb_vel, children, name) in &mut npc_query {
        daze_timer.timer.tick(time.delta());

        // not required to control velocity because it is managed elsewhere

        if daze_timer.timer.finished() {
            info!("{:?}, {} can now aggro", npc, name);

            // REFACTOR: Abstract Daze Cure by event (also in daze_cure_by_mind_control())
            commands.entity(npc).remove::<Dazed>();
            for child in children {
                if let Ok(daze_effect) = daze_effect_query.get(*child) {
                    commands.entity(daze_effect).despawn();
                }
            }
        }
    }
}
