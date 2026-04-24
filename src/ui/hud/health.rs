/*
 * File: health.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;

use crate::{characters::prelude::*, log::prelude::*, ui::prelude::*};

/// Health bar for the [`Hud`].
#[derive(Component)]
pub(super) struct HudHealthBar;

/// Health bar showing the current [`Health`] of the [`Player`].
pub(super) fn health_bar() -> impl Bundle {
    let bar = BarBuilder::round_big_hud()
        .with_bar_background(HEALTH_BAR_BACKGROUND)
        .build();

    (HudHealthBar, NodeRect::default(), Visibility::Hidden, bar)
}

/// Update health bar from [`Player`] [`Health`].
pub(super) fn update_health_bar(
    health: Single<&Health, (Changed<Health>, With<Player>)>,
    bar_container: Single<(&mut Visibility, &Children), With<HudHealthBar>>,
    children_query: Query<&Children>,
    mut node_query: Query<&mut Node>,
) {
    let (mut visibility, children) = bar_container.into_inner();
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
