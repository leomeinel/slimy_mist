use bevy::{prelude::*, window::PrimaryWindow};
use bevy_enhanced_input::prelude::*;
use virtual_joystick::VirtualJoystickMessage;

use crate::{characters::prelude::*, input::prelude::*, render::prelude::*, ui::prelude::*};

/// Mock [`Walk`] from virtual [`VirtualJoystickMessage`].
pub(super) fn mock_walk_from_virtual_joystick(
    mut reader: MessageReader<VirtualJoystickMessage<u8>>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    for joystick in reader.read() {
        if joystick.id() != JoystickID::MOVEMENT {
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
    pointer_blocked: Res<PointerBlockedByUi>,
    touches: Res<Touches>,
) {
    for touch in touches.iter_just_released() {
        if pointer_blocked.0.contains(&Some(touch.id())) {
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
    pointer_blocked: Res<PointerBlockedByUi>,
    pointer_start: Res<PointerStartTimeSecs>,
    touches: Res<Touches>,
    time: Res<Time>,
) {
    if !pointer_start.is_tap(time.elapsed_secs()) {
        return;
    }

    if touches
        .iter_just_released()
        .any(|t| !pointer_blocked.0.contains(&Some(t.id())) && !t.is_vertical_swipe())
    {
        commands
            .entity(*melee)
            .mock_once::<Player, Melee>(TriggerState::Fired, true);
    }
}

/// Mock [`Aim`] from [`Touches`].
pub(super) fn mock_aim_from_touch(
    aim: Single<Entity, With<Player>>,
    camera: Single<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    player_transform: Single<&GlobalTransform, With<Player>>,
    mut commands: Commands,
    pointer_blocked: Res<PointerBlockedByUi>,
    touches: Res<Touches>,
) {
    let (camera, camera_transform) = *camera;

    // NOTE: We are using `just_pressed` to allow use in `Melee`.
    for touch in touches.iter_just_pressed() {
        if pointer_blocked.0.contains(&Some(touch.id())) {
            continue;
        }
        if let Ok(world_pointer_pos) =
            camera.viewport_to_world_2d(camera_transform, touch.position())
        {
            let direction = world_pointer_pos - player_transform.translation().xy();
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
    pointer_blocked: Res<PointerBlockedByUi>,
    pointer_start: Res<PointerStartTimeSecs>,
    time: Res<Time>,
) {
    if pointer_blocked.0.contains(&None)
        || !mouse.just_released(MouseButton::Left)
        || !pointer_start.is_tap(time.elapsed_secs())
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
    player_transform: Single<&GlobalTransform, With<Player>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    pointer_blocked: Res<PointerBlockedByUi>,
) {
    // NOTE: We are using `just_pressed` to allow use in `Melee`.
    if pointer_blocked.0.contains(&None) || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = *camera;

    if let Some(pointer_pos) = window.cursor_position()
        && let Ok(world_pointer_pos) = camera.viewport_to_world_2d(camera_transform, pointer_pos)
    {
        let direction = world_pointer_pos - player_transform.translation().xy();
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
