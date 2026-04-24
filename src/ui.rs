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

mod hud;
mod interaction;
mod menus;
mod palette;
mod widgets;
mod world;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::hud::joystick::{JoystickID, JoystickMap, JoystickState};
    pub(crate) use super::hud::{HUD_MAX_ELEMENT_WIDTH_PX, Hud, HudSystems};
    pub(crate) use super::interaction::{
        InteractionAssets, InteractionOverride, InteractionPalette, OverrideInteraction,
    };
    pub(crate) use super::menus::credits::{
        CreditsAssets, CreditsData, CreditsDataCache, CreditsHandle,
    };
    #[cfg(any(target_os = "android", target_os = "ios"))]
    pub(crate) use super::menus::enter_pause_menu_on_click;
    pub(crate) use super::menus::{
        Menu, enter_main_menu, enter_main_menu_on_click, enter_pause_menu, enter_screen_back_menu,
        enter_screen_back_menu_on_click, enter_settings_menu_on_click, exit_menus,
        exit_menus_on_click,
    };
    pub(crate) use super::palette::*;
    pub(crate) use super::widgets::bar::BarBuilder;
    pub(crate) use super::widgets::button::{
        ButtonConfig, ButtonContainer, ButtonNodeConfig, ButtonText, MEDIUM_BUTTON_WIDTH, button,
        switch,
    };
    pub(crate) use super::widgets::{
        ROOT_MAX_ELEMENT_WIDTH_PX, header_widget, label_widget, root_auto_scroll_widget,
        root_widget,
    };
    pub(crate) use super::world::health::{
        WORLD_UI_HEALTH_BAR_HEIGHT, WORLD_UI_HEALTH_BAR_PADDING, WORLD_UI_HEALTH_BAR_WIDTH,
        WorldUiHealthBarMap,
    };
    pub(crate) use super::world::{WorldUi, WorldUiAnchor};
    pub(crate) use super::{
        AppUiSystems, BODY_FONT_SIZE, BORDER_RADIUS_ROUND_BIG, BORDER_RADIUS_ROUND_MEDIUM,
        HEADER_FONT_SIZE, NodeRect, UiFontHandle,
    };
}

use bevy::{
    input_focus::{
        InputDispatchPlugin, InputFocusVisible, directional_navigation::DirectionalNavigationPlugin,
    },
    prelude::*,
    window::WindowResized,
};

use crate::core::prelude::*;

pub(super) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin));
        app.add_plugins((
            menus::MenusPlugin,
            interaction::UiInteractionPlugin,
            hud::HudPlugin,
            world::WorldUiPlugin,
        ));

        app.insert_resource(InputFocusVisible(true));

        app.add_systems(Update, scale_ui);
        app.add_systems(
            PostUpdate,
            update_node_rects.after(TransformSystems::Propagate),
        );
    }
}

/// [`BorderRadius`] for round big [`Node`]s.
pub(crate) const BORDER_RADIUS_ROUND_BIG: BorderRadius = BorderRadius::all(Val::Px(40.));
/// [`BorderRadius`] for round medium [`Node`]s.
pub(crate) const BORDER_RADIUS_ROUND_MEDIUM: BorderRadius = BorderRadius::all(Val::Px(15.));

/// Font size for any header.
pub(crate) const HEADER_FONT_SIZE: f32 = 54.;
/// Font size for any body.
pub(crate) const BODY_FONT_SIZE: f32 = 27.;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(crate) enum AppUiSystems {
    VisualizeInteraction,
}

/// Wrapper for [`Handle<Font>`] for the ui.
#[derive(Resource, Default)]
pub(crate) struct UiFontHandle(pub(crate) Handle<Font>);

/// [`Rect`] of a [`Node`] in screen space.
///
/// This is useful since it allows directly checking if it contains pointer positions.
#[derive(Component, Default)]
#[require(Node)]
pub(crate) struct NodeRect(pub(crate) Rect);
impl NodeRect {
    pub(crate) fn touched_id(&self, touches: &Touches) -> Option<u64> {
        // NOTE: We need both `iter()` and `iter_just_released()` since we care about pressed and released.
        touches
            .iter()
            .chain(touches.iter_just_released())
            .find_map(|t| {
                if self.0.contains(t.start_position()) || self.0.contains(t.position()) {
                    Some(t.id())
                } else {
                    None
                }
            })
    }
    pub(crate) fn clicked(
        &self,
        mouse: &ButtonInput<MouseButton>,
        mouse_button: &MouseButton,
        cursor_pos: Option<Vec2>,
        start_pos: Option<Vec2>,
    ) -> bool {
        (mouse.pressed(*mouse_button) || mouse.just_released(*mouse_button))
            && (cursor_pos.is_some_and(|p| self.0.contains(p))
                || start_pos.is_some_and(|p| self.0.contains(p)))
    }
}

/// Update [`NodeRect`]s from [`ComputedNode`]s and [`UiGlobalTransform`].
fn update_node_rects(
    node_query: Query<(&mut NodeRect, &ComputedNode, &UiGlobalTransform)>,
    node_changed_query: Query<(), Or<(Changed<ComputedNode>, Changed<UiGlobalTransform>)>>,
    ui_scale: Res<UiScale>,
) {
    if node_changed_query.is_empty() && !ui_scale.is_changed() {
        return;
    }
    for (mut rect, node, transform) in node_query {
        // NOTE: Factor is used for converting to screen space.
        let factor = node.inverse_scale_factor * ui_scale.0;
        rect.0 = Rect::from_center_size(transform.translation * factor, node.size() * factor);
    }
}

/// Scale [`UiScale`] according to [`MIN_SIDE_SCALE_THRESHOLD`].
fn scale_ui(mut reader: MessageReader<WindowResized>, mut ui_scale: ResMut<UiScale>) {
    for resized in reader.read() {
        let min_length = resized.width.min(resized.height);
        ui_scale.0 = if min_length > MIN_SIDE_SCALE_THRESHOLD {
            1.
        } else {
            min_length / MIN_SIDE_SCALE_THRESHOLD
        };
    }
}
