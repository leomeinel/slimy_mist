/*
 * File: buttons.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

// FIXME: Simplify this with for example a ButtonConfig struct that is used as parameter.

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::auto_directional_navigation::AutoDirectionalNavigation,
};

use crate::{input::prelude::*, ui::prelude::*};

/// Button base marker
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct ButtonBase;

/// Button base marker
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct ButtonText;

/// A builder for creating [`Button`] [`Bundle`]s with customizable appearance, text, and interaction behavior.
struct ButtonBuilder {
    name: &'static str,
    base_background: Color,
    surface_background: Color,
    hovered_background: Color,
    text: &'static str,
    font: Handle<Font>,
    font_size: f32,
    navigable: bool,
}
impl ButtonBuilder {
    /// Builds a [`Button`] [`Bundle`] and attaches an [`Observer`].
    fn build_with<E, B, M>(
        self,
        base_bundle: impl Bundle,
        surface_bundle: impl Bundle,
        action: impl IntoObserverSystem<E, B, M>,
    ) -> impl Bundle
    where
        E: EntityEvent,
        B: Bundle,
    {
        let observer = IntoObserverSystem::into_system(action);
        (
            Name::new(self.name),
            Node::default(),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                parent
                    .spawn((
                        Name::new(format!("{} Base", self.name)),
                        ButtonBase,
                        BackgroundColor(self.base_background),
                        base_bundle,
                        ZIndex(0),
                    ))
                    .with_children(|base| {
                        let mut children = base.spawn((
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
                                Text(self.text.to_uppercase()),
                                TextFont::from(self.font).with_font_size(self.font_size),
                                TextColor(BUTTON_TEXT),
                                // Don't bubble picking events from the text up to the button.
                                Pickable::IGNORE,
                                ZIndex(2),
                            )],
                        ));
                        if self.navigable {
                            children.insert((
                                InteractionOverride::default(),
                                AutoDirectionalNavigation::default(),
                                UiNav,
                            ));
                        }
                        children.observe(observer);
                    });
            })),
        )
    }
}

/// A rounded [`Button`] with text and an action defined as an [`Observer`].
pub(crate) fn button_rounded<E, B, M>(
    width: Option<Val>,
    text: &'static str,
    font: Handle<Font>,
    navigable: bool,
    action: impl IntoObserverSystem<E, B, M>,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
{
    button(
        text,
        font,
        width.unwrap_or(px(400)),
        Some(4.5),
        BorderRadius::all(px(30)),
        8,
        navigable,
        action,
    )
}

/// A rounded switch [`Button`] with text and an action defined as an [`Observer`].
pub(crate) fn switch_rounded<E, B, M>(
    width: Option<Val>,
    text: &'static str,
    font: Handle<Font>,
    navigable: bool,
    action: impl IntoObserverSystem<E, B, M>,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
{
    switch(
        text,
        font,
        width.unwrap_or(px(60)),
        Some(2.),
        BorderRadius::all(px(30)),
        4,
        navigable,
        action,
    )
}

/// A circle [`Button`] with text and an action defined as an [`Observer`].
pub(crate) fn button_circle<E, B, M>(
    width: Option<Val>,
    text: &'static str,
    font: Handle<Font>,
    navigable: bool,
    action: impl IntoObserverSystem<E, B, M>,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
{
    button(
        text,
        font,
        width.unwrap_or(px(30)),
        Some(1.),
        BorderRadius::MAX,
        4,
        navigable,
        action,
    )
}

/// A [`Button`] with text and an action defined as an [`Observer`].
fn button<E, B, M>(
    text: &'static str,
    font: Handle<Font>,
    width: Val,
    aspect_ratio: Option<f32>,
    border_radius: BorderRadius,
    offset: i32,
    navigable: bool,
    action: impl IntoObserverSystem<E, B, M>,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
{
    let (base_bundle, surface_bundle) = button_bundles(width, aspect_ratio, border_radius, offset);
    let builder = ButtonBuilder {
        name: "Button",
        base_background: BUTTON_BASE_BACKGROUND.into(),
        surface_background: BUTTON_BACKGROUND.into(),
        hovered_background: BUTTON_HOVERED_BACKGROUND.into(),
        text,
        font,
        font_size: HEADER_FONT_SIZE,
        navigable,
    };
    builder.build_with(base_bundle, surface_bundle, action)
}

/// A switch [`Button`] with text and an action defined as an [`Observer`].
fn switch<E, B, M>(
    text: &'static str,
    font: Handle<Font>,
    width: Val,
    aspect_ratio: Option<f32>,
    border_radius: BorderRadius,
    offset: i32,
    navigable: bool,
    action: impl IntoObserverSystem<E, B, M>,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
{
    let (base_bundle, surface_bundle) = button_bundles(width, aspect_ratio, border_radius, offset);
    let builder = ButtonBuilder {
        name: "Switch",
        base_background: SWITCH_BASE_OFF_BACKGROUND.into(),
        surface_background: SWITCH_OFF_BACKGROUND.into(),
        hovered_background: SWITCH_OFF_HOVERED_BACKGROUND.into(),
        text,
        font,
        font_size: BODY_FONT_SIZE,
        navigable,
    };
    builder.build_with(base_bundle, surface_bundle, action)
}

/// Tuples meant to be used as [`Bundle`]s for [`Button`].
fn button_bundles(
    width: Val,
    aspect_ratio: Option<f32>,
    border_radius: BorderRadius,
    offset: i32,
) -> (Node, (Node, NodeOffset)) {
    let common_node = Node {
        width,
        aspect_ratio,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        border_radius,
        ..default()
    };

    (
        Node {
            overflow: Overflow::visible(),
            ..common_node.clone()
        },
        (
            Node {
                bottom: px(offset),
                position_type: PositionType::Absolute,
                ..common_node
            },
            NodeOffset(IVec2::new(0, offset)),
        ),
    )
}
