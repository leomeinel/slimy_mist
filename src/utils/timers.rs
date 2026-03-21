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

pub(crate) fn tick_component_timer<T>(mut query: Query<&mut T>, time: Res<Time>)
where
    T: Component<Mutability = Mutable> + DerefMut<Target = Timer>,
{
    for mut timer in &mut query {
        timer.tick(time.delta());
    }
}

pub(crate) fn tick_resource_timer<T>(mut timer: ResMut<T>, time: Res<Time>)
where
    T: Resource + DerefMut<Target = Timer>,
{
    timer.tick(time.delta());
}
