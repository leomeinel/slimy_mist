/*
 * File: scroll.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/bevyengine/bevy/blob/main/examples/ui/scroll_and_overflow/scroll.rs
 */

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::UiSystems,
};

use crate::{core::prelude::*, input::prelude::*};

pub(super) struct UiInputScrollPlugin;
impl Plugin for UiInputScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                input_scroll_directional_nav,
                input_scroll_mouse_wheel,
                input_scroll_touch,
            )
                .in_set(AppSystems::RecordInput),
        );
        // NOTE: We are running this in `FixedUpdate` to ensure consistent auto scrolling.
        app.add_systems(FixedUpdate, auto_scroll.after(AppSystems::RecordInput));
        app.add_systems(
            PostUpdate,
            reset_scroll_node_layout.after(UiSystems::Layout),
        );

        app.add_observer(on_scroll_action);
        app.add_observer(on_scroll);
    }
}

/// UI scrolling action.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub(super) struct ScrollAction {
    entity: Entity,
    /// Scroll delta in logical coordinates.
    delta: Vec2,
}

/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub(super) struct Scroll {
    entity: Entity,
    /// Scroll delta in logical coordinates.
    delta: Vec2,
}

/// Marker [`Component`] for auto scrolling.
#[derive(Component)]
pub(crate) struct AutoScroll(pub(crate) Vec2);

/// Marker [`Component`] for input scrolling.
#[derive(Component)]
pub(crate) struct InputScroll(pub(crate) Vec2);

/// Trigger [`Scroll`] for [`Entity`] with [`InputScroll`] from [`UiNavActionSet`].
///
/// - This assumes that only a single [`InputScroll`] is present.
pub(super) fn input_scroll_directional_nav(
    scroll: Single<(Entity, &Node, &InputScroll)>,
    mut commands: Commands,
    action_set: Res<UiNavActionSet>,
) {
    let (entity, node, scroll) = *scroll;
    let Some(delta) = action_set.direction().map(|d| -d.as_vec2() * scroll.0) else {
        return;
    };
    if (node.align_items == AlignItems::Center || delta.x == 0.)
        && (node.justify_content == JustifyContent::Center || delta.y == 0.)
    {
        return;
    }

    commands.trigger(ScrollAction { entity, delta });
}

/// Trigger [`Scroll`] for [`Entity`] with [`InputScroll`] from [`Touches`].
///
/// - This assumes that only a single [`InputScroll`] is present.
pub(super) fn input_scroll_touch(
    scroll: Single<(Entity, &Node, &InputScroll)>,
    mut commands: Commands,
    touches: Res<Touches>,
) {
    for touch in touches.iter() {
        let (entity, node, scroll) = *scroll;
        if ((node.align_items == AlignItems::Center || scroll.0.x == 0.)
            && (node.justify_content == JustifyContent::Center || scroll.0.y == 0.))
            || !touch.is_vertical_swipe()
        {
            continue;
        }
        let delta = -touch.delta();

        commands.trigger(ScrollAction { entity, delta });
    }
}

/// Trigger [`Scroll`] for [`Entity`] with [`InputScroll`] from [`MouseWheel`].
///
/// - This assumes that only a single [`InputScroll`] is present.
pub(super) fn input_scroll_mouse_wheel(
    mut reader: MessageReader<MouseWheel>,
    scroll: Single<(Entity, &Node, &InputScroll)>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel in reader.read() {
        let (entity, node, scroll) = *scroll;
        if (node.align_items == AlignItems::Center || scroll.0.x == 0.)
            && (node.justify_content == JustifyContent::Center || scroll.0.y == 0.)
        {
            continue;
        }

        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= scroll.0;
        }
        if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        commands.trigger(ScrollAction { entity, delta });
    }
}

/// Trigger [`Scroll`] automatically for [`Entity`]s with [`AutoScroll`].
pub(super) fn auto_scroll(query: Query<(Entity, &Node, &AutoScroll)>, mut commands: Commands) {
    for (entity, node, scroll) in query {
        let delta = scroll.0;
        if (node.align_items == AlignItems::Center || delta.x == 0.)
            && (node.justify_content == JustifyContent::Center || delta.y == 0.)
        {
            continue;
        }
        commands.trigger(Scroll { entity, delta });
    }
}

/// Reset layout for scrolling [`Node`]s.
///
/// This sets [`Node::justify_content`] and [`Node::align_items`] according to if there is any overflow.
pub(super) fn reset_scroll_node_layout(
    query: Query<(&mut Node, &ComputedNode), Or<(With<AutoScroll>, With<InputScroll>)>>,
) {
    for (mut node, computed) in query {
        let delta = computed.content_size() - computed.size();

        if node.overflow.x == OverflowAxis::Scroll {
            if delta.x > 0. {
                node.align_items = AlignItems::FlexStart;
            } else {
                node.align_items = AlignItems::Center;
            }
        }

        if node.overflow.y == OverflowAxis::Scroll {
            if delta.y > 0. {
                node.justify_content = JustifyContent::FlexStart;
            } else {
                node.justify_content = JustifyContent::Center;
            }
        }
    }
}

/// On a triggered [`ScrollAction`] trigger [`Scroll`].
///
/// This also overrides [`AutoScroll`].
pub(super) fn on_scroll_action(event: On<ScrollAction>, mut commands: Commands) {
    let entity = event.entity;
    commands.entity(entity).try_remove::<AutoScroll>();
    commands.trigger(Scroll {
        entity,
        delta: event.delta,
    });
}

/// On a triggered [`Scroll`], scroll associated [`Node`].
pub(super) fn on_scroll(
    mut event: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(event.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor;

    let delta = &mut event.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += delta.x;
            // Consume the X portion of the scroll delta.
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            // Consume the Y portion of the scroll delta.
            delta.y = 0.;
        }
    }

    // Stop propagating when the delta is fully consumed.
    if *delta == Vec2::ZERO {
        event.propagate(false);
    }
}
