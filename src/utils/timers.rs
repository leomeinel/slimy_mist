/*
 * File: timers.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use std::ops::DerefMut;

use bevy::{ecs::component::Mutable, prelude::*};

/// Tick [`Timer`]s wrapped in [`Component`]s.
pub(crate) fn tick_component_timers<T>(mut query: Query<&mut T>, time: Res<Time>)
where
    T: Component<Mutability = Mutable> + DerefMut<Target = Timer>,
{
    for mut timer in &mut query {
        timer.tick(time.delta());
    }
}

/// Tick [`Timer`] wrapped in [`Resource`].
pub(crate) fn tick_resource_timer<T>(mut timer: ResMut<T>, time: Res<Time>)
where
    T: Resource + DerefMut<Target = Timer>,
{
    timer.tick(time.delta());
}

/// Remove [`Timer`]s wrapped in [`Component`]s using [`TimerMode::Once`] after they are finished.
pub(crate) fn remove_oneshot_component_timers<T>(query: Query<(Entity, &T)>, mut commands: Commands)
where
    T: Component<Mutability = Mutable> + DerefMut<Target = Timer>,
{
    for (entity, timer) in query {
        if timer.mode() == TimerMode::Once && timer.is_finished() {
            // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
            commands.entity(entity).try_remove::<T>();
        }
    }
}
