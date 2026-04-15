/*
 * File: materials.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use std::marker::PhantomData;

use bevy::prelude::*;

use crate::render::prelude::*;

/// [`PointLight2d`](bevy_fast_light::prelude::PointLight2d) shadow for all type `T`.
///
/// The size of the [`Mesh`] is meant to be derived from [`CollisionDataCache`](crate::physics::prelude::CollisionDataCache).
#[derive(Resource, Default)]
pub(crate) struct Light2dShadow<T>
where
    T: Visible,
{
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) y_offset: f32,
    pub(crate) _phantom: PhantomData<T>,
}
