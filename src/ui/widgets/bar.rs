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
    pub(crate) height: Val,
    pub(crate) border: UiRect,
    pub(crate) padding: UiRect,
    pub(crate) bar_background: BackgroundColor,
}
impl BarBuilder {
    pub(crate) fn round_big_hud() -> Self {
        Self {
            width: px(HUD_MAX_ELEMENT_WIDTH_PX),
            // NOTE: This ensures height consistency with big circle hud buttons.
            height: MEDIUM_BUTTON_WIDTH,
            border: UiRect::all(px(5)),
            // FIXME: This is a hack to avoid overflow not respecting `border_radius`.
            //        Additionally it is necessary because of the `Outline` hack.
            //        It should actually be 10 pixels.
            padding: UiRect::all(px(12)),
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
                height: self.height,
                border: self.border,
                border_radius: BORDER_RADIUS_ROUND,
                align_items: AlignItems::Center,
                padding: self.padding,
                ..default()
            },
            BorderColor::all(BAR_CONTAINER_BORDER),
            BackgroundColor::from(BAR_CONTAINER_BACKGROUND),
            children![(
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::RowReverse,
                    justify_content: JustifyContent::FlexStart,
                    width: percent(100),
                    height: percent(100),
                    border_radius: BORDER_RADIUS_ROUND,
                    ..default()
                },
                self.bar_background,
                children![(
                    Node {
                        width: Val::ZERO,
                        height: percent(100),
                        ..default()
                    },
                    // FIXME: This is a hack to circumvent rounding errors causing a pixel on the
                    //        right of the bar to be visible.
                    //        I sadly couldn't reproduce this in a minimal example yet, therefore
                    //        no bug report has been submitted.
                    Outline::new(px(1), Val::ZERO, BAR_CONTAINER_BACKGROUND.into()),
                    BackgroundColor::from(BAR_CONTAINER_BACKGROUND),
                )]
            )],
        )
    }
}
