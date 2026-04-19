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

use crate::{screens::prelude::*, ui::prelude::*};

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
            padding: UiRect::all(vmin(10)),
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
                children![(
                    #[cfg(any(target_os = "android", target_os = "ios"))]
                    pause_button(&font),
                )],
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

#[cfg(any(target_os = "android", target_os = "ios"))]
/// Pause button.
fn pause_button(font: &UiFontHandle) -> impl Bundle {
    let button_config = ButtonNodeConfig::circle_medium();
    let button_width = button_config.width;
    let button = button(
        ButtonConfig::non_navigable()
            .with_text("⋮")
            .with_header_font(font.0.clone()),
        button_config,
        enter_pause_menu_on_click,
    );

    (
        Node {
            width: button_width,
            height: button_width,
            ..default()
        },
        NodeRect::default(),
        DespawnOnExit(Screen::Gameplay),
        button,
    )
}
