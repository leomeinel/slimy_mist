/*
 * Heavily inspired by:
 * - https://github.com/TheBevyFlock/bevy_new_2d
 */

//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;

pub(crate) mod prelude {
    pub(crate) use super::gameplay::EnterGameplaySystems;
    pub(crate) use super::splash::SplashAssets;
    pub(crate) use super::{
        Screen, enter_gameplay_screen_on_click, enter_splash_screen, enter_title_screen,
        enter_title_screen_on_click,
    };
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

        app.add_systems(OnEnter(Screen::Title), enter_main_menu);
        app.add_systems(OnExit(Screen::Title), exit_menus);
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

/// Enter [`Screen::Splash`].
pub(crate) fn enter_splash_screen(mut next_state: ResMut<NextState<Screen>>) {
    (*next_state).set_if_neq(Screen::Splash);
}

/// Enter [`Screen::Title`].
pub(crate) fn enter_title_screen(mut next_state: ResMut<NextState<Screen>>) {
    (*next_state).set_if_neq(Screen::Title);
}

/// Enter [`Screen::Title`] title on [`Pointer`] click.
pub(crate) fn enter_title_screen_on_click(
    _: On<Pointer<Click>>,
    next_state: ResMut<NextState<Screen>>,
) {
    enter_title_screen(next_state);
}
/// Enter [`Screen::Gameplay`].
pub(crate) fn enter_gameplay_screen_on_click(
    _: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    (*next_state).set_if_neq(Screen::Gameplay);
}
