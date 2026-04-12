/*
 * File: ui.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

pub(super) mod nav;
pub(super) mod scroll;

use bevy::{platform::collections::HashSet, prelude::*, window::PrimaryWindow};

use crate::{input::prelude::*, screens::prelude::*, ui::prelude::*};

pub(super) struct UiInputPlugin;
impl Plugin for UiInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((nav::UiInputNavPlugin, scroll::UiInputScrollPlugin));

        app.init_resource::<UiNavActionSet>();

        app.add_systems(
            PreUpdate,
            process_inputs.run_if(any_with_component::<UiNav>),
        );
        app.add_systems(
            PreUpdate,
            update_pointer_blocked
                .after(EnterGameplaySystems::Resources)
                .before(InputSystems::Mock)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

/// A set of optional touch ids indicating that input is currently blocked by ui.
///
/// - [`Some`] means that the id for an input has been stored. This is true for touch interactions.
/// - [`None`] means that no id was stored, but an input has happened. This is true for mouse interactions.
#[derive(Resource, Default)]
pub(crate) struct PointerBlockedByUi(pub(crate) HashSet<Option<u64>>);

/// Marker [`Component`] for directional navigation.
#[derive(Component)]
pub(crate) struct UiNav;

/// Action for UI navigation.
#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum UiNavAction {
    Up,
    Down,
    Left,
    Right,
    /// Select action.
    ///
    /// - `true` indicates a press.
    /// - `false` indicates a release.
    Select(bool),
}
impl UiNavAction {
    pub(crate) fn variants() -> Vec<Self> {
        vec![
            UiNavAction::Up,
            UiNavAction::Down,
            UiNavAction::Left,
            UiNavAction::Right,
            UiNavAction::Select(true),
            UiNavAction::Select(false),
        ]
    }
    pub(crate) fn keycode(&self) -> KeyCode {
        match self {
            UiNavAction::Up => KeyCode::ArrowUp,
            UiNavAction::Down => KeyCode::ArrowDown,
            UiNavAction::Left => KeyCode::ArrowLeft,
            UiNavAction::Right => KeyCode::ArrowRight,
            UiNavAction::Select(_) => KeyCode::Enter,
        }
    }
    pub(crate) fn gamepad_button(&self) -> GamepadButton {
        match self {
            UiNavAction::Up => GamepadButton::DPadUp,
            UiNavAction::Down => GamepadButton::DPadDown,
            UiNavAction::Left => GamepadButton::DPadLeft,
            UiNavAction::Right => GamepadButton::DPadRight,
            UiNavAction::Select(_) => GamepadButton::South,
        }
    }
    pub(crate) fn try_from_vec2(vec2: Vec2) -> Option<UiNavAction> {
        if vec2.x.abs() > vec2.y.abs() {
            Some(if vec2.x > 0. { Self::Right } else { Self::Left })
        } else if vec2.y != 0. {
            Some(if vec2.y > 0. { Self::Up } else { Self::Down })
        } else {
            None
        }
    }
}

/// [`HashSet`] containing currently relevant [`UiNavAction`]s.
#[derive(Default, Resource)]
pub(crate) struct UiNavActionSet(pub(crate) HashSet<UiNavAction>);
impl UiNavActionSet {
    pub(crate) fn direction(&self) -> Option<Dir2> {
        let net_east_west =
            self.0.contains(&UiNavAction::Right) as i8 - self.0.contains(&UiNavAction::Left) as i8;
        let net_north_south =
            self.0.contains(&UiNavAction::Up) as i8 - self.0.contains(&UiNavAction::Down) as i8;
        Dir2::from_xy(net_east_west as f32, net_north_south as f32).ok()
    }
}

/// Update [`PointerBlockedByUi`] from [`NodeRect`]s.
fn update_pointer_blocked(
    window: Single<&Window, With<PrimaryWindow>>,
    rect_query: Query<&NodeRect>,
    mut pointer_blocked: ResMut<PointerBlockedByUi>,
    drag: Res<MouseDrag>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    pointer_blocked.0.clear();
    for rect in rect_query {
        let touch_id = rect.touched_id(&touches);
        if touch_id.is_some()
            || rect.clicked(
                &mouse,
                &MouseButton::Left,
                window.cursor_position(),
                drag.start_pos,
            )
        {
            pointer_blocked.0.insert(touch_id);
        }
    }
}

// FIXME: I'm pretty sure that right stick joystick input is broken. For now I can't further describe how
//        since I only have gamepads that are very broken, but I will test this further.
//        Also for future testing use https://gamepadtest.com
/// Process inputs and insert [`UiNavAction`] into [`UiNavActionSet`].
fn process_inputs(
    gamepad_query: Query<&Gamepad>,
    mut action_set: ResMut<UiNavActionSet>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    action_set.0.clear();

    if let Some(action) = gamepad_query
        .iter()
        .find_map(|g| UiNavAction::try_from_vec2(g.right_stick()))
    {
        action_set.0.insert(action);
        return;
    };

    for action in UiNavAction::variants() {
        let on_just_pressed = action != UiNavAction::Select(false);
        let just_pressed = keyboard.just_pressed(action.keycode())
            || gamepad_query
                .iter()
                .any(|g| g.just_pressed(action.gamepad_button()));
        let just_released = keyboard.just_released(action.keycode())
            || gamepad_query
                .iter()
                .any(|g| g.just_released(action.gamepad_button()));

        if (on_just_pressed && just_pressed) || (!on_just_pressed && just_released) {
            action_set.0.insert(action);
        }
    }
}
