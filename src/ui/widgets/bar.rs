/*
 * File: bar.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;

use crate::ui::prelude::*;

/// A builder for creating bar [`Bundle`]s with customizable appearance.
#[derive(Default)]
pub(crate) struct BarBuilder {
    pub(crate) width: Val,
    pub(crate) aspect_ratio: f32,
    pub(crate) border: UiRect,
    pub(crate) bar_background: BackgroundColor,
}
impl BarBuilder {
    pub(crate) fn round_big_hud() -> Self {
        Self {
            width: HUD_MAX_ELEMENT_WIDTH,
            aspect_ratio: 4.5,
            border: UiRect::all(px(5)),
            ..default()
        }
    }
    pub(crate) fn with_bar_background<T>(self, color: T) -> Self
    where
        T: Into<Color>,
    {
        Self {
            bar_background: BackgroundColor::from(color),
            ..self
        }
    }
    pub(crate) fn build(self) -> impl Bundle {
        (
            Node {
                width: self.width,
                aspect_ratio: Some(self.aspect_ratio),
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Start,
                border: self.border,
                border_radius: BorderRadius::all(px(30)),
                padding: self.border,
                ..default()
            },
            BorderColor::all(BAR_CONTAINER_BORDER),
            BackgroundColor::from(BAR_CONTAINER_BACKGROUND),
            children![(
                Node {
                    border_radius: BorderRadius::all(px(30)),
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                self.bar_background,
            )],
        )
    }
}
