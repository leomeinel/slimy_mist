/*
 * File: layers.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/NiklasEi/bevy_asset_loader/blob/main/bevy_asset_loader/examples/custom_dynamic_assets
 */

use std::marker::PhantomData;

use bevy::{asset::RenderAssetUsages, prelude::*};
use serde::Deserialize;

use crate::{images::prelude::*, log::prelude::*, render::prelude::*};

/// Layer data deserialized from a ron file.
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(Deserialize, Asset, TypePath, Default)]
pub(crate) struct LayerData<T>
where
    T: Visible,
{
    #[serde(default)]
    pub(crate) base: Vec<String>,
    #[serde(default)]
    pub(crate) floating: Option<Vec<String>>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Handle for [`LayerData`].
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(Resource)]
pub(crate) struct LayerHandle<T>(pub(crate) Handle<LayerData<T>>)
where
    T: Visible;

/// Cache for [`LayerData`]
///
/// This is to allow easier access.
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(Resource, Default)]
pub(crate) struct LayerDataCache<T>
where
    T: Visible,
{
    pub(crate) base: Vec<Handle<Image>>,
    pub(crate) floating: Option<Vec<Handle<Image>>>,
    pub(crate) _phantom: PhantomData<T>,
}

/// [`Image`] for displaying `T`
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(Resource, Default)]
pub(crate) struct DisplayLayers<T>
where
    T: Visible,
{
    pub(crate) base: Handle<Image>,
    pub(crate) floating: Option<Handle<Image>>,
    pub(crate) _phantom: PhantomData<T>,
}
impl<T> DisplayLayers<T>
where
    T: Visible,
{
    pub(crate) fn from_layer_data_cache(
        data: &LayerDataCache<T>,
        meta: &ImageMeta<T>,
        images: &mut ResMut<Assets<Image>>,
    ) -> DisplayLayers<T>
    where
        T: Visible,
    {
        let base = layered_image(data.base.clone(), meta, images);
        let floating = data
            .floating
            .as_ref()
            .map(|f| layered_image(f.clone(), meta, images));

        Self {
            base,
            floating,
            ..default()
        }
    }
}

/// A single [`Handle<Image>`] from layers.
fn layered_image<T>(
    layers: Vec<Handle<Image>>,
    meta: &ImageMeta<T>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<Image>
where
    T: Visible,
{
    // Combine `Images` into a single `Image` by overriding non-transparent pixels in each previous iteration of `data`.
    let data: Vec<_> = layers
        .iter()
        .map(|image| {
            let image = images.get(image).expect(ERR_INVALID_IMAGE);
            image.data.clone().expect(ERR_INVALID_IMAGE)
        })
        .collect();
    // FIXME: This does not work correctly for transparent pixels.
    // NOTE: We are iterating in reverse order to make the first layer the top layer.
    let data = data
        .into_iter()
        .rev()
        .reduce(|mut current, next| {
            for (c, n) in current.iter_mut().zip(next).filter(|(_, n)| *n != 0) {
                *c = n;
            }
            current
        })
        .expect(ERR_INVALID_IMAGE);

    let image = Image::new(
        meta.size,
        meta.dimension,
        data,
        meta.format,
        RenderAssetUsages::all(),
    );
    images.add(image)
}
