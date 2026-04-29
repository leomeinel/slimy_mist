/*
 * File: images.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

// FIXME: I'd like to have an easy way to use shaders for my sprites. This does currently not seem possible.
//        Also see:
//        - https://github.com/merwaaan/bevy_spritesheet_animation/issues/66
//        - https://github.com/bevyengine/bevy/pull/22484 (merged)
//            - Part of: https://github.com/bevyengine/bevy/milestone/40 (0.19)

mod layers;
mod outline;
mod tiles;
mod transitions;

pub(crate) mod prelude {
    pub(crate) use super::layers::{DisplayLayers, LayerData, LayerDataCache, LayerHandle};
    pub(crate) use super::tiles::{TileData, TileDataCache, TileHandle};
    pub(crate) use super::transitions::{FadeInOut, apply_fade_in_out, tick_fade_in_out};
    pub(crate) use super::{CelSize, ImageMeta, image_from_data};
}

use std::marker::PhantomData;

use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::*};
use bevy_ecs_tilemap::prelude::*;

use crate::{
    animations::prelude::*, characters::prelude::*, images::prelude::*, log::prelude::*,
    render::prelude::*, screens::prelude::*,
};

pub(super) struct ImagesPlugin;
impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);

        app.add_systems(
            OnEnter(Screen::Gameplay),
            (
                (
                    insert_images_and_related::<Player>,
                    insert_images_and_related::<Slime>,
                ),
                (
                    outline::add_outline::<Player>,
                    outline::add_outline::<Slime>,
                ),
                (insert_cel_size::<Player>, insert_cel_size::<Slime>),
            )
                .in_set(EnterGameplaySystems::Images)
                .chain(),
        );
    }
}

/// Cel size.
#[derive(Resource, Default)]
pub(crate) struct CelSize<T>
where
    T: Visible,
{
    pub(crate) size: UVec2,
    pub(crate) _phantom: PhantomData<T>,
}
impl<T> CelSize<T>
where
    T: Visible,
{
    fn from_animation_data(animation_data: &AnimationDataCache<T>, meta: &ImageMeta<T>) -> Self {
        // NOTE: This asserts that `atlas_columns` and `atlas_rows` are correct for `meta`.
        assert_eq!(meta.size.width % animation_data.atlas_columns as u32, 0);
        assert_eq!(meta.size.height % animation_data.atlas_rows as u32, 0);
        let size = UVec2::new(
            meta.size.width / animation_data.atlas_columns as u32,
            meta.size.height / animation_data.atlas_rows as u32,
        );

        Self { size, ..default() }
    }
}

/// Image metadata.
#[derive(Resource)]
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

        // NOTE: This is to ensure that we do have an alpha channel. We could extend this in the future.
        assert_eq!(format, TextureFormat::bevy_default());

        // NOTE: This asserts that each image has the same metadata.
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

/// [`Handle<Image>`] from [`Image::data`] and [`ImageMeta`].
pub(crate) fn image_from_data<T>(
    data: Vec<u8>,
    meta: &ImageMeta<T>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<Image>
where
    T: Visible,
{
    let image = Image::new(
        meta.size,
        meta.dimension,
        data,
        meta.format,
        RenderAssetUsages::all(),
    );
    images.add(image)
}

/// Insert [`DisplayLayers`] and [`ImageMeta`].
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
    commands.insert_resource(meta);
}

/// Insert [`CelSize`].
fn insert_cel_size<T>(
    mut commands: Commands,
    animation_data: Res<AnimationDataCache<T>>,
    meta: Res<ImageMeta<T>>,
) where
    T: Visible,
{
    commands.insert_resource(CelSize::from_animation_data(&animation_data, &meta));
}
