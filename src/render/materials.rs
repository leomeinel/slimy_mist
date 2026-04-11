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

/// Artificial shadow for all type `T`.
///
/// The size of this is meant to be derived from [`CollisionDataCache`](crate::physics::prelude::CollisionDataCache).
#[derive(Resource, Default)]
pub(crate) struct ArtificialShadow<T>
where
    T: Visible,
{
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) material: Handle<ColorMaterial>,
    pub(crate) _phantom: PhantomData<T>,
}
