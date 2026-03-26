/*
 * File: mock.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_enhanced_input::prelude::*;
use virtual_joystick::VirtualJoystickMessage;

use crate::{characters::prelude::*, input::prelude::*, render::prelude::*};

/// Mock [`Walk`] from virtual [`VirtualJoystickMessage`].
pub(super) fn mock_walk_from_virtual_joystick(
    mut reader: MessageReader<VirtualJoystickMessage<u8>>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    for joystick in reader.read() {
        if joystick.id() != JoystickID::Movement as u8 {
            continue;
        }

        let input = joystick.axis();
        if input == &Vec2::ZERO {
            continue;
        }
        commands
            .entity(*player)
            .mock_once::<Player, Walk>(TriggerState::Fired, *input);
    }
}

/// Mock [`Jump`] from [`Touches`].
pub(super) fn mock_jump_from_touch(
    jump: Single<Entity, With<Player>>,
    mut commands: Commands,
    touches: Res<Touches>,
    rect: Res<JoystickRect<{ JoystickID::Movement as u8 }>>,
) {
    for touch in touches.iter_just_released() {
        if rect.intersects_with(touch.start_position()) {
            continue;
        }

        if touch.is_swipe_up() {
            commands
                .entity(*jump)
                .mock_once::<Player, Jump>(TriggerState::Fired, true);
        }
    }
}

/// Mock [`Melee`] from [`Touches`].
pub(super) fn mock_melee_from_touch(
    melee: Single<Entity, With<Player>>,
    mut commands: Commands,
    pointer_start: Res<PointerStartTimeSecs>,
    touches: Res<Touches>,
    rect: Res<JoystickRect<{ JoystickID::Movement as u8 }>>,
    time: Res<Time>,
) {
    if !pointer_start.is_tap(time.elapsed_secs())
        || touches
            .iter_just_released()
            .any(|t| rect.intersects_with(t.start_position()))
    {
        return;
    }

    if touches.iter_just_released().any(|t| !t.is_vertical_swipe()) {
        commands
            .entity(*melee)
            .mock_once::<Player, Melee>(TriggerState::Fired, true);
    }
}

/// Mock [`Aim`] from [`Touches`].
pub(super) fn mock_aim_from_touch(
    aim: Single<Entity, With<Player>>,
    camera: Single<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    player_transform: Single<&Transform, With<Player>>,
    mut commands: Commands,
    touches: Res<Touches>,
    rect: Res<JoystickRect<{ JoystickID::Movement as u8 }>>,
) {
    let (camera, camera_transform) = *camera;

    // NOTE: We are using `just_pressed` to allow use in `Melee`.
    for touch in touches.iter_just_pressed() {
        if let Ok(pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            if rect.intersects_with(pos) {
                continue;
            }

            let direction = pos - player_transform.translation.xy();
            commands.entity(*aim).mock::<Player, Aim>(
                TriggerState::Fired,
                direction.normalize_or_zero(),
                MockSpan::Manual,
            );
        }
    }
}

/// Mock [`Melee`] from clicks.
pub(super) fn mock_melee_from_click(
    melee: Single<Entity, With<Player>>,
    mut commands: Commands,
    drag: Res<MouseDrag>,
    mouse: Res<ButtonInput<MouseButton>>,
    pointer_start: Res<PointerStartTimeSecs>,
    rect: Res<JoystickRect<{ JoystickID::Movement as u8 }>>,
    time: Res<Time>,
) {
    let Some(start_pos) = drag.start_pos else {
        return;
    };
    if !mouse.just_released(MouseButton::Left)
        || !pointer_start.is_tap(time.elapsed_secs())
        || rect.intersects_with(start_pos)
        || drag.is_vertical_swipe()
    {
        return;
    }

    commands
        .entity(*melee)
        .mock_once::<Player, Melee>(TriggerState::Fired, true);
}

/// Mock [`Aim`] from clicks.
pub(super) fn mock_aim_from_click(
    aim: Single<Entity, With<Player>>,
    camera: Single<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    player_transform: Single<&Transform, With<Player>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    rect: Res<JoystickRect<{ JoystickID::Movement as u8 }>>,
) {
    // NOTE: We are using `just_pressed` to allow use in `Melee`.
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = *camera;

    if let Some(pos) = window.cursor_position()
        && let Ok(pos) = camera.viewport_to_world_2d(camera_transform, pos)
    {
        if rect.intersects_with(pos) {
            return;
        }

        let direction = pos - player_transform.translation.xy();
        commands.entity(*aim).mock::<Player, Aim>(
            TriggerState::Fired,
            direction.normalize_or_zero(),
            MockSpan::Manual,
        );
    }
}

/// On a completed [`Melee`], reset [`ActionMock`] for [`Aim`].
pub(super) fn reset_aim_mock(
    _: On<Complete<Melee>>,
    mock: Single<&mut ActionMock, With<Action<Aim>>>,
) {
    let mut mock = mock.into_inner();
    mock.enabled = false;
}
