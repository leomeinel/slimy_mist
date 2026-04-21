/*
 * File: particles.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::{math::FloatPow, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{animations::prelude::*, characters::prelude::*, log::prelude::*, render::prelude::*};

/// Minimum fraction of [`WalkSpeed`] for which to activate [`Particle`]s.
const MIN_WALK_SPEED_FRAC: f32 = 0.75;

/// Toggle [`ParticleWalkingDust`] for `T`.
///
/// This triggers [`ToggleParticle`] on certain conditions.
pub(super) fn toggle_walking_dust<T>(
    base_query: Query<&Children, With<AnimationBase>>,
    character_query: Query<
        (
            &mut AnimationState,
            &KinematicCharacterControllerOutput,
            &WalkSpeed,
            &Children,
        ),
        With<T>,
    >,
    particle_query: Query<(Entity, &ParticleWalkingDust, &ParticleTimer)>,
    mut commands: Commands,
    time: Res<Time>,
) where
    T: Visible,
{
    for (animation_state, controller_output, walk_speed, children) in character_query {
        let child = children
            .iter()
            .find(|e| base_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let children = base_query.get(child).expect(ERR_INVALID_CHILDREN);

        let child = children
            .iter()
            .find(|e| particle_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let (entity, particle, timer) = particle_query.get(child).expect(ERR_INVALID_CHILDREN);

        if timer.0.just_finished() {
            let activate = particle.is_active(animation_state.0.0)
                && controller_output.desired_translation.length_squared()
                    > ((walk_speed.0 * MIN_WALK_SPEED_FRAC) * time.delta_secs()).squared();

            commands.trigger(ToggleParticle::<ParticleWalkingDust>::new(entity, activate));
        }
    }
}
