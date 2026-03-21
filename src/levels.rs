/*
 * File: levels.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Game worlds

mod navmesh;
mod overworld;

pub(crate) mod prelude {
    pub(crate) use super::overworld::{Overworld, OverworldAssets, OverworldProcGen};
    pub(crate) use super::{Level, LevelAssets, LevelRng, impl_level_assets};
}

use bevy::{prelude::*, reflect::Reflectable};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{screens::prelude::*, utils::prelude::*};

pub(super) struct LevelsPlugin;
impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(navmesh::NavMeshPlugin);

        app.add_systems(Startup, setup_rng::<LevelRng>);
        app.add_systems(
            OnEnter(Screen::Gameplay),
            overworld::spawn_overworld.in_set(EnterGameplaySystems::Levels),
        );
    }
}

/// Applies to anything that is a level
pub(crate) trait Level
where
    Self: Component + Default + Reflectable,
{
}

/// Applies to anything that stores level assets
pub(crate) trait LevelAssets
where
    Self: AssetCollection + Resource + Default + Reflectable,
{
    fn music(&self) -> &Option<Vec<Handle<AudioSource>>>;
    fn tile_set(&self) -> &Handle<Image>;
}
macro_rules! impl_level_assets {
    ($type: ty) => {
        impl LevelAssets for $type {
            fn music(&self) -> &Option<Vec<Handle<AudioSource>>> {
                &self.music
            }
            fn tile_set(&self) -> &Handle<Image> {
                &self.tile_set
            }
        }
    };
}
pub(crate) use impl_level_assets;

/// Rng for levels
#[derive(Component, Default)]
pub(crate) struct LevelRng;
impl ForkedRng for LevelRng {}
