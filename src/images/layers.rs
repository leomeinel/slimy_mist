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

use crate::{log::prelude::*, render::prelude::*};

/// Layer data deserialized from a ron file as a generic
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(serde::Deserialize, Asset, TypePath, Default)]
pub(crate) struct LayerData<T>
where
    T: Visible,
{
    #[serde(default)]
    pub(crate) layers: Vec<String>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Handle for [`LayerData`] as a generic
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
    pub(crate) images: Vec<Handle<Image>>,
    pub(crate) _phantom: PhantomData<T>,
}
impl<T> LayerDataCache<T>
where
    T: Visible,
{
    // FIXME: Use two images. One for floating and one for the rest.
    pub(crate) fn to_display_image(&self, images: &mut ResMut<Assets<Image>>) -> DisplayImage<T>
    where
        T: Visible,
    {
        // Collect metadata for first image
        // NOTE: We are just assuming that all `Images` have the exact same metadata here. I deem this to be appropriate here.
        let (size, dimension, format) = self
            .images
            .first()
            .map(|image| {
                let descriptor = &images
                    .get(image)
                    .expect(ERR_INVALID_IMAGE)
                    .texture_descriptor;
                (descriptor.size, descriptor.dimension, descriptor.format)
            })
            .expect(ERR_INVALID_IMAGE);

        // Combine `Images` into a single `Image` by overriding non-transparent pixels in each previous iteration of `data`.
        let data: Vec<_> = self
            .images
            .iter()
            .map(|image| {
                let image = images.get(image).expect(ERR_INVALID_IMAGE);
                image.data.clone().expect(ERR_INVALID_IMAGE)
            })
            .collect();
        // FIXME: This probably does not work for transparent pixels.
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
        let image = Image::new(size, dimension, data, format, RenderAssetUsages::all());

        DisplayImage::new(images.add(image))
    }
}

/// Insert [`DisplayImage`].
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
pub(super) fn insert_display_image<T>(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    data: Res<LayerDataCache<T>>,
) where
    T: Visible,
{
    commands.insert_resource(data.to_display_image(&mut images));
}

/// [`Image`] for displaying `T`
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
#[derive(Resource, Default)]
pub(crate) struct DisplayImage<T>
where
    T: Visible,
{
    pub(crate) image: Handle<Image>,
    pub(crate) _phantom: PhantomData<T>,
}
impl<T> DisplayImage<T>
where
    T: Visible,
{
    fn new(image: Handle<Image>) -> Self {
        Self { image, ..default() }
    }
}
