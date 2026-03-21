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
        app.init_state::<Menu>();

        app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
        app.add_systems(
            OnEnter(Menu::Credits),
            (credits::spawn_credits_menu, credits::start_credits_music),
        );
        app.add_systems(
            Update,
            credits::go_back
                .run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
        );

        app.add_systems(OnEnter(Menu::Pause), pause::spawn_pause_menu);
        app.add_systems(
            Update,
            pause::go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
        );

        app.add_systems(OnEnter(Menu::Settings), settings::spawn_settings_menu);
        app.add_systems(
            Update,
            settings::go_back
                .run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
        );
        app.add_systems(
            Update,
            (
                settings::update_joystick_button.before(AppUiSystems::VisualizeInteraction),
                settings::update_global_volume_label,
            )
                .run_if(in_state(Menu::Settings)),
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

fn spawn_main_menu(mut commands: Commands, font: Res<UiFontHandle>) {
    // Spawn Main menu with state changing buttons
    commands.spawn((
        root_widget("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(any(target_family = "wasm", target_os = "android", target_os = "ios")))]
        children![
            button_large("Play", font.0.clone(), enter_gameplay_screen),
            button_large("Settings", font.0.clone(), open_settings_menu),
            button_large("Credits", font.0.clone(), open_credits_menu),
            button_large("Exit", font.0.clone(), exit_app),
        ],
        // Do not add exit button for wasm, android and ios
        #[cfg(any(target_family = "wasm", target_os = "android", target_os = "ios"))]
        children![
            button_large("Play", font.0.clone(), enter_gameplay_screen),
            button_large("Settings", font.0.clone(), open_settings_menu),
            button_large("Credits", font.0.clone(), open_credits_menu),
        ],
    ));
}

/// Enter the gameplay screen
fn enter_gameplay_screen(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<Screen>>) {
    (*next_state).set_if_neq(Screen::Gameplay);
}

/// Open settings
fn open_settings_menu(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Settings);
}

/// Open credits
fn open_credits_menu(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Credits);
}

/// Exit the app
#[cfg(not(any(target_family = "wasm", target_os = "android", target_os = "ios")))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit_msg: MessageWriter<AppExit>) {
    app_exit_msg.write(AppExit::Success);
}
