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
mod tiles;
mod transitions;

pub(crate) mod prelude {
    pub(crate) use super::layers::{DisplayLayers, LayerData, LayerDataCache, LayerHandle};
    pub(crate) use super::tiles::{TileData, TileDataCache, TileHandle};
    pub(crate) use super::transitions::{FadeInOut, apply_fade_in_out, tick_fade_in_out};
    pub(crate) use super::{ImageMeta, ImageSize};
}

use std::marker::PhantomData;

use bevy::{prelude::*, render::render_resource::*};
use bevy_ecs_tilemap::prelude::*;

use crate::{
    characters::prelude::*, images::prelude::*, log::prelude::*, render::prelude::*,
    screens::prelude::*,
};

pub(super) struct ImagesPlugin;
impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);

        app.add_systems(
            OnEnter(Screen::Gameplay),
                (
                insert_images_and_related::<Player>,
                insert_images_and_related::<Slime>,
                )
                    .in_set(EnterGameplaySystems::Sprites),
        );
    }
}

/// Image size.
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(Resource, Default)]
pub(crate) struct ImageSize<T>
where
    T: Visible,
{
    pub(crate) size: UVec2,
    pub(crate) _phantom: PhantomData<T>,
}
impl<T> From<ImageMeta<T>> for ImageSize<T>
where
    T: Visible,
{
    fn from(meta: ImageMeta<T>) -> Self {
        Self {
            size: UVec2::new(meta.size.width, meta.size.height),
            ..default()
        }
    }
}

/// Image metadata.
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
pub(crate) struct ImageMeta<T>
where
    T: Visible,
{
    pub(crate) size: Extent3d,
    pub(crate) dimension: TextureDimension,
    pub(crate) format: TextureFormat,
    pub(crate) _phantom: PhantomData<T>,
}
impl<T> ImageMeta<T>
where
    T: Visible,
{
    fn from_layer_data_cache(data: &LayerDataCache<T>, images: &mut ResMut<Assets<Image>>) -> Self {
        let (size, dimension, format) = data
        .base
            .first()
            .map(|image| {
                let descriptor = &images
                    .get(image)
                    .expect(ERR_INVALID_IMAGE)
                    .texture_descriptor;
                (descriptor.size, descriptor.dimension, descriptor.format)
            })
            .expect(ERR_INVALID_IMAGE);

        // Assert that all layers have the same metadata.
        assert!(data.base.iter().all(|image| {
            let descriptor = &images
                .get(image)
                .expect(ERR_INVALID_IMAGE)
                .texture_descriptor;
            (descriptor.size, descriptor.dimension, descriptor.format) == (size, dimension, format)
        }));
        assert!(data.floating.as_ref().is_none_or(|layers| {
            layers.iter().all(|image| {
                let descriptor = &images
                    .get(image)
                    .expect(ERR_INVALID_IMAGE)
                    .texture_descriptor;
                (descriptor.size, descriptor.dimension, descriptor.format)
                    == (size, dimension, format)
            })
        }));

        Self {
            size,
            dimension,
            format,
            _phantom: PhantomData,
        }
    }
}

/// Insert [`DisplayLayers`] and [`ImageSize`].
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
fn insert_images_and_related<T>(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    data: Res<LayerDataCache<T>>,
) where
    T: Visible,
{
    let meta = ImageMeta::from_layer_data_cache(&data, &mut images);
    commands.insert_resource(DisplayLayers::from_layer_data_cache(
        &data,
        &meta,
        &mut images,
    ));
    commands.insert_resource(ImageSize::from(meta));
}
