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

// FIXME: This should be influenced by aiming direction and should determine sprite flip.
//        Movement direction should be secondary.
/// Direction the [`Character`](crate::characters::Character) is facing.
#[derive(Component)]
pub(crate) struct FacingDirection(pub(crate) Vec2);
impl Default for FacingDirection {
    fn default() -> Self {
        Self(Vec2::X)
    }
}

/// [`Character`](crate::characters::Character) jump height.
#[derive(Component, Default)]
pub(crate) struct JumpHeight(pub(crate) f32);

/// [`Character`](crate::characters::Character) walking speed.
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
        Self(Timer::from_seconds(
            JUMP_DURATION_SECS / 2.,
            TimerMode::Once,
        ))
    }
}

/// Update [`FacingDirection`] from [`KinematicCharacterControllerOutput::desired_translation`].
pub(super) fn update_facing(
    query: Query<
        (&mut FacingDirection, &KinematicCharacterControllerOutput),
        Changed<KinematicCharacterControllerOutput>,
    >,
) {
    for (mut facing, controller_output) in query {
        // NOTE: This only checks for desired movement, not actual movement. This is to ensure that
        //       even if a character can't move, it can still change its' facing direction.
        if controller_output.desired_translation != Vec2::ZERO {
            facing.0 = controller_output.desired_translation.normalize_or_zero();
        }
    }
}
