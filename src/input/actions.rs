/*
 * File: actions.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animations::prelude::*, characters::prelude::*, core::prelude::*, log::prelude::*};

/// Walk [`InputAction`]
#[derive(InputAction)]
#[action_output(Vec2)]
pub(crate) struct Walk;

/// Jump [`InputAction`]
#[derive(InputAction)]
#[action_output(bool)]
pub(crate) struct Jump;

/// Melee attack [`InputAction`]
#[derive(InputAction)]
#[action_output(bool)]
pub(crate) struct Melee;

/// Aim direction [`InputAction`]
#[derive(InputAction)]
#[action_output(Vec2)]
pub(crate) struct Aim;

/// Input [`Action`]s for [`Player`].
pub(crate) fn player_input() -> impl Bundle {
    actions!(
        Player[
            // Movement
            (
                Action::<Walk>::new(),
                ActionSettings {
                    require_reset: true,
                    ..default()
                },
                DeadZone::default(),
                SmoothNudge::default(),
                Bindings::spawn((
                    Cardinal::arrows(),
                    Cardinal::wasd_keys(),
                    Axial::left_stick(),
                ))
            ),
            (
                Action::<Jump>::new(),
                bindings![KeyCode::Space, GamepadButton::South],
            ),
            // Attack
            (
                Action::<Melee>::new(),
                bindings![GamepadButton::RightTrigger],
            ),
            (
                Action::<Aim>::new(),
                ActionSettings {
                    require_reset: true,
                    ..default()
                },
                DeadZone::default(),
                Bindings::spawn(Axial::right_stick())
            ),
        ]
    )
}

/// On a fired [`Walk`], set translation to the given input.
pub(super) fn apply_walk(
    event: On<Fire<Walk>>,
    player: Single<
        (
            &mut AnimationCache,
            &mut KinematicCharacterController,
            &WalkSpeed,
        ),
        With<Player>,
    >,
    pause: Res<State<Pause>>,
    time: Res<Time>,
) {
    if pause.get().0 {
        return;
    }

    let (mut cache, mut controller, walk_speed) = player.into_inner();
    let direction = event.value * walk_speed.0 * time.delta_secs();
    controller.translation = Some(direction);

    if cache.state == AnimationState::Idle {
        cache.set_new_state(AnimationState::Walk);
    }
}

/// On a completed [`Walk`], set translation to zero.
pub(super) fn reset_walk(
    _: On<Complete<Walk>>,
    player: Single<(&mut AnimationCache, &mut KinematicCharacterController), With<Player>>,
) {
    let (mut cache, mut controller) = player.into_inner();

    if cache.state != AnimationState::Jump {
        let direction = Vec2::ZERO;
        controller.translation = Some(direction);
        cache.set_new_state(AnimationState::Idle);
    }
}

/// On a fired [`Jump`], add [`JumpTimer`].
pub(super) fn set_jump(
    _: On<Fire<Jump>>,
    player: Single<(Entity, &mut AnimationCache), With<Player>>,
    mut commands: Commands,
    pause: Res<State<Pause>>,
) {
    if pause.get().0 {
        return;
    }

    let (entity, mut cache) = player.into_inner();

    if cache.state != AnimationState::Jump {
        commands.entity(entity).insert(JumpTimer::default());
        cache.set_new_state(AnimationState::Jump);
    }
}

/// On a fired [`Melee`], write [`InitAttack`].
pub(super) fn init_melee_attack(
    _: On<Fire<Melee>>,
    mut writer: MessageWriter<InitAttack>,
    player: Single<(Entity, &AttackStats, Option<&AttackTimer>), With<Player>>,
    mut commands: Commands,
    pause: Res<State<Pause>>,
) {
    if pause.get().0 {
        return;
    }
    let (entity, stats, timer) = *player;
    if let Some(timer) = timer
        && !timer.0.is_finished()
    {
        return;
    }
    let Some(melee) = &stats.melee else {
        warn_once!("{}", WARN_INVALID_ATTACK_DATA);
        return;
    };

    commands.trigger(DelayAttack {
        entity,
        cooldown_secs: *melee.cooldown_secs,
    });
    writer.write(InitAttack::Melee(entity));
}
