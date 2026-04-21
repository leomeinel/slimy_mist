/*
 * File: buttons.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::auto_directional_navigation::AutoDirectionalNavigation,
};

use crate::{input::prelude::*, ui::prelude::*};

/// Button container marker.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct ButtonContainer;

/// Button text marker.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct ButtonText;

/// Button config.
#[derive(Default)]
pub(crate) struct ButtonConfig {
    pub(crate) navigable: bool,
    pub(crate) text: &'static str,
    pub(crate) text_font: TextFont,
}
impl ButtonConfig {
    pub(crate) fn navigable() -> Self {
        Self {
            navigable: true,
            ..default()
        }
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    pub(crate) fn non_navigable() -> Self {
        Self::default()
    }
    pub(crate) fn with_text(self, text: &'static str) -> Self {
        Self { text, ..self }
    }
    pub(crate) fn with_body_font(self, font: Handle<Font>) -> Self {
        Self {
            text_font: TextFont::from(font).with_font_size(BODY_FONT_SIZE),
            ..self
        }
    }
    pub(crate) fn with_header_font(self, font: Handle<Font>) -> Self {
        Self {
            text_font: TextFont::from(font).with_font_size(HEADER_FONT_SIZE),
            ..self
        }
    }
}

/// Config for the [`Node`] used by a button.
#[derive(Default)]
pub(crate) struct ButtonNodeConfig {
    pub(crate) width: Val,
    pub(crate) aspect_ratio: Option<f32>,
    pub(crate) border_radius: BorderRadius,
    pub(crate) shadow_offset: Vec2,
}
impl ButtonNodeConfig {
    pub(crate) fn circle_small() -> Self {
        Self {
            width: px(30),
            aspect_ratio: Some(1.),
            border_radius: BorderRadius::MAX,
            shadow_offset: Vec2::new(0., 4.),
        }
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    pub(crate) fn circle_medium() -> Self {
        Self {
            width: px(60),
            aspect_ratio: Some(1.),
            border_radius: BorderRadius::MAX,
            offset: Vec2::new(0., 4.),
        }
    }
    pub(crate) fn round_medium() -> Self {
        Self {
            width: px(60),
            aspect_ratio: Some(2.),
            border_radius: BorderRadius::all(px(30)),
            shadow_offset: Vec2::new(0., 4.),
        }
    }
    pub(crate) fn round_big() -> Self {
        Self {
            width: px(400),
            aspect_ratio: Some(4.5),
            border_radius: BorderRadius::all(px(30)),
            shadow_offset: Vec2::new(0., 6.),
        }
    }
}

/// A builder for creating [`Button`] [`Bundle`]s with customizable appearance, text, and interaction behavior.
#[derive(Default)]
struct ButtonBuilder {
    config: ButtonConfig,
    node_config: ButtonNodeConfig,
    name: &'static str,
    shadow_color: Color,
    background: Color,
    hovered_background: Color,
}
impl ButtonBuilder {
    fn with_button(self) -> Self {
        Self {
            name: "Button",
            shadow_color: BUTTON_SHADOW.into(),
            background: BUTTON_BACKGROUND.into(),
            hovered_background: BUTTON_HOVERED_BACKGROUND.into(),
            ..self
        }
    }
    fn with_switch(self) -> Self {
        Self {
            name: "Switch",
            shadow_color: SWITCH_SHADOW_OFF.into(),
            background: SWITCH_OFF_BACKGROUND.into(),
            hovered_background: SWITCH_OFF_HOVERED_BACKGROUND.into(),
            ..self
        }
    }
    /// Builds a [`Button`] [`Bundle`] and attaches an [`Observer`].
    fn build<E, B, M>(self, action: impl IntoObserverSystem<E, B, M>) -> impl Bundle
    where
        E: EntityEvent,
        B: Bundle,
    {
        let observer = IntoObserverSystem::into_system(action);
        (
            Name::new(format!("{} Container", self.name)),
            ButtonContainer,
            Node::default(),
            Children::spawn(SpawnWith(move |commands: &mut ChildSpawner| {
                let mut button = commands.spawn((
                    Name::new(self.name),
                    Button,
                    BackgroundColor(self.background),
                    InteractionPalette {
                        none: self.background,
                        hovered: self.hovered_background,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    Node {
                        width: self.node_config.width,
                        aspect_ratio: self.node_config.aspect_ratio,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border_radius: self.node_config.border_radius,
                        ..default()
                    },
                    BoxShadow::new(
                        self.shadow_color,
                        Val::ZERO,
                        px(self.node_config.shadow_offset.y),
                        Val::ZERO,
                        Val::ZERO,
                    ),
                    ZIndex(1),
                    children![(
                        Name::new(format!("{} Text", self.name)),
                        ButtonText,
                        Node::default(),
                        Text(self.config.text.to_uppercase()),
                        self.config.text_font,
                        TextColor(BUTTON_TEXT),
                        Pickable::IGNORE,
                        ZIndex(2),
                    )],
                ));
                if self.config.navigable {
                    button.insert((
                        InteractionOverride::default(),
                        AutoDirectionalNavigation::default(),
                        UiNav,
                    ));
                }
                button.observe(observer);
            })),
        )
    }
}

/// A [`Button`] with text and an action defined as an [`Observer`].
pub(crate) fn button<E, B, M>(
    config: ButtonConfig,
    node_config: ButtonNodeConfig,
    action: impl IntoObserverSystem<E, B, M>,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
{
    let builder = ButtonBuilder {
        config,
        node_config,
        ..default()
    };
    builder.with_button().build(action)
}

/// A switch [`Button`] with text and an action defined as an [`Observer`].
pub(crate) fn switch<E, B, M>(
    config: ButtonConfig,
    node_config: ButtonNodeConfig,
    action: impl IntoObserverSystem<E, B, M>,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
{
    let builder = ButtonBuilder {
        config,
        node_config,
        ..default()
    };
    builder.with_switch().build(action)
}
