/*
 * File: lib.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! Main with [`AppPlugin`]

// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod animations;
mod audio;
mod characters;
mod core;
#[cfg(feature = "dev")]
mod debug;
mod images;
mod input;
mod levels;
mod log;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod mobile;
mod physics;
mod procgen;
mod render;
mod screens;
mod ui;
mod utils;

#[cfg(any(target_os = "android", target_os = "ios"))]
use bevy::window::WindowMode;
use bevy::{asset::AssetMetaCheck, prelude::*, window::ScreenEdge};
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;

/// Main function for library
///
/// This structure is needed for mobile support.
#[bevy_main]
pub fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

/// AppPlugin that adds everything this app needs to run
pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Slimy Mist".to_string(),
                        fit_canvas_to_parent: true,
                        // android/ios only
                        #[cfg(any(target_os = "android", target_os = "ios"))]
                        resizable: false,
                        #[cfg(any(target_os = "android", target_os = "ios"))]
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        // ios only
                        recognize_rotation_gesture: true,
                        prefers_home_indicator_hidden: true,
                        prefers_status_bar_hidden: true,
                        preferred_screen_edges_deferring_system_gestures: ScreenEdge::Bottom,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        app.add_plugins((
            EntropyPlugin::<WyRand>::default(),
            RapierPhysicsPlugin::<()>::default(),
        ));
        app.add_plugins((
            characters::CharactersPlugin,
            #[cfg(feature = "dev")]
            debug::DebugPlugin,
            input::InputPlugin,
            #[cfg(any(target_os = "android", target_os = "ios"))]
            mobile::MobilePlugin,
            procgen::ProcGenPlugin,
            screens::ScreensPlugin,
            ui::UiPlugin,
            animations::AnimationsPlugin,
            audio::AudioPlugin,
            levels::LevelsPlugin,
            render::RenderPlugin,
            images::ImagesPlugin,
            core::CorePlugin,
        ));
    }
}
