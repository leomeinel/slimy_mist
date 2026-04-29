use bevy::prelude::*;

/// [`Color`] from RGB values as [`u8`] array.
///
/// Valid color channel values are from 0-255.
pub(crate) const fn color_from_rgb(rgb: &[u8; 3]) -> Color {
    Color::srgb_u8(rgb[0], rgb[1], rgb[2])
}

/// [`Color`] from RGB values as [`u8`] array with a custom alpha.
///
/// Valid color channel values are from 0-255.
#[allow(dead_code)]
pub(crate) const fn color_from_rgba(rgb: &[u8; 3], a: u8) -> Color {
    Color::srgba_u8(rgb[0], rgb[1], rgb[2], a)
}
