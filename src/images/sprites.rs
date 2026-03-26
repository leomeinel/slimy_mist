/*
 * File: sprites.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{characters::prelude::*, log::prelude::*};

/// Flip [`Sprite`]s
///
/// ## Traits
///
/// - `T` must implement [`Character`].
pub(super) fn flip_sprites<T>(
    character_query: Query<(&FacingDirection, &Children), With<T>>,
    mut sprite_query: Query<&mut Sprite, With<SpritesheetAnimation>>,
) where
    T: Character,
{
    for (direction, children) in character_query {
        if direction.0.x == 0. {
            continue;
        }

        let child = children
            .iter()
            .find(|e| sprite_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let mut sprite = sprite_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
        sprite.flip_x = direction.0.x < 0.;
    }
}
