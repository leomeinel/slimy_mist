/*
 * File: nav.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * This is heavily inspired by: https://github.com/vleue/vleue_navigator
 */

use std::ops::Deref;

use bevy::{math::FloatPow, prelude::*};
use bevy_rapier2d::prelude::*;
use vleue_navigator::prelude::*;

use crate::{animations::prelude::*, characters::prelude::*, log::prelude::*, procgen::prelude::*};

/// Navigation target
///
/// [`NavTarget::0`] is meant as a priority. The higher it is, the more preferred the target is.
#[derive(Component, Default)]
pub(crate) struct NavTarget(pub(crate) u8);

/// Marker [`Component`] for a navigator that will pathfind to [`NavTarget`]
#[derive(Component)]
pub(crate) struct Navigator;

/// Path that is used for pathfinding to [`NavTarget`]
#[derive(Component)]
pub(crate) struct Path {
    pub(crate) current: Vec2,
    pub(crate) next: Vec<Vec2>,
    target: Entity,
}

/// [`EntityEvent`] to stop navigation.
///
/// This removes [`Path`] and switches to [`AnimationAction::Idle`].
#[derive(EntityEvent)]
pub(super) struct StopNav(Entity);

/// Find [`Path`] to [`NavTarget`]
pub(super) fn find_path<T>(
    navmesh: Single<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
    target_query: Query<(Entity, &Transform, &NavTarget), Without<Navigator>>,
    navigator_query: Query<
        (Entity, &Transform),
        (With<Navigator>, Without<Path>, Without<NavTarget>),
    >,
    mut commands: Commands,
    mut navmeshes: ResMut<Assets<NavMesh>>,
    mut delta: Local<f32>,
) where
    T: ProcGenerated,
{
    let (navmesh, status) = navmesh.deref();
    if **status != NavMeshStatus::Built && *delta == 0. {
        return;
    }
    let navmesh = navmeshes.get_mut(*navmesh).expect(ERR_INVALID_NAVMESH);

    // Get target with maximum priority
    let Some((target, target_pos, _)) = target_query.iter().max_by_key(|(_, _, t)| t.0) else {
        return;
    };
    let target_pos = target_pos.translation;
    if !navmesh.transformed_is_in_mesh(target_pos) {
        return;
    }

    let mut path_found = false;
    for (entity, transform) in &navigator_query {
        let Some((current, next)) =
            next_path_step(&mut delta, navmesh, transform.translation, target_pos)
        else {
            continue;
        };

        commands.entity(entity).insert(Path {
            current,
            next,
            target,
        });
        path_found = true;
    }
    if path_found {
        *delta = 0.
    }
}

/// Refresh [`Path`]
pub(super) fn refresh_path<T>(
    navmesh: Single<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
    navigator_query: Query<(Entity, &Transform, &mut Path), With<Navigator>>,
    target_transforms: Query<&Transform, With<NavTarget>>,
    mut commands: Commands,
    mut navmeshes: ResMut<Assets<NavMesh>>,
    mut delta: Local<f32>,
) where
    T: ProcGenerated,
{
    if target_transforms.is_empty() {
        return;
    }

    let (navmesh, status) = navmesh.deref();
    if **status != NavMeshStatus::Built && *delta == 0. {
        return;
    }
    let navmesh = navmeshes.get_mut(*navmesh).expect(ERR_INVALID_NAVMESH);

    let mut path_found = false;
    for (entity, transform, mut path) in navigator_query {
        let target_pos = target_transforms
            .get(path.target)
            .expect(ERR_INVALID_NAV_TARGET)
            .translation;
        if !navmesh.transformed_is_in_mesh(target_pos) {
            continue;
        };
        let Some((current, next)) =
            next_path_step(&mut delta, navmesh, transform.translation, target_pos)
        else {
            commands.trigger(StopNav(entity));
            continue;
        };

        path.current = current;
        path.next = next;
        path_found = true;
    }
    if path_found {
        *delta = 0.
    }
}

/// Next step for the [`Path`].
///
/// This also validates if `start` is inside of `navmesh`.
fn next_path_step(
    delta: &mut f32,
    navmesh: &mut NavMesh,
    start: Vec3,
    end: Vec3,
) -> Option<(Vec2, Vec<Vec2>)> {
    if !navmesh.transformed_is_in_mesh(start) {
        *delta += 0.1;
        navmesh.set_search_delta(*delta);
        return None;
    }

    let path = navmesh.transformed_path(start, end)?;
    let (first, remaining) = path.path.split_first()?;
    let mut next: Vec<_> = remaining.iter().map(|p| p.xy()).collect();
    next.reverse();

    Some((first.xy(), next))
}

/// Number used as divisor for path overshoot threshold
const PATH_OVERSHOOT_THRESHOLD_DIVISOR: f32 = 50.;

/// Apply [`Path`]
pub(super) fn apply_path(
    navigator_query: Query<
        (
            Entity,
            &Transform,
            &mut AnimationState,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
            &mut Path,
            &WalkSpeed,
        ),
        With<Navigator>,
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (
        entity,
        transform,
        mut animation_state,
        mut controller,
        controller_output,
        mut path,
        walk_speed,
    ) in navigator_query
    {
        let navigator_pos = transform.translation.xy();
        let direction = path.current - navigator_pos;
        let direction = direction.normalize_or_zero() * walk_speed.0 * time.delta_secs();
        controller.translation = Some(direction);

        // If `entity` collided with `path.target` stop applying path and return.
        // NOTE: This does not reliably determine whether the `entity` can not advance, just if it has collided with their target.
        //       For now this should be enough since not switching to `Idle` for these entities might cause the illusion of them
        //       still trying to wiggle their way around obstacles.
        if let Some(output) = controller_output
            && output.collisions.iter().any(|c| c.entity == path.target)
        {
            commands.trigger(StopNav(entity));
            return;
        }

        if animation_state.0.0 == AnimationAction::Idle {
            animation_state.set_new_action(AnimationAction::Walk);
        }

        // NOTE: We are looping until threshold to allow multiple next
        while navigator_pos.distance_squared(path.current)
            < (walk_speed.0 / PATH_OVERSHOOT_THRESHOLD_DIVISOR).squared()
        {
            if let Some(next) = path.next.pop() {
                path.current = next;
            } else {
                commands.trigger(StopNav(entity));
                break;
            }
        }
    }
}

/// Remove [`Path`] and set [`AnimationState`] state to [`AnimationAction::Idle`].
pub(super) fn on_stop_nav(
    event: On<StopNav>,
    mut animation_state_query: Query<&mut AnimationState, With<Navigator>>,
    mut commands: Commands,
) {
    let entity = event.0;
    let Ok(mut animation_state) = animation_state_query.get_mut(entity) else {
        return;
    };

    // NOTE: We are using `try_remove` to avoid use after despawn because of `procgen::despawn`.
    commands.entity(entity).try_remove::<Path>();
    animation_state.set_new_action(AnimationAction::Idle);
}
