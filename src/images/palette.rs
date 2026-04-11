/*
 * File: palette.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

// FIXME: If we are extending this, it should have all colors from the palette and have them structured
//        according to the palette.

use bevy::prelude::*;

/// [`Color`] from palette used by images at index 1.
///
/// This is used for coloring outlines.
pub(crate) const OUTLINE_COLOR: Color = Color::srgba_u8(28, 35, 36, 255);
