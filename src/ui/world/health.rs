/*
 * File: health.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

// FIXME: This seems like a very hacky approach, but seems to be somewhat common practice.
//        Generally however it is recommended to use textures to achieve this, but this approach
//        currently provides the most consistency.

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{characters::prelude::*, log::prelude::*, render::prelude::*, ui::prelude::*};

/// [`Node::width`] for [`WorldUiHealthBar`].
pub(crate) const WORLD_UI_HEALTH_BAR_WIDTH: Val = Val::Px(42.);
/// [`Node::height`] for [`WorldUiHealthBar`].
pub(crate) const WORLD_UI_HEALTH_BAR_HEIGHT: Val = Val::Px(12.);
/// [`Node::padding`] [`Val`] for [`WorldUiHealthBar`].
pub(crate) const WORLD_UI_HEALTH_BAR_PADDING: Val = Val::Px(3.);

/// World [`Entity`]s mapped to their corresponding [`WorldUiHealthBar`] [`Entity`]s.
#[derive(Resource, Default)]
pub(crate) struct WorldUiHealthBarMap(pub(crate) HashMap<Entity, Entity>);

/// Marker [`Component`] for any health bar rendered to [`WorldUi`].
#[derive(Component)]
pub(super) struct WorldUiHealthBar;

/// Spawn [`WorldUiHealthBar`] for all added `T` and add to [`WorldUiHealthBarMap`].
pub(super) fn spawn_health_bar<T>(
    container: Single<Entity, With<WorldUi>>,
    visible_query: Query<Entity, Added<T>>,
    mut commands: Commands,
    mut map: ResMut<WorldUiHealthBarMap>,
) where
    T: Visible,
{
    for entity in visible_query {
        let bar = commands.spawn(health_bar()).id();
        commands.entity(*container).add_child(bar);
        map.0.insert(entity, bar);
    }
}

/// Health bar showing the current [`Health`].
fn health_bar() -> impl Bundle {
    let bar = BarBuilder::health_medium_world()
        .with_position_type(PositionType::Absolute)
        .with_bar_background(HEALTH_BAR_BACKGROUND)
        .build();

    (WorldUiHealthBar, Visibility::Hidden, bar)
}

/// Despawn [`WorldUiHealthBar`] for all removed `T` and remove from [`WorldUiHealthBarMap`].
pub(super) fn despawn_health_bar<T>(
    mut removed_visible: RemovedComponents<T>,
    mut commands: Commands,
    mut map: ResMut<WorldUiHealthBarMap>,
) where
    T: Visible,
{
    for entity in removed_visible.read() {
        let Some(entity) = map.0.remove(&entity) else {
            continue;
        };
        // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
        commands.entity(entity).try_despawn();
    }
}

/// Update [`WorldUiHealthBar`] from [`Health`] of all `T`.
pub(super) fn update_health_bar<T>(
    mut node_query: Query<&mut Node>,
    mut bar_container_query: Query<(&mut Visibility, &Children), With<WorldUiHealthBar>>,
    health_query: Query<(Entity, &Health), (Changed<Health>, With<T>)>,
    children_query: Query<&Children>,
    map: Res<WorldUiHealthBarMap>,
) where
    T: Visible,
{
    for (entity, health) in health_query {
        let Some(entity) = map.0.get(&entity) else {
            continue;
        };
        let Ok((mut visibility, children)) = bar_container_query.get_mut(*entity) else {
            continue;
        };
        let child = children
            .iter()
            .find(|e| children_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let children = children_query.get(child).expect(ERR_INVALID_CHILDREN);
        let child = children
            .iter()
            .find(|e| node_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let mut mask_node = node_query.get_mut(child).expect(ERR_INVALID_CHILDREN);

        let mask_percent = (1. - health.fraction()) * 100.;
        *visibility = if mask_percent > 0. {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        mask_node.width = percent(mask_percent);
    }
}

/// Move and scale [`WorldUiHealthBar`] for all `T`.
///
/// This moves [`WorldUiHealthBar`] by moving the [`Node`] according to the world position.
///
/// Additionally this scales [`Node::width`], [`Node::height`] and [`Node::padding`] to not have [`Node`]s in [`WorldUi`] affected by [`UiScale`].
pub(super) fn move_and_scale_health_bar<T>(
    camera: Single<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    mut bar_node_query: Query<(&mut Node, &ComputedNode, &Visibility), With<WorldUiHealthBar>>,
    transform_query: Query<(Entity, &GlobalTransform, &WorldUiAnchor), With<T>>,
    map: Res<WorldUiHealthBarMap>,
    ui_scale: Res<UiScale>,
) where
    T: Visible,
{
    let (camera, camera_transform) = *camera;

    for (entity, transform, offset) in transform_query {
        let Some(entity) = map.0.get(&entity) else {
            continue;
        };
        let Ok((mut bar_node, computed, visibility)) = bar_node_query.get_mut(*entity) else {
            continue;
        };
        if visibility == Visibility::Hidden {
            continue;
        }
        let WorldUiAnchor::HealthBar(offset) = offset;
        let world_position = transform.translation().xy() + offset;
        let Ok(ui_position) = camera.world_to_viewport(camera_transform, world_position.extend(0.))
        else {
            continue;
        };

        let x_offset = -(computed.size.x * computed.inverse_scale_factor) / 2.;
        bar_node.width = WORLD_UI_HEALTH_BAR_WIDTH / ui_scale.0;
        bar_node.height = WORLD_UI_HEALTH_BAR_HEIGHT / ui_scale.0;
        bar_node.padding = UiRect::all(WORLD_UI_HEALTH_BAR_PADDING / ui_scale.0);
        bar_node.left = px(ui_position.x / ui_scale.0 + x_offset);
        bar_node.top = px(ui_position.y / ui_scale.0);
    }
}
