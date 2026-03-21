/*
 * File: ui.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! Reusable UI widgets & theming.

mod buttons;
mod interaction;
mod menus;
mod palette;
mod widgets;

pub(crate) mod prelude {
    pub(crate) use super::buttons::{
        ButtonBase, ButtonText, button_large, button_small, switch_medium,
    };
    pub(crate) use super::interaction::{
        InteractionAssets, InteractionOverride, InteractionPalette, OverrideInteraction,
    };
    pub(crate) use super::menus::Menu;
    pub(crate) use super::menus::credits::{
        CreditsAssets, CreditsData, CreditsDataCache, CreditsHandle,
    };
    pub(crate) use super::palette::*;
    pub(crate) use super::widgets::{
        header_widget, label_widget, root_auto_scroll_widget, root_widget,
    };
    pub(crate) use super::{
        AppUiSystems, BODY_FONT_SIZE, HEADER_FONT_SIZE, NodeOffset, UiFontHandle,
    };
}

use bevy::{
    input_focus::{
        InputDispatchPlugin, InputFocusVisible,
        directional_navigation::{AutoNavigationConfig, DirectionalNavigationPlugin},
    },
    prelude::*,
};

pub(super) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin));
        app.add_plugins((menus::MenusPlugin, interaction::UiInteractionPlugin));

        app.insert_resource(InputFocusVisible(true));
        // FIXME: This currently sometimes navigates in weird ways. This is especially visible in the `Settings`
        //        `Menu`. The current `min_alignment_factor` is usable, but still not great.
        app.insert_resource(AutoNavigationConfig {
            min_alignment_factor: 0.01,
            prefer_aligned: true,
            ..default()
        });
    }
}

/// Font size for any header.
pub(crate) const HEADER_FONT_SIZE: f32 = 36.;
/// Font size for any body.
pub(crate) const BODY_FONT_SIZE: f32 = 18.;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(crate) enum AppUiSystems {
    VisualizeInteraction,
}

/// Wrapper for [`Handle<Font>`] for the ui.
#[derive(Resource, Default)]
pub(crate) struct UiFontHandle(pub(crate) Handle<Font>);

/// Offset that stores the offset for a [`Node`].
///
/// Can apply to [`Node::left`] and [`Node::bottom`] according to [`Self::0`].
#[derive(Component, Default)]
pub(crate) struct NodeOffset(pub(crate) IVec2);
