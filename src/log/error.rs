/*
 * File: error.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

/// Error on invalid attacker [`Entity`](bevy::prelude::Entity).
pub(crate) const ERR_INVALID_ATTACKER: &str = "Invalid attacker entity. This might be a bug.";
/// Error on invalid [`bevy::prelude::Children`].
pub(crate) const ERR_INVALID_CHILDREN: &str = "Invalid children. This might be a bug.";
/// Error on invalid domain of [`bevy::prelude::EasingCurve`].
pub(crate) const ERR_INVALID_DOMAIN_EASING: &str =
    "Invalid domain of easing curve. This might be a bug.";
/// Error on invalid [`Image`](bevy::prelude::Image).
pub(crate) const ERR_INVALID_IMAGE: &str = "Invalid image. The config might be invalid.";
/// Error on invalid [`ProcGenCache::chunk_positions`](crate::procgen::ProcGenCache::chunk_positions).
pub(crate) const ERR_INVALID_CHUNK_POSITIONS: &str =
    "Invalid chunk positions. This might be a bug.";
/// Error on invalid [`NavMesh`](vleue_navigator::NavMesh).
pub(crate) const ERR_INVALID_NAVMESH: &str = "Invalid nav mesh. This might be a bug.";
/// Error on invalid [`NavTarget`](crate::characters::prelude::NavTarget).
pub(crate) const ERR_INVALID_NAV_TARGET: &str = "Invalid nav target. This might be a bug.";
/// Error on invalid [`ReadRapierContext`](bevy_rapier2d::prelude::ReadRapierContext).
pub(crate) const ERR_INVALID_RAPIER_CONTEXT: &str = "Invalid rapier context. This might be a bug.";
/// Error on invalid or incomplete idle [`AnimationData`](crate::animations::prelude::AnimationData).
pub(crate) const ERR_INVALID_IDLE_ANIMATION_DATA: &str =
    "Invalid or incomplete idle animation data. The config might be invalid.";
/// Error on invalid [`TextureAtlas`](bevy::prelude::TextureAtlas).
pub(crate) const ERR_INVALID_TEXTURE_ATLAS: &str = "Invalid texture atlas. This might be a bug.";

/// Error on loading [`AnimationData`](crate::animations::prelude::AnimationData).
pub(crate) const ERR_LOADING_ANIMATION_DATA: &str =
    "Could not load animation data. The config might be missing.";
/// Error on loading [`CollisionData`](crate::characters::prelude::CollisionData).
pub(crate) const ERR_LOADING_COLLISION_DATA: &str =
    "Could not load collision data. The config might be missing.";
/// Error on loading [`CreditsData`](crate::ui::prelude::CreditsData).
pub(crate) const ERR_LOADING_CREDITS_DATA: &str =
    "Could not load credits data. The config might be missing.";
/// Error on loading [`LayerData`](crate::images::prelude::LayerData).
pub(crate) const ERR_LOADING_LAYER_DATA: &str =
    "Could not load layer data. The config might be missing.";
/// Error on loading [`TileData`](crate::images::prelude::TileData).
pub(crate) const ERR_LOADING_TILE_DATA: &str =
    "Could not load tile data. The config might be missing.";

/// Error on nonexistent [`Image`](bevy::prelude::Image).
pub(crate) const ERR_NONEXISTENT_IMAGE: &str = "Nonexistent image. This might be a bug.";
/// Error on nonexistent [`Animation`](bevy_spritesheet_animation::animation::Animation).
pub(crate) const ERR_NONEXISTENT_ANIMATION: &str = "Nonexistent animation. This might be a bug.";
