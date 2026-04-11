/*
 * File: outline.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;

use crate::{images::prelude::*, log::prelude::*, render::prelude::*};

/// Add an outline to [`DisplayLayers`].
pub(super) fn add_outline<T>(
    mut layers: ResMut<DisplayLayers<T>>,
    mut images: ResMut<Assets<Image>>,
    image_meta: Res<ImageMeta<T>>,
    image_size: Res<ImageSize<T>>,
) where
    T: Visible,
{
    let base = images.get(layers.base.id()).expect(ERR_INVALID_IMAGE);
    let floating = layers
        .floating
        .as_ref()
        .map(|i| images.get(i.id()).expect(ERR_INVALID_IMAGE));

    let base_data =
        outlined_image_data(base.data.clone().expect(ERR_INVALID_IMAGE), image_size.size);
    let floating_data = floating
        .map(|i| outlined_image_data(i.data.clone().expect(ERR_INVALID_IMAGE), image_size.size));

    layers.base = image_from_data(base_data, &*image_meta, &mut images);
    if let Some(floating_data) = floating_data {
        layers.floating = Some(image_from_data(floating_data, &*image_meta, &mut images));
    }
}

/// Image data with an added outline in the same format as [`Image::data`].
fn outlined_image_data(data: Vec<u8>, size: UVec2) -> Vec<u8> {
    let (width, height) = (size.x as usize, size.y as usize);
    let mut outlined_data = data.clone();
    for y in 0..height {
        for x in 0..width {
            let i = pixel_index(x, y, width);
            if !is_transparent(&data, i) || !has_opaque_neighbor(&data, x, y, width, height) {
                continue;
            }
            outlined_data[i..i + 4].copy_from_slice(&OUTLINE_COLOR.to_srgba().to_u8_array());
        }
    }

    outlined_data
}

/// Offsets of pixel neighbors.
const NEIGHBOR_OFFSETS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

/// Whether pixel coordinates x, y have any opaqua neighbor.
///
/// This is computed with the help of [`NEIGHBOR_OFFSETS`].
fn has_opaque_neighbor(data: &[u8], x: usize, y: usize, width: usize, height: usize) -> bool {
    NEIGHBOR_OFFSETS.iter().any(|(ox, oy)| {
        let (nx, ny) = (x as isize + ox, y as isize + oy);
        nx >= 0
            && ny >= 0
            && nx < width as isize
            && ny < height as isize
            && !is_transparent(data, pixel_index(nx as usize, ny as usize, width))
    })
}

/// Index of pixel coordinates x, y in [`Image::data`].
fn pixel_index(x: usize, y: usize, width: usize) -> usize {
    (y * width + x) * 4
}

/// Whether the pixel index `i` in data is transparent.
fn is_transparent(data: &[u8], i: usize) -> bool {
    data[i + 3] == 0
}
