/*
 * File: menus.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! The game's menus and transitions between them.

pub(super) mod credits;
mod pause;
mod settings;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{screens::prelude::*, ui::prelude::*};

pub(super) struct MenusPlugin;
impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((pause::PausePlugin, settings::SettingsPlugin));

        app.init_state::<Menu>();

        app.add_systems(OnExit(Screen::Gameplay), exit_menus);
        app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
        app.add_systems(
            OnEnter(Menu::Credits),
            (credits::spawn_credits_menu, credits::start_credits_music),
        );

        app.add_systems(
            Update,
            exit_menus.run_if(
                not(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP))
                    .and(in_state(Screen::Gameplay)),
            ),
        );
        app.add_systems(
            Update,
            enter_main_menu
                .run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
        );
    }
}

/// The game's main menu
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
}

/// Spawn Main menu with [`State`] changing buttons.
fn spawn_main_menu(mut commands: Commands, font: Res<UiFontHandle>) {
    commands.spawn((
        root_widget("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(any(target_family = "wasm", target_os = "android", target_os = "ios")))]
        children![
            button_rounded(
                None,
                "Play",
                font.0.clone(),
                true,
                enter_gameplay_screen_on_click
            ),
            button_rounded(
                None,
                "Settings",
                font.0.clone(),
                true,
                enter_settings_menu_on_click
            ),
            button_rounded(
                None,
                "Credits",
                font.0.clone(),
                true,
                enter_credits_menu_on_click
            ),
            button_rounded(None, "Exit", font.0.clone(), true, exit_app_on_click),
        ],
        // Do not add exit button for wasm, android and ios
        #[cfg(any(target_family = "wasm", target_os = "android", target_os = "ios"))]
        children![
            button_rounded(
                None,
                "Play",
                font.0.clone(),
                true,
                enter_gameplay_screen_on_click
            ),
            button_rounded(
                None,
                "Settings",
                font.0.clone(),
                true,
                enter_settings_menu_on_click
            ),
            button_rounded(
                None,
                "Credits",
                font.0.clone(),
                true,
                enter_credits_menu_on_click
            ),
        ],
    ));
}

/// Exit [`Menu`]s.
pub(crate) fn exit_menus(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::None);
}

/// Exit [`Menu`]s on [`Pointer`] click.
pub(crate) fn exit_menus_on_click(_: On<Pointer<Click>>, next_state: ResMut<NextState<Menu>>) {
    exit_menus(next_state);
}

/// Enter [`Menu::Main`].
pub(crate) fn enter_main_menu(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Main);
}

/// Enter [`Menu::Main`].
pub(crate) fn enter_main_menu_on_click(_: On<Pointer<Click>>, next_state: ResMut<NextState<Menu>>) {
    enter_main_menu(next_state);
}

/// Enter [`Menu::Credits`] on [`Pointer`] click.
fn enter_credits_menu_on_click(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Credits);
}

/// Enter [`Menu::Settings`] on [`Pointer`] click.
pub(crate) fn enter_settings_menu_on_click(
    _: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<Menu>>,
) {
    (*next_state).set_if_neq(Menu::Settings);
}

/// Enter [`Menu::Pause`].
pub(crate) fn enter_pause_menu(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Pause);
}

/// Enter [`Menu::Pause`] on [`Pointer`] click.
#[cfg(any(target_os = "android", target_os = "ios"))]
pub(crate) fn enter_pause_menu_on_click(
    _: On<Pointer<Click>>,
    next_state: ResMut<NextState<Menu>>,
) {
    enter_pause_menu(next_state);
}

/// Enter [`Screen::back_menu()`].
pub(crate) fn enter_screen_back_menu(
    mut next_state: ResMut<NextState<Menu>>,
    state: Res<State<Screen>>,
) {
    (*next_state).set_if_neq(state.back_menu());
}

/// Enter [`Screen::back_menu()`] on [`Pointer`] click.
pub(crate) fn enter_screen_back_menu_on_click(
    _: On<Pointer<Click>>,
    next_state: ResMut<NextState<Menu>>,
    state: Res<State<Screen>>,
) {
    enter_screen_back_menu(next_state, state);
}

/// Exit [`App`] on [`Pointer`] click.
#[cfg(not(any(target_family = "wasm", target_os = "android", target_os = "ios")))]
fn exit_app_on_click(_: On<Pointer<Click>>, mut app_exit_msg: MessageWriter<AppExit>) {
    app_exit_msg.write(AppExit::Success);
}
