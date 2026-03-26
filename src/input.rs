/*
 * File: input.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

mod actions;
mod joystick;
mod mock;
mod pointer;
mod ui;

pub(crate) mod prelude {
    pub(crate) use super::actions::{Aim, Jump, Melee, Walk, player_input};
    pub(crate) use super::joystick::{
        JoystickAssets, JoystickID, JoystickMap, JoystickRect, JoystickState,
    };
    pub(crate) use super::pointer::{MouseDrag, PointerStartTimeSecs, Swipe};
    pub(crate) use super::ui::scroll::{AutoScroll, InputScroll};
    pub(crate) use super::ui::{UiNav, UiNavAction, UiNavActionSet};
}

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{characters::prelude::*, screens::prelude::*};

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin);
        app.add_plugins((joystick::JoystickPlugin, ui::UiInputPlugin));

        app.add_input_context::<Player>();

        app.add_observer(actions::apply_walk);
        app.add_observer(actions::init_melee_attack);
        app.add_observer(actions::reset_walk);
        app.add_observer(actions::set_jump);
        app.add_observer(mock::reset_aim_mock);

        app.configure_sets(
            PreUpdate,
            (InputSystems::Cache, InputSystems::Mock)
                .before(EnhancedInputSystems::Update)
                .run_if(in_state(Screen::Gameplay))
                .chain(),
        );
        app.add_systems(
            PreUpdate,
            (
                mock::mock_walk_from_virtual_joystick,
                mock::mock_jump_from_touch,
                (mock::mock_melee_from_click, mock::mock_melee_from_touch).chain(),
                (mock::mock_aim_from_click, mock::mock_aim_from_touch).chain(),
            )
                .in_set(InputSystems::Mock)
                .chain(),
        );
        app.add_systems(
            PreUpdate,
            (
                pointer::update_pointer_start_time_secs,
                pointer::update_mouse_drag,
            )
                .before(EnhancedInputSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

/// A [`SystemSet`] for systems that initialize [`Screen::Gameplay`]
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum InputSystems {
    Cache,
    Mock,
}
