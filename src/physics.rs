pub(crate) mod prelude {
    pub(crate) use super::{CollisionData, CollisionDataCache, CollisionHandle};
}

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
    pub(crate) y_offset: Option<f32>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Handle for [`CollisionData`].
#[derive(Resource)]
pub(crate) struct CollisionHandle<T>(pub(crate) Handle<CollisionData<T>>)
where
    T: Visible;

// FIXME: This should be split up to allow easy access without imports for non-physics systems.
//        y_offset is currently only being used to offset other entities from the collision entity,
//        therefore the current approach does not make sense.
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
    pub(crate) y_offset: f32,
    pub(crate) _phantom: PhantomData<T>,
}
