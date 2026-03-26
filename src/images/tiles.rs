/*
 * File: tiles.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use std::marker::PhantomData;

use bevy::{platform::collections::HashSet, prelude::*};
use serde::Deserialize;

use crate::procgen::prelude::*;

/// Tile data deserialized from a ron file.
///
/// ## Traits
///
/// - `T` must implement [`ProcGenerated`] and is used as a level's procedurally generated item.
#[derive(Deserialize, Asset, TypePath, Default)]
pub(crate) struct TileData<T>
where
    T: ProcGenerated,
{
    pub(crate) tile_size: f32,
    #[serde(default)]
    pub(crate) full_dirt: Option<HashSet<(usize, usize)>>,
    #[serde(default)]
    pub(crate) full_grass: Option<HashSet<(usize, usize)>>,
    #[serde(default)]
    pub(crate) corner_outer_grass_to_dirt: Option<HashSet<(usize, usize)>>,
    #[serde(default)]
    pub(crate) corner_outer_dirt_to_grass: Option<HashSet<(usize, usize)>>,
    #[serde(default)]
    pub(crate) side_dirt_and_grass: Option<HashSet<(usize, usize)>>,
    #[serde(default)]
    pub(crate) diag_stripe_grass_in_dirt: Option<HashSet<(usize, usize)>>,
    #[serde(skip)]
    pub(crate) _phantom: PhantomData<T>,
}

/// Handle for [`TileData`].
///
/// ## Traits
///
/// - `T` must implement [`ProcGenerated`] and is used as a level's procedurally generated item.
#[derive(Resource)]
pub(crate) struct TileHandle<T>(pub(crate) Handle<TileData<T>>)
where
    T: ProcGenerated;

/// Cache for [`TileData`]
///
/// This is to allow easier access.
///
/// ## Traits
///
/// - `T` must implement [`ProcGenerated`].
#[derive(Resource, Default)]
pub(crate) struct TileDataCache<T>
where
    T: ProcGenerated,
{
    pub(crate) tile_size: f32,
    pub(crate) _full_dirt: Option<HashSet<(usize, usize)>>,
    pub(crate) _full_grass: Option<HashSet<(usize, usize)>>,
    pub(crate) _corner_outer_grass_to_dirt: Option<HashSet<(usize, usize)>>,
    pub(crate) _corner_outer_dirt_to_grass: Option<HashSet<(usize, usize)>>,
    pub(crate) _side_dirt_and_grass: Option<HashSet<(usize, usize)>>,
    pub(crate) _diag_stripe_grass_in_dirt: Option<HashSet<(usize, usize)>>,
    pub(crate) _phantom: PhantomData<T>,
}
