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

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{core::prelude::*, screens::prelude::*, ui::prelude::*, utils::prelude::*};

pub(super) struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Menu::None),
            unpause.run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(OnEnter(Menu::Pause), (pause, spawn_pause_menu));
        app.add_systems(OnEnter(Pause(true)), spawn_pause_overlay);

        app.add_systems(
            Update,
            exit_menus.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
        );
        app.add_systems(
            Update,
            enter_pause_menu.run_if(
                in_state(Menu::None)
                    .and(
                        input_just_pressed(KeyCode::KeyP)
                            .or(input_just_pressed(KeyCode::Escape))
                            .or(window_unfocused),
                    )
                    .and(in_state(Screen::Gameplay)),
            ),
        );

        app.add_systems(OnExit(Screen::Gameplay), unpause);
    }
}

/// Spawn pause overlay.
fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(PAUSE_BACKGROUND),
        DespawnOnExit(Pause(true)),
    ));
}

/// Spawn pause menu.
fn spawn_pause_menu(mut commands: Commands, font: Res<UiFontHandle>) {
    commands.spawn((
        root_widget("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            header_widget("Game paused", font.0.clone()),
            button_rounded(None, "Continue", font.0.clone(), true, exit_menus_on_click),
            button_rounded(
                None,
                "Settings",
                font.0.clone(),
                true,
                enter_settings_menu_on_click
            ),
            button_rounded(
                None,
                "Quit to title",
                font.0.clone(),
                true,
                enter_title_screen_on_click
            ),
        ],
    ));
}

/// Unpause the game
fn unpause(mut next_state: ResMut<NextState<Pause>>) {
    (*next_state).set_if_neq(Pause(false));
}

/// Pause the game
fn pause(mut next_state: ResMut<NextState<Pause>>) {
    (*next_state).set_if_neq(Pause(true));
}
