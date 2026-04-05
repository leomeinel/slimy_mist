/*
 * File: z_levels.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

/// Z-level for the level
pub(crate) const LEVEL_Z: f32 = 1.;
/// Z-level for any foreground object
///
/// The value is chosen so that there is a very reasonable distance to [`OrthographicProjection::far`](bevy::camera::OrthographicProjection::far)
/// while considering relative y-sorting.
pub(crate) const FOREGROUND_Z: f32 = 5.;
/// Z-level for any overlay object
pub(crate) const OVERLAY_Z: f32 = 10.;
/// Z-level for light
pub(crate) const LIGHT_Z: f32 = 10.;

/// Z-level delta for background objects
///
/// This is set to a delta compatible with relative y-sorting that should never subtract more than 1
/// from [`YSort`](crate::render::prelude::YSort)'s field.
pub(crate) const BACKGROUND_Z_DELTA: f32 = -1.;
/// Z-level delta for image layers.
///
/// This is set to a somewhat arbitrary meant to be rendering safe minimal delta to only impact local layer rendering.
pub(crate) const LAYER_Z_DELTA: f32 = 1e-5;
