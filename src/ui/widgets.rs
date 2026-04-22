/*
 * File: widgets.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! Helper functions for creating common widgets.

use bevy::{prelude::*, ui::FocusPolicy};

use crate::{input::prelude::*, ui::prelude::*};

/// A non-scrolling root UI [`Bundle`] with centered content that fills its parent [`Node`].
pub(crate) fn root_widget(name: &'static str) -> impl Bundle {
    ui_root_bundle(name, Overflow::DEFAULT)
}

/// An auto-scrolling root UI [`Bundle`] with centered content that fills its parent [`Node`].
pub(crate) fn root_auto_scroll_widget(name: &'static str) -> impl Bundle {
    (
        ui_root_bundle(name, Overflow::scroll_y()),
        AutoScroll(Vec2::new(0., BODY_FONT_SIZE / 100.)),
        InputScroll(Vec2::new(0., BODY_FONT_SIZE)),
        UiNav,
    )
}

/// A root UI [`Bundle`] with centered content that fills its parent [`Node`].
///
/// This blocks any picking events and interactions with lower [`Node`]s.
fn ui_root_bundle(name: &'static str, overflow: Overflow) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(30),
            overflow,
            ..default()
        },
        FocusPolicy::Block,
    )
}

/// A simple header label. Bigger than [`label_widget`].
pub(crate) fn header_widget(text: impl Into<String>, font: Handle<Font>) -> impl Bundle {
    styled_text("Header", HEADER_TEXT, text, font, HEADER_FONT_SIZE)
}

/// A simple text label.
pub(crate) fn label_widget(text: impl Into<String>, font: Handle<Font>) -> impl Bundle {
    styled_text("Label", LABEL_TEXT, text, font, BODY_FONT_SIZE)
}

/// A simple styled text
fn styled_text(
    name: &'static str,
    color: Color,
    text: impl Into<String>,
    font: Handle<Font>,
    font_size: f32,
) -> impl Bundle {
    (
        Name::new(name),
        Text(text.into()),
        TextFont::from(font).with_font_size(font_size),
        TextColor(color),
    )
}
