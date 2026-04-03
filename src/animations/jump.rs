/*
 * File: jump.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;

use crate::{animations::prelude::*, characters::prelude::*, log::prelude::*};

/// Jump height
const JUMP_HEIGHT: f32 = 12.;

/// Move sprite from [`EaseFunction::QuadraticOut`].
pub(super) fn move_sprite(
    player: Single<(&AnimationState, &mut JumpHeight, &JumpTimer, &Children), With<Player>>,
    mut base_query: Query<&mut Transform, With<AnimationBase>>,
) {
    let (animation_state, mut jump_height, timer, children) = player.into_inner();
    if animation_state.0.0 != AnimationAction::Jump {
        return;
    }

    // Apply jump
    let factor = EaseFunction::QuadraticOut
        .ping_pong()
        .expect(ERR_INVALID_DOMAIN_EASING);
    // NOTE: We are multiplying by 2 since `PingPongCurve` has a domain from 0 to 2.
    let factor = factor.sample_clamped(timer.0.fraction() * 2.);
    let target = JUMP_HEIGHT * factor;

    let child = children
        .iter()
        .find(|e| base_query.contains(*e))
        .expect(ERR_INVALID_CHILDREN);
    let mut transform = base_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
    transform.translation.y += target - jump_height.0;
    jump_height.0 = target;
}

/// Switch [`AnimationState`] out of [`AnimationAction::Jump`] after [`JumpTimer`] has finished.
pub(super) fn switch_animation(player: Single<(&mut AnimationState, &JumpTimer), With<Player>>) {
    let (mut animation_state, timer) = player.into_inner();
    if !timer.0.just_finished() {
        return;
    }
    if animation_state.0.0 == AnimationAction::Jump {
        animation_state.0.0 = AnimationAction::Idle;
    }
}
