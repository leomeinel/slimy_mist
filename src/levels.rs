//! Game worlds

mod navmesh;
mod overworld;

pub(crate) mod prelude {
    pub(crate) use super::overworld::{Overworld, OverworldAssets, OverworldProcGen};
    pub(crate) use super::{Level, LevelAssets, LevelDimensions, LevelRng, impl_level_assets};
}

use std::marker::PhantomData;

use bevy::{prelude::*, reflect::Reflectable};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{procgen::prelude::*, screens::prelude::*, utils::prelude::*};

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

/// Dimensions for a [`Level`] of type `T`.
///
/// This is related to [`TileData`](crate::images::prelude::TileData).
#[derive(Resource, Default)]
pub(crate) struct LevelDimensions<T>
where
    T: ProcGenerated,
{
    pub(crate) chunk_size_px: Vec2,
    pub(crate) world_height: f32,
    pub(crate) _phantom: PhantomData<T>,
}
