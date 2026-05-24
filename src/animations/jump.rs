use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{animations::prelude::*, characters::prelude::*, log::prelude::*, render::Visible};

/// Jump duration in milliseconds for `T`.
#[derive(Resource, Debug, Default)]
pub(crate) struct JumpDuration<T>
where
    T: Visible,
{
    pub(crate) millis: u64,
    _phantom: PhantomData<T>,
}
impl<T> JumpDuration<T>
where
    T: Visible,
{
    pub(crate) fn from_millis(millis: u64) -> Self {
        Self {
            millis,
            ..default()
        }
    }
}

/// Switch [`AnimationState`] out of [`AnimationAction::Jump`] after [`JumpTimer`] has finished.
pub(super) fn switch_animation<T>(
    container_query: Query<(&mut AnimationState, &JumpTimer), With<T>>,
) where
    T: Visible,
{
    for (mut state, timer) in container_query {
        if timer.0.just_finished() && state.0.0 == AnimationAction::Jump {
            state.0.0 = AnimationAction::Idle;
        }
    }
}

/// Insert [`JumpTimer`] on [`AnimationAction::Jump`].
pub(super) fn insert_timer<T>(
    container_query: Query<(Entity, &AnimationState), (With<T>, Without<JumpTimer>)>,
    mut commands: Commands,
    jump_duration: Res<JumpDuration<T>>,
) where
    T: Visible,
{
    for (entity, state) in container_query {
        if state.0.0 == AnimationAction::Jump {
            // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
            commands
                .entity(entity)
                .try_insert(JumpTimer::from_millis(jump_duration.millis));
        }
    }
}

/// Jump height
const JUMP_HEIGHT: f32 = 12.;

/// Move sprite from [`EaseFunction::QuadraticOut`].
pub(super) fn move_sprite<T>(
    container_query: Query<(&AnimationState, &mut JumpHeight, &JumpTimer, &Children), With<T>>,
    mut base_query: Query<&mut Transform, With<AnimationBase>>,
) where
    T: Visible,
{
    for (state, mut jump_height, timer, children) in container_query {
        if state.0.0 != AnimationAction::Jump {
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
}
