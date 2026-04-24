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
    pub(crate) padding: UiRect,
    pub(crate) bar_background: BackgroundColor,
}
impl BarBuilder {
    pub(crate) fn round_big_hud() -> Self {
        Self {
            width: px(HUD_MAX_ELEMENT_WIDTH_PX),
            // NOTE: This ensures height consistency with big circle hud buttons.
            height: MEDIUM_BUTTON_WIDTH,
            padding: UiRect::all(px(15)),
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
                border_radius: BorderRadius::MAX,
                align_items: AlignItems::Center,
                padding: self.padding,
                ..default()
            },
            BackgroundColor::from(BAR_CONTAINER_BACKGROUND),
            children![(
                Node {
                    width: percent(100),
                    height: percent(100),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                self.bar_background,
                children![(
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::ZERO,
                        width: Val::ZERO,
                        height: percent(100),
                        ..default()
                    },
                    // FIXME: This is a hack to circumvent rounding errors causing a pixel on the
                    //        right of the bar to be visible.
                    //        Also see: https://github.com/bevyengine/bevy/issues/23964
                    Outline::new(px(1), Val::ZERO, BAR_CONTAINER_BACKGROUND.into()),
                    BackgroundColor::from(BAR_CONTAINER_BACKGROUND),
                )]
            )],
        )
    }
}
