/*
 * File: screens.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;

pub(crate) mod prelude {
    pub(crate) use super::Screen;
    pub(crate) use super::gameplay::EnterGameplaySystems;
    pub(crate) use super::splash::SplashAssets;
}

use bevy::prelude::*;

use crate::ui::prelude::*;

pub(super) struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            gameplay::GameplayPlugin,
            loading::LoadingPlugin,
            splash::SplashPlugin,
        ));

        app.init_state::<Screen>();

        app.add_systems(OnEnter(Screen::Title), open_main_menu);
        app.add_systems(OnExit(Screen::Title), close_main_menu);
    }
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) enum Screen {
    #[default]
    Loading,
    LoadingCache,
    Splash,
    Title,
    Gameplay,
}
impl Screen {
    pub(crate) fn back_menu(&self) -> Menu {
        match self {
            Screen::Title => Menu::Main,
            _ => Menu::Pause,
        }
    }
}

/// Open main menu
fn open_main_menu(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Main);
}

/// Close main menu
fn close_main_menu(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::None);
}
