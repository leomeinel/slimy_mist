/*
 * File: hud.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

pub(super) mod joystick;

use bevy::prelude::*;

use crate::{core::prelude::*, screens::prelude::*, ui::prelude::*};

pub(super) struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(joystick::UiJoystickPlugin);

        app.configure_sets(
            OnEnter(Screen::Gameplay),
            (HudSystems::Spawn, HudSystems::Append)
                .after(EnterGameplaySystems::Resources)
                .chain(),
        );
        app.configure_sets(
            OnEnter(JoystickState::<{ JoystickID::MOVEMENT }>::Toggled(true)),
            (HudSystems::Spawn, HudSystems::Append)
                .after(EnterGameplaySystems::Resources)
                .run_if(in_state(Screen::Gameplay))
                .chain(),
        );
        app.configure_sets(
            OnEnter(JoystickState::<{ JoystickID::MOVEMENT }>::Toggled(false)),
            (HudSystems::Spawn, HudSystems::Append)
                .after(EnterGameplaySystems::Resources)
                .run_if(in_state(Screen::Gameplay))
                .chain(),
        );

        app.add_systems(
            OnEnter(Screen::Gameplay),
            spawn_hud.in_set(HudSystems::Spawn),
        );
    }
}

/// Maximum width of any element in the [`Hud`] in pixels.
///
/// This is calculated to be equal to the maximum possible width that of any section of the [`Hud`].
pub(crate) const HUD_MAX_ELEMENT_WIDTH_PX: f32 =
    (MIN_SIDE_SCALE_THRESHOLD * (1. - HUD_PADDING_PERCENT / 50.)) / 2.;

/// A system set for ordering systems related to [`Hud`].
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) enum HudSystems {
    #[default]
    Spawn,
    Append,
}

/// Hud marker components with their position or order.
#[derive(Component, Default, PartialEq, Eq)]
pub(crate) enum Hud {
    #[default]
    Root,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// [`Node::row_gap`] for [`Hud::Root`].
const HUD_ROW_GAP: Val = Val::Percent(5.);

/// [`Node::padding`] for [`Hud::Root`] in percent.
const HUD_PADDING_PERCENT: f32 = 10.;

/// Spawn [`Hud`].
///
/// The [`Hud`] is the main way to display UI outside of menus.
///
/// It has a padding on each side and is separated into four sections that content can be added to:
///
/// - [`Hud::TopLeft`]
/// - [`Hud::TopRight`]
/// - [`Hud::BottomLeft`]
/// - [`Hud::BottomRight`]
fn spawn_hud(
    mut commands: Commands,
    #[cfg(any(target_os = "android", target_os = "ios"))] font: Res<UiFontHandle>,
) {
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        Hud::Root,
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.),
            height: percent(100.),
            padding: UiRect::all(vmin(HUD_PADDING_PERCENT)),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::percent(2, 50.),
            ..default()
        },
        children![
            (
                Hud::TopLeft,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: HUD_ROW_GAP,
                    ..default()
                },
                children![health_bar()],
            ),
            (
                Hud::TopRight,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: HUD_ROW_GAP,
                    ..default()
                },
                #[cfg(any(target_os = "android", target_os = "ios"))]
                children![pause_button(&font)],
            ),
            (
                Hud::BottomLeft,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: HUD_ROW_GAP,
                    ..default()
                },
            ),
            (
                Hud::BottomRight,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: HUD_ROW_GAP,
                    ..default()
                },
            )
        ],
    ));
}

// FIXME: This is currently static. It should scale with health.
/// Health bar showing the current [`Health`](crate::characters::prelude::Health) of the [`Player`](crate::characters::prelude::Player).
fn health_bar() -> impl Bundle {
    let bar = BarBuilder::round_big_hud()
        .with_bar_background(HEALTH_BAR_BACKGROUND)
        .build();

    (NodeRect::default(), bar)
}

#[cfg(any(target_os = "android", target_os = "ios"))]
/// Pause button.
fn pause_button(font: &UiFontHandle) -> impl Bundle {
    let button = button(
        ButtonConfig::non_navigable()
            .with_text("⋮")
            .with_header_font(font.0.clone()),
        ButtonNodeConfig::circle_big_hud(),
        enter_pause_menu_on_click,
    );

    (NodeRect::default(), button)
}
