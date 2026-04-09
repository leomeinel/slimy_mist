/*
 * File: splash.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! A splash screen that plays briefly at startup.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::{
    core::prelude::*, images::prelude::*, screens::prelude::*, ui::prelude::*, utils::prelude::*,
};

pub(super) struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_BACKGROUND.into()));

        app.add_systems(
            OnEnter(Screen::Splash),
            (spawn_splash_screen, insert_splash_timer),
        );
        app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
        app.add_systems(
            Update,
            enter_title_screen
                .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::Splash))),
        );
        app.add_systems(
            Update,
            (
                tick_fade_in_out::<SplashImage>.in_set(AppSystems::TickTimers),
                apply_fade_in_out.in_set(AppSystems::Update),
                tick_resource_timer::<SplashTimer>.in_set(AppSystems::TickTimers),
                check_splash_timer.in_set(AppSystems::Update),
            )
                .run_if(in_state(Screen::Splash)),
        );
    }
}

/// Assets for splash screen
#[derive(AssetCollection, Resource)]
pub(crate) struct SplashAssets {
    #[asset(path = "images/ui/splash.webp")]
    #[asset(image(sampler(filter = linear)))]
    splash: Handle<Image>,
}

/// Marker [`Component`] for splash screen image.
#[derive(Component)]
struct SplashImage;

/// Timer that tracks splash screen
#[derive(Resource, Debug, Clone, PartialEq, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
struct SplashTimer(Timer);
impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

/// Default display duration of the splash screen
const SPLASH_DURATION_SECS: f32 = 1.8;
/// Fade duration of the splash screen
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

/// Spawn splash screen
fn spawn_splash_screen(mut commands: Commands, splash_assets: Res<SplashAssets>) {
    commands.spawn((
        root_widget("Splash Screen"),
        BackgroundColor(CLEAR_BACKGROUND.into()),
        DespawnOnExit(Screen::Splash),
        children![(
            Name::new("Splash image"),
            SplashImage,
            Node {
                margin: UiRect::all(auto()),
                width: percent(70),
                ..default()
            },
            ImageNode::new(splash_assets.splash.clone()),
            FadeInOut {
                total_duration: SPLASH_DURATION_SECS,
                fade_duration: SPLASH_FADE_DURATION_SECS,
                t: 0.0,
            },
        )],
    ));
}

/// Check status of [`SplashTimer`]
fn check_splash_timer(next_state: ResMut<NextState<Screen>>, timer: Res<SplashTimer>) {
    if timer.0.just_finished() {
        enter_title_screen(next_state);
    }
}

/// Initialize [`SplashTimer`]
fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

/// Remove [`SplashTimer`]
fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}
