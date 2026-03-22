/*
 * File: images.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

// FIXME: I'd like to have an easy way to use shaders for my sprites. This does currently not seem possible.
//        We also need this for showing an outline.
//        Also see:
//        - https://github.com/merwaaan/bevy_spritesheet_animation/issues/66
//        - https://github.com/bevyengine/bevy/pull/22484 (merged)
//            - Part of: https://github.com/bevyengine/bevy/milestone/40 (0.19)

mod layers;
mod sprites;
mod tiles;
mod transitions;

pub(crate) mod prelude {
    pub(crate) use super::ImageMeta;
    pub(crate) use super::layers::{DisplayImage, LayerData, LayerDataCache, LayerHandle};
    pub(crate) use super::tiles::{TileData, TileDataCache, TileHandle};
    pub(crate) use super::transitions::{FadeInOut, apply_fade_in_out, tick_fade_in_out};
}

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{characters::prelude::*, render::prelude::*, screens::prelude::*};

pub(super) struct ImagesPlugin;
impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);

        app.add_systems(
            OnEnter(Screen::Gameplay),
            (
                layers::insert_display_image::<Player>,
                layers::insert_display_image::<Slime>,
            )
                .in_set(EnterGameplaySystems::Sprites),
        );
        app.add_systems(
            Update,
            (
                sprites::flip_sprites::<Player>,
                sprites::flip_sprites::<Slime>,
            ),
        );
    }
}

/// Image metadata.
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(Resource, Default)]
pub(crate) struct ImageMeta<T>
where
    T: Visible,
{
    pub(crate) size: UVec2,
    pub(crate) _phantom: PhantomData<T>,
}
