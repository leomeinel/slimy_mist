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

use crate::{log::prelude::*, render::prelude::*};

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
impl<T> LayerDataCache<T>
where
    T: Visible,
{
    pub(crate) fn to_display_layers(&self, images: &mut ResMut<Assets<Image>>) -> DisplayLayers<T>
    where
        T: Visible,
    {
        let base = layered_image(self.base.clone(), images);
        let floating = self
            .floating
            .as_ref()
            .map(|f| layered_image(f.clone(), images));

        DisplayLayers {
            base,
            floating,
            ..default()
        }
    }
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

/// Insert [`DisplayLayers`].
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
pub(super) fn insert_display_layers<T>(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    data: Res<LayerDataCache<T>>,
) where
    T: Visible,
{
    commands.insert_resource(data.to_display_layers(&mut images));
}

fn layered_image(layers: Vec<Handle<Image>>, images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    let (size, dimension, format) = layers
        .first()
        .map(|image| {
            let descriptor = &images
                .get(image)
                .expect(ERR_INVALID_IMAGE)
                .texture_descriptor;
            (descriptor.size, descriptor.dimension, descriptor.format)
        })
        .expect(ERR_INVALID_IMAGE);
    assert!(layers.iter().all(|image| {
        let descriptor = &images
            .get(image)
            .expect(ERR_INVALID_IMAGE)
            .texture_descriptor;
        (descriptor.size, descriptor.dimension, descriptor.format) == (size, dimension, format)
    }));

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

    let image = Image::new(size, dimension, data, format, RenderAssetUsages::all());
    images.add(image)
}
