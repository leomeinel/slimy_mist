/*
 * File: collision.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use std::marker::PhantomData;

use bevy::prelude::*;
use serde::Deserialize;

use crate::render::prelude::*;

/// Collision data deserialized from a ron file.
#[derive(Deserialize, Asset, TypePath, Default)]
pub(crate) struct CollisionData<T>
where
    T: Visible,
{
    #[serde(default)]
    pub(crate) shape: Option<String>,
    #[serde(default)]
    pub(crate) width: Option<f32>,
    #[serde(default)]
    pub(crate) height: Option<f32>,
    #[serde(default)]
    pub(crate) offset: Option<f32>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Handle for [`CollisionData`].
#[derive(Resource)]
pub(crate) struct CollisionHandle<T>(pub(crate) Handle<CollisionData<T>>)
where
    T: Visible;

/// Cache for [`CollisionData`]
///
/// This is to allow easier access.
#[derive(Resource, Default)]
pub(crate) struct CollisionDataCache<T>
where
    T: Visible,
{
    pub(crate) shape: String,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) offset: f32,
    pub(crate) _phantom: PhantomData<T>,
}
