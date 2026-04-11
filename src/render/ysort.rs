/*
 * File: ysort.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;

use crate::{images::prelude::*, levels::prelude::*, procgen::prelude::*, render::prelude::*};

/// Sorts entities by their y position.
#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct YSort(pub(crate) f32);

/// Applies an offset to the [`YSort`].
///
/// The offset is expected to be in px.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct YSortOffset(pub(crate) f32);

// FIXME: We currently can't use Changed<Transform> because we always need to update z-level based on relative position.
/// Y-sort `T` [`Entity`]s.
pub(super) fn relative_sort<T, A>(
    query: Query<(&mut Transform, &YSort, Option<&YSortOffset>), With<T>>,
    cache: Res<ProcGenCache<A>>,
    image_size: Res<ImageSize<T>>,
    level_dimensions: Res<LevelDimensions<A>>,
) where
    T: Visible,
    A: ProcGenerated,
{
    let min_world_y = cache.min_chunk_pos().y as f32 * level_dimensions.chunk_size_px.y;
    // NOTE: We could also just divide by `world_height`, but multiplying `world_height` by 2 ensures that we never
    //       add/subtract more than 1 to `sort.0`.
    //       This also helps with keeping `BACKGROUND_Z_DELTA` low while making sure that it is displayed behind
    //       all visible objects on the canvas.
    let scale_divisor = level_dimensions.world_height * 2.;
    let texture_offset = image_size.size.y as f32 / 2.;

    for (mut transform, sort, sort_offset) in query {
        let sort_offset = sort_offset.map_or(0., |offset| offset.0);
        let relative_y = transform.translation.y - min_world_y;

        transform.translation.z =
            sort.0 - (relative_y - texture_offset + sort_offset) / scale_divisor;
    }
}
