/*
 * File: settings.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*};

use crate::{input::prelude::*, log::prelude::*, ui::prelude::*};

pub(super) struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);

        app.add_systems(
            Update,
            enter_screen_back_menu
                .run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
        );
        app.add_systems(
            Update,
            (
                update_joystick_button.before(AppUiSystems::VisualizeInteraction),
                update_global_volume_label,
            )
                .run_if(in_state(Menu::Settings)),
        );
    }
}

/// Global volume label marker
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct GlobalVolumeLabel;

/// Toggle joystick button marker
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct ToggleJoystickButton<const ID: u8>;

/// Spawn settings menu
fn spawn_settings_menu(mut commands: Commands, font: Res<UiFontHandle>) {
    commands.spawn((
        root_widget("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            header_widget("Settings", font.0.clone()),
            settings_grid(font.0.clone()),
            button_rounded(
                None,
                "Back",
                font.0.clone(),
                true,
                enter_screen_back_menu_on_click
            ),
        ],
    ));
}

/// Total width of [`settings_grid`].
const SETTINGS_GRID_WIDTH_PX: f32 = 400.;

/// Custom settings grid
fn settings_grid(font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            width: px(SETTINGS_GRID_WIDTH_PX),
            display: Display::Grid,
            row_gap: px(BODY_FONT_SIZE / 2.),
            grid_template_columns: RepeatedGridTrack::px(2, SETTINGS_GRID_WIDTH_PX / 2.),
            ..default()
        },
        children![
            settings_label(font.clone(), "Master Volume"),
            global_volume_widget(font.clone()),
            settings_label(font.clone(), "Joystick"),
            toggle_joystick_widget(font.clone()),
        ],
    )
}

/// Label for configuration in settings.
fn settings_label(font: Handle<Font>, label: &'static str) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::End,
            ..default()
        },
        children![label_widget(label, font.clone())],
    )
}

/// Widget to adjust global volume
fn global_volume_widget(font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            button_circle(None, "-", font.clone(), true, lower_global_volume_on_click),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    ..default()
                },
                children![(GlobalVolumeLabel, label_widget("", font.clone()))],
            ),
            button_circle(None, "+", font.clone(), true, raise_global_volume_on_click),
        ],
    )
}

/// Minimum global volume
const MIN_VOLUME: f32 = 0.0;
/// Maximum global volume
const MAX_VOLUME: f32 = 3.0;

/// Lower global volume
fn lower_global_volume_on_click(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

/// Raise global volume
fn raise_global_volume_on_click(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

/// Update global volume label that displays volume
fn update_global_volume_label(
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
    global_volume: Res<GlobalVolume>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

/// Widget to toggle movement joystick
fn toggle_joystick_widget(font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Toggle Joystick Widget"),
        Node {
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            ToggleJoystickButton::<{ JoystickID::Movement as u8 }>,
            switch_rounded(None, "", font.clone(), true, toggle_joystick_on_click),
        )],
    )
}

/// Toggle [`JoystickState<ID>`].
fn toggle_joystick_on_click(
    _: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<JoystickState<{ JoystickID::Movement as u8 }>>>,
    state: Res<State<JoystickState<{ JoystickID::Movement as u8 }>>>,
) {
    (*next_state).set_if_neq(JoystickState::<{ JoystickID::Movement as u8 }>::Toggled(
        !state.is_active(),
    ));
}

/// Update global volume label that displays volume
fn update_joystick_button(
    children: Single<&Children, With<ToggleJoystickButton<{ JoystickID::Movement as u8 }>>>,
    mut base_query: Query<(&mut BackgroundColor, &Children), (With<ButtonBase>, Without<Button>)>,
    mut surface_query: Query<
        (&mut InteractionPalette, &Children),
        (With<Button>, Without<ButtonBase>),
    >,
    mut text_query: Query<&mut Text, With<ButtonText>>,
    state: Res<State<JoystickState<{ JoystickID::Movement as u8 }>>>,
) {
    let (base_color, surface_color, hover_color, new_text) = if state.is_active() {
        (
            SWITCH_BASE_ON_BACKGROUND,
            SWITCH_ON_BACKGROUND,
            SWITCH_ON_HOVERED_BACKGROUND,
            "On",
        )
    } else {
        (
            SWITCH_BASE_OFF_BACKGROUND,
            SWITCH_OFF_BACKGROUND,
            SWITCH_OFF_HOVERED_BACKGROUND,
            "Off",
        )
    };

    // `ButtonBase`
    let child = children
        .iter()
        .find(|e| base_query.contains(*e))
        .expect(ERR_INVALID_CHILDREN);
    let (mut background, children) = base_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
    background.0 = base_color.into();

    // `Button`
    let child = children
        .iter()
        .find(|e| surface_query.contains(*e))
        .expect(ERR_INVALID_CHILDREN);
    let (mut palette, children) = surface_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
    palette.none = surface_color.into();
    palette.hovered = hover_color.into();

    // `ButtonText`
    let child = children
        .iter()
        .find(|e| text_query.contains(*e))
        .expect(ERR_INVALID_CHILDREN);
    let mut text = text_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
    text.0 = new_text.to_uppercase();
}
