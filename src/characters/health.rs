/*
 * File: health.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;

/// Health that determines if a [`Component`] should be despawned.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Health {
    pub(crate) max: f32,
    pub(crate) current: f32,
}
impl Health {
    pub(crate) fn new(max: f32) -> Self {
        Self { max, current: max }
    }
    pub(crate) fn fraction(&self) -> f32 {
        if self.max > 0. {
            (self.current / self.max).clamp(0., 1.)
        } else {
            0.
        }
    }
    pub(crate) fn is_alive(&self) -> bool {
        self.current > 0.
    }
}

/// Apply damage to [`Health`].
#[derive(Event)]
pub(crate) struct Damage {
    pub(crate) targets: Vec<Entity>,
    pub(crate) damage: f32,
}

/// Apply [`Damage`] to [`Health`] and handle despawning.
pub(super) fn on_damage(
    event: On<Damage>,
    mut target_query: Query<&mut Health>,
    mut commands: Commands,
) {
    for entity in &event.targets {
        let Ok(mut health) = target_query.get_mut(*entity) else {
            continue;
        };
        health.current -= event.damage;
        if !health.is_alive() {
            commands.entity(*entity).despawn();
        }
    }
}
