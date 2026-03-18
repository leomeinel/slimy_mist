/*
 * File: input.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

pub(crate) mod actions;
pub(crate) mod joystick;
mod mock;
pub(crate) mod pointer;
pub(crate) mod ui;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{characters::player::Player, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // Add library plugins
    app.add_plugins(EnhancedInputPlugin);

    // Add child plugins
    app.add_plugins((joystick::plugin, mock::plugin, pointer::plugin, ui::plugin));

    // Order new `InitGameplaySystems` variants by adding them here:
    app.configure_sets(
        PreUpdate,
        (InputSystems::Cache, InputSystems::Mock)
            .before(EnhancedInputSystems::Update)
            .run_if(in_state(Screen::Gameplay))
            .chain(),
    );

    // Handle bevy_enhanced_input with input context and observers
    app.add_input_context::<Player>();
    app.add_observer(actions::apply_walk);
    app.add_observer(actions::reset_walk);
    app.add_observer(actions::set_jump);
    app.add_observer(actions::trigger_melee_attack);
    app.add_observer(actions::reset_aim);
}

/// A [`SystemSet`] for systems that initialize [`Screen::Gameplay`]
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum InputSystems {
    Cache,
    Mock,
}
