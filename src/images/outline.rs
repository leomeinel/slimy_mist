use bevy::prelude::*;

use crate::{images::prelude::*, log::prelude::*, render::prelude::*, utils::prelude::*};

/// Add an outline to [`DisplayLayers`].
pub(super) fn add_outline<T>(
    mut layers: ResMut<DisplayLayers<T>>,
    mut images: ResMut<Assets<Image>>,
    image_meta: Res<ImageMeta<T>>,
) where
    T: Visible,
{
    let base = images.get(layers.base.id()).expect(ERR_INVALID_IMAGE);
    let floating = layers
        .floating
        .as_ref()
        .map(|i| images.get(i.id()).expect(ERR_INVALID_IMAGE));

    let base_data = outlined_image_data(
        base.data.clone().expect(ERR_INVALID_IMAGE),
        image_meta.size.width,
        image_meta.size.height,
    );
    let floating_data = floating.map(|i| {
        outlined_image_data(
            i.data.clone().expect(ERR_INVALID_IMAGE),
            image_meta.size.width,
            image_meta.size.height,
        )
    });

    layers.base = image_from_data(base_data, &image_meta, &mut images);
    if let Some(floating_data) = floating_data {
        layers.floating = Some(image_from_data(floating_data, &image_meta, &mut images));
    }
}

/// Image data with an added outline in the same format as [`Image::data`].
fn outlined_image_data(data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut outlined_data = data.clone();
    for y in 0..height {
        for x in 0..width {
            let i = pixel_index(x, y, width);
            if !is_transparent_pixel(&data, i) || !has_opaque_neighbor(&data, x, y, width, height) {
                continue;
            }
            outlined_data[i..i + 4].copy_from_slice(&OUTLINE_COLOR.to_srgba().to_u8_array());
        }
    }

    outlined_data
}
