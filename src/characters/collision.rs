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
use bevy_rapier2d::prelude::*;

use crate::characters::prelude::*;

/// Collision data deserialized from a ron file as a generic
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(serde::Deserialize, Asset, TypePath, Default)]
pub(crate) struct CollisionData<T>
where
    T: Character,
{
    #[serde(default)]
    pub(crate) shape: Option<String>,
    #[serde(default)]
    pub(crate) width: Option<f32>,
    #[serde(default)]
    pub(crate) height: Option<f32>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Handle for [`CollisionData`] as a generic
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource)]
pub(crate) struct CollisionHandle<T>(pub(crate) Handle<CollisionData<T>>)
where
    T: Character;

/// Cache for [`CollisionData`]
///
/// This is to allow easier access.
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct CollisionDataCache<T>
where
    T: Character,
{
    pub(crate) shape: String,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) _phantom: PhantomData<T>,
}

/// Cache for data that is related to [`CollisionData`]
///
/// This is to allow easier access.
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct CollisionDataRelatedCache<T>
where
    T: Character,
{
    pub(crate) shadow: StaticShadow,
    pub(crate) _phantom: PhantomData<T>,
}

/// [`Collider`] for different shapes
pub(crate) fn character_collider(shape: String, width: f32, height: f32) -> Collider {
    // NOTE: For capsules, we just assume that the values are correct, meaning that for x: `width > height` and for y: `width < height`
    match shape.as_str() {
        "ball" => Collider::ball(width / 2.),
        "capsule_x" => Collider::capsule_x((height - width) / 2., height / 2.),
        "capsule_y" => Collider::capsule_y((width - height) / 2., width / 2.),
        _ => Collider::cuboid(width / 2., height / 2.),
    }
}
