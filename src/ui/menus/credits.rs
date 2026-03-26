/*
 * File: credits.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! The credits menu.

use bevy::{ecs::spawn::SpawnIter, prelude::*};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

use crate::{audio::prelude::*, ui::prelude::*};

/// Credits data deserialized from a ron file.
#[derive(Deserialize, Asset, TypePath, Default)]
pub(crate) struct CreditsData {
    #[serde(default)]
    pub(crate) created_by: Vec<[String; 2]>,
    #[serde(default)]
    pub(crate) assets: Vec<[String; 2]>,
    #[serde(default)]
    pub(crate) code: Vec<[String; 2]>,
}

/// Handle for [`CreditsData`].
#[derive(Resource)]
pub(crate) struct CreditsHandle(pub(crate) Handle<CreditsData>);

/// Cache for [`CreditsData`]
///
/// This is to allow easier access.
#[derive(Resource, Default)]
pub(crate) struct CreditsDataCache {
    pub(crate) created_by: Vec<[String; 2]>,
    pub(crate) assets: Vec<[String; 2]>,
    pub(crate) code: Vec<[String; 2]>,
}

/// Assets for credits
#[derive(AssetCollection, Resource)]
pub(crate) struct CreditsAssets {
    #[asset(path = "audio/music/screen-saver.ogg")]
    music: Handle<AudioSource>,
}

/// Spawn menu with credits for assets and creators of the game
pub(super) fn spawn_credits_menu(
    mut commands: Commands,
    credits_data: Res<CreditsDataCache>,
    font: Res<UiFontHandle>,
) {
    commands.spawn((
        root_auto_scroll_widget("Credits Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Credits),
        children![
            header_widget("Created by", font.0.clone()),
            grid(credits_data.created_by.clone(), font.0.clone()),
            header_widget("Assets", font.0.clone()),
            grid(credits_data.assets.clone(), font.0.clone()),
            header_widget("Code", font.0.clone()),
            grid(credits_data.code.clone(), font.0.clone()),
            button_large("Back", font.0.clone(), go_back_on_click),
        ],
    ));
}

/// Play music for credits
pub(super) fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        DespawnOnExit(Menu::Credits),
        music(credits_music.music.clone()),
    ));
}

/// Grid with custom settings that fit the credits screen
fn grid(content: Vec<[String; 2]>, font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            move |(i, text)| {
                (
                    label_widget(text, font.clone()),
                    Node {
                        justify_self: if i.is_multiple_of(2) {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

/// Go back to [`Menu::Main`].
pub(super) fn go_back(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Main);
}

/// Go back to main menu on click
fn go_back_on_click(_: On<Pointer<Click>>, next_state: ResMut<NextState<Menu>>) {
    go_back(next_state);
}
