/*
 * File: movement.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::characters::prelude::*;

/// Direction the [`Character`] is facing.
#[derive(Component)]
pub(crate) struct FacingDirection(pub(crate) Vec2);
impl Default for FacingDirection {
    fn default() -> Self {
        Self(Vec2::NEG_Y)
    }
}

/// [`Character`] jump height.
#[derive(Component, Default)]
pub(crate) struct JumpHeight(pub(crate) f32);

/// [`Character`] walking speed.
#[derive(Component)]
pub(crate) struct WalkSpeed(pub(crate) f32);

/// Jumping duration in seconds
pub(crate) const JUMP_DURATION_SECS: f32 = 1.;

/// Timer that tracks jumping
#[derive(Component, Debug, Clone, PartialEq, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub(crate) struct JumpTimer(pub(crate) Timer);
impl Default for JumpTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(JUMP_DURATION_SECS, TimerMode::Once))
    }
}

/// Update [`FacingDirection`].
pub(super) fn update_facing_direction(
    query: Query<
        (
            &mut FacingDirection,
            Option<&AttackTimer>,
            &AimDirection,
            Option<&KinematicCharacterControllerOutput>,
        ),
        Or<(
            Changed<AimDirection>,
            Changed<KinematicCharacterControllerOutput>,
        )>,
    >,
) {
    for (mut facing, timer, aim_direction, controller_output) in query {
        let direction = if let Some(timer) = timer
            && !timer.0.is_finished()
            && aim_direction.0 != Vec2::ZERO
        {
            aim_direction.0
        } else if let Some(controller_output) = controller_output
            && controller_output.desired_translation != Vec2::ZERO
        {
            controller_output.desired_translation
        } else {
            return;
        }
        .normalize_or_zero();

        facing.0 = direction;
    }
}
