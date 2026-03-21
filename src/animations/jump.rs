/*
 * File: jump.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{animations::prelude::*, characters::prelude::*, log::prelude::*};

/// Jump height
const JUMP_HEIGHT: f32 = 12.;

/// Apply jump
pub(super) fn apply_jump(
    player: Single<(&AnimationCache, &mut JumpHeight, &JumpTimer, &Children), With<Player>>,
    mut transform_query: Query<&mut Transform, With<SpritesheetAnimation>>,
) {
    let (cache, mut jump_height, timer, children) = player.into_inner();

    // Return if we are not jumping or falling
    let state = cache.state;
    if !matches!(state, AnimationState::Jump | AnimationState::Fall) {
        return;
    }

    // Apply visual jump or fall
    let factor = if state == AnimationState::Jump {
        1.0f32
    } else {
        -1.0f32
    };
    let eased_time = EaseFunction::QuadraticOut.sample_clamped(timer.0.fraction());
    let target = JUMP_HEIGHT * factor * eased_time;

    let child = children
        .iter()
        .find(|e| transform_query.contains(*e))
        .expect(ERR_INVALID_CHILDREN);
    let mut transform = transform_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
    transform.translation.y += target - jump_height.0;
    jump_height.0 = target;
}

/// Limit jump by setting fall after specific time and then switching to idle
pub(super) fn limit_jump(
    player: Single<(Entity, &mut AnimationCache, &mut JumpHeight, &JumpTimer), With<Player>>,
    mut commands: Commands,
) {
    let (entity, mut cache, mut jump_height, timer) = player.into_inner();

    // Return if timer has not finished
    if !timer.0.just_finished() {
        return;
    }

    // Reset jump height
    jump_height.0 = 0.;

    // Set animation states
    match cache.state {
        AnimationState::Jump => {
            commands.entity(entity).insert(JumpTimer::default());
            cache.state = AnimationState::Fall;
        }
        AnimationState::Fall => cache.state = AnimationState::Idle,
        _ => (),
    }
}
