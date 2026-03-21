/*
 * File: pause.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! The pause menu.

use bevy::prelude::*;

use crate::{screens::prelude::*, ui::prelude::*};

/// Spawn pause menu
pub(super) fn spawn_pause_menu(mut commands: Commands, font: Res<UiFontHandle>) {
    commands.spawn((
        root_widget("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            header_widget("Game paused", font.0.clone()),
            button_large("Continue", font.0.clone(), go_back_on_click),
            button_large("Settings", font.0.clone(), open_settings_on_click),
            button_large("Quit to title", font.0.clone(), quit_to_title_on_click),
        ],
    ));
}

/// Go back to [`Menu::None`].
pub(super) fn go_back(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::None);
}

/// Call [`go_back`] on pointer click.
fn go_back_on_click(_: On<Pointer<Click>>, next_state: ResMut<NextState<Menu>>) {
    go_back(next_state);
}

/// Open settings
fn open_settings_on_click(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Settings);
}

/// Quit to title
fn quit_to_title_on_click(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<Screen>>) {
    (*next_state).set_if_neq(Screen::Title);
}
