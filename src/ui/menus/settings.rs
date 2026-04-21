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

use crate::{log::prelude::*, ui::prelude::*};

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
                update_joystick_button::<{ JoystickID::MOVEMENT }>
                    .before(AppUiSystems::VisualizeInteraction),
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
    let button = button(
        ButtonConfig::navigable()
            .with_text("Back")
            .with_header_font(font.0.clone()),
        ButtonNodeConfig::round_big(),
        enter_screen_back_menu_on_click,
    );

    commands.spawn((
        root_widget("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            header_widget("Settings", font.0.clone()),
            settings_grid(font.0.clone()),
            button,
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
            toggle_joystick_widget::<{ JoystickID::MOVEMENT }>(font.clone()),
        ],
    )
}

/// Label for configuration in settings.
fn settings_label(font: Handle<Font>, label: &'static str) -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::End,
            ..default()
        },
        children![label_widget(label, font.clone())],
    )
}

/// Widget to adjust global volume
fn global_volume_widget(font: Handle<Font>) -> impl Bundle {
    let button_minus = button(
        ButtonConfig::navigable()
            .with_text("-")
            .with_body_font(font.clone()),
        ButtonNodeConfig::circle_small(),
        lower_global_volume_on_click,
    );
    let button_plus = button(
        ButtonConfig::navigable()
            .with_text("+")
            .with_body_font(font.clone()),
        ButtonNodeConfig::circle_small(),
        raise_global_volume_on_click,
    );

    (
        Name::new("Global Volume Widget"),
        Node {
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            button_minus,
            (
                Name::new("Current Volume"),
                Node {
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(px(10)),
                    ..default()
                },
                children![(GlobalVolumeLabel, label_widget("", font.clone()))],
            ),
            button_plus,
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

/// Widget to toggle joystick with `const ID`.
fn toggle_joystick_widget<const ID: u8>(font: Handle<Font>) -> impl Bundle {
    let switch = switch(
        ButtonConfig::navigable().with_body_font(font.clone()),
        ButtonNodeConfig::round_medium(),
        toggle_joystick_on_click::<ID>,
    );

    (
        Name::new("Toggle Joystick Widget"),
        Node {
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(ToggleJoystickButton::<ID>, switch)],
    )
}

/// Toggle [`JoystickState<ID>`].
fn toggle_joystick_on_click<const ID: u8>(
    _: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<JoystickState<ID>>>,
    state: Res<State<JoystickState<ID>>>,
) {
    (*next_state).set_if_neq(JoystickState::<ID>::Toggled(!state.is_active()));
}

/// Update button for joystick with `const ID`.
fn update_joystick_button<const ID: u8>(
    button_container_children: Single<
        &Children,
        (With<ToggleJoystickButton<ID>>, With<ButtonContainer>),
    >,
    mut button_query: Query<(&mut InteractionPalette, &mut BoxShadow, &Children), With<Button>>,
    mut text_query: Query<&mut Text, With<ButtonText>>,
    state: Res<State<JoystickState<ID>>>,
) {
    let (shadow_color, switch_color, hover_color, new_text) = if state.is_active() {
        (
            SWITCH_SHADOW_ON,
            SWITCH_ON_BACKGROUND,
            SWITCH_ON_HOVERED_BACKGROUND,
            "On",
        )
    } else {
        (
            SWITCH_SHADOW_OFF,
            SWITCH_OFF_BACKGROUND,
            SWITCH_OFF_HOVERED_BACKGROUND,
            "Off",
        )
    };

    let button = button_container_children
        .iter()
        .find(|e| button_query.contains(*e))
        .expect(ERR_INVALID_CHILDREN);
    let (mut palette, mut box_shadow, children) =
        button_query.get_mut(button).expect(ERR_INVALID_CHILDREN);
    palette.none = switch_color.into();
    palette.hovered = hover_color.into();
    let shadow = box_shadow.0.first_mut().expect(ERR_INVALID_BOX_SHADOW);
    shadow.color = shadow_color.into();

    let text = children
        .iter()
        .find(|e| text_query.contains(*e))
        .expect(ERR_INVALID_CHILDREN);
    let mut text = text_query.get_mut(text).expect(ERR_INVALID_CHILDREN);
    text.0 = new_text.to_uppercase();
}
