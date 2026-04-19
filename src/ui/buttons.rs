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

/// Button base marker
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct ButtonBase;

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
    pub(crate) offset: NodeOffset,
}
impl ButtonNodeConfig {
    pub(crate) fn circle_small() -> Self {
        Self {
            width: px(30),
            aspect_ratio: Some(1.),
            border_radius: BorderRadius::MAX,
            offset: NodeOffset(Vec2::new(0., 4.)),
        }
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    pub(crate) fn circle_medium() -> Self {
        Self {
            width: px(60),
            aspect_ratio: Some(1.),
            border_radius: BorderRadius::MAX,
            offset: NodeOffset(Vec2::new(0., 4.)),
        }
    }
    pub(crate) fn round_medium() -> Self {
        Self {
            width: px(60),
            aspect_ratio: Some(2.),
            border_radius: BorderRadius::all(px(30)),
            offset: NodeOffset(Vec2::new(0., 4.)),
        }
    }
    pub(crate) fn round_big() -> Self {
        Self {
            width: px(400),
            aspect_ratio: Some(4.5),
            border_radius: BorderRadius::all(px(30)),
            offset: NodeOffset(Vec2::new(0., 6.)),
        }
    }
    fn into_bundles(self) -> (Node, (Node, NodeOffset)) {
        let base_node = Node {
            width: self.width,
            aspect_ratio: self.aspect_ratio,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: self.border_radius,
            ..default()
        };
        (
            Node {
                ..base_node.clone()
            },
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: px(self.offset.0.y),
                    ..base_node.clone()
                },
                self.offset,
            ),
        )
    }
}

// FIXME: Currently there is a very small area at the top that is hoverable but will not trigger the observer when clicked.
/// A builder for creating [`Button`] [`Bundle`]s with customizable appearance, text, and interaction behavior.
#[derive(Default)]
struct ButtonBuilder {
    config: ButtonConfig,
    node_config: ButtonNodeConfig,
    name: &'static str,
    base_background: Color,
    surface_background: Color,
    hovered_background: Color,
}
impl ButtonBuilder {
    fn with_button(self) -> Self {
        Self {
            name: "Button",
            base_background: BUTTON_BASE_BACKGROUND.into(),
            surface_background: BUTTON_BACKGROUND.into(),
            hovered_background: BUTTON_HOVERED_BACKGROUND.into(),
            ..self
        }
    }
    fn with_switch(self) -> Self {
        Self {
            name: "Switch",
            base_background: SWITCH_BASE_OFF_BACKGROUND.into(),
            surface_background: SWITCH_OFF_BACKGROUND.into(),
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
        let (base_bundle, surface_bundle) = self.node_config.into_bundles();
        let observer = IntoObserverSystem::into_system(action);
        (
            Name::new(self.name),
            ButtonContainer,
            Node {
                padding: UiRect::top(surface_bundle.0.bottom),
                ..default()
            },
            Children::spawn(SpawnWith(move |commands: &mut ChildSpawner| {
                commands.spawn((
                    Name::new(format!("{} Base", self.name)),
                    ButtonBase,
                    BackgroundColor(self.base_background),
                    base_bundle,
                    ZIndex(0),
                ));
                let mut surface = commands.spawn((
                    Name::new(format!("{} Surface", self.name)),
                    Button,
                    BackgroundColor(self.surface_background),
                    InteractionPalette {
                        none: self.surface_background,
                        hovered: self.hovered_background,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    surface_bundle,
                    ZIndex(1),
                    children![(
                        Name::new(format!("{} Text", self.name)),
                        ButtonText,
                        Text(self.config.text.to_uppercase()),
                        self.config.text_font,
                        TextColor(BUTTON_TEXT),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                        ZIndex(2),
                    )],
                ));
                if self.config.navigable {
                    surface.insert((
                        InteractionOverride::default(),
                        AutoDirectionalNavigation::default(),
                        UiNav,
                    ));
                }
                surface.observe(observer);
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
