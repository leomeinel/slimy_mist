/*
 * File: animations.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by:
 * - https://github.com/NiklasEi/bevy_common_assets/tree/main
 * - https://github.com/merwaaan/bevy_spritesheet_animation
 */

//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

mod audio;
mod jump;
mod sprites;

pub(crate) mod prelude {
    pub(crate) use super::{
        ANIMATION_DELAY_RANGE_SECS, AnimationCache, AnimationData, AnimationDataCache,
        AnimationHandle, AnimationRng, AnimationState, AnimationTimer, Animations,
    };
}

use std::{marker::PhantomData, ops::Range};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{characters::prelude::*, core::prelude::*, screens::prelude::*, utils::prelude::*};

pub(super) struct AnimationsPlugin;
impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpritesheetAnimationPlugin);

        app.add_systems(Startup, setup_rng::<AnimationRng>);
        app.add_systems(
            OnEnter(Screen::Gameplay),
            (
                sprites::setup_animations::<Player>,
                sprites::setup_animations::<Slime>,
            )
                .in_set(EnterGameplaySystems::Animations),
        );
        app.add_systems(
            Update,
            (
                (
                    sprites::update_animations::<Player>,
                    audio::update_animation_sounds::<Player, PlayerAssets>,
                )
                    .chain(),
                (
                    sprites::update_animations::<Slime>,
                    audio::update_animation_sounds::<Slime, SlimeAssets>,
                )
                    .chain(),
            )
                .run_if(in_state(Screen::Gameplay))
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        );
        app.add_systems(
            Update,
            (
                jump::apply_jump.before(PhysicsSet::SyncBackend),
                jump::limit_jump,
            )
                .chain()
                .in_set(AppSystems::Update),
        );
        app.add_systems(
            Update,
            tick_component_timer::<AnimationTimer>.in_set(AppSystems::TickTimers),
        );
    }
}

/// Animation delay [`Range`] in seconds
pub(crate) const ANIMATION_DELAY_RANGE_SECS: Range<f32> = 0.0..10.0;

/// Animation data deserialized from a ron file as a generic.
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(serde::Deserialize, Asset, TypePath, Default)]
pub(crate) struct AnimationData<T>
where
    T: Character,
{
    pub(crate) atlas_columns: usize,
    pub(crate) atlas_rows: usize,
    #[serde(default)]
    pub(crate) idle_frames: Option<Vec<(usize, usize)>>,
    #[serde(default)]
    pub(crate) idle_interval_ms: Option<u32>,
    #[serde(default)]
    pub(crate) walk_frames: Option<Vec<(usize, usize)>>,
    #[serde(default)]
    pub(crate) walk_interval_ms: Option<u32>,
    #[serde(default)]
    pub(crate) walk_sound_frames: Option<Vec<usize>>,
    #[serde(default)]
    pub(crate) run_frames: Option<Vec<(usize, usize)>>,
    #[serde(default)]
    pub(crate) run_interval_ms: Option<u32>,
    #[serde(default)]
    pub(crate) run_sound_frames: Option<Vec<usize>>,
    #[serde(default)]
    pub(crate) jump_frames: Option<Vec<(usize, usize)>>,
    #[serde(default)]
    pub(crate) jump_sound_frames: Option<Vec<usize>>,
    #[serde(default)]
    pub(crate) fall_frames: Option<Vec<(usize, usize)>>,
    #[serde(default)]
    pub(crate) fall_sound_frames: Option<Vec<usize>>,
    #[serde(skip)]
    pub(crate) _phantom: PhantomData<T>,
}

/// Handle for [`AnimationData`] as a generic
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource)]
pub(crate) struct AnimationHandle<T>(pub(crate) Handle<AnimationData<T>>)
where
    T: Character;

/// Cache for [`AnimationData`]
///
/// This is to allow easier access.
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct AnimationDataCache<T>
where
    T: Character,
{
    pub(crate) atlas_columns: usize,
    pub(crate) atlas_rows: usize,
    pub(crate) idle_frames: Option<Vec<(usize, usize)>>,
    pub(crate) idle_interval_ms: Option<u32>,
    pub(crate) walk_frames: Option<Vec<(usize, usize)>>,
    pub(crate) walk_interval_ms: Option<u32>,
    pub(crate) walk_sound_frames: Option<Vec<usize>>,
    // FIXME: We should use fields prefixed with `_`
    pub(crate) _run_frames: Option<Vec<(usize, usize)>>,
    pub(crate) _run_interval_ms: Option<u32>,
    pub(crate) _run_sound_frames: Option<Vec<usize>>,
    pub(crate) jump_frames: Option<Vec<(usize, usize)>>,
    pub(crate) jump_sound_frames: Option<Vec<usize>>,
    pub(crate) fall_frames: Option<Vec<(usize, usize)>>,
    pub(crate) fall_sound_frames: Option<Vec<usize>>,
    pub(crate) _phantom: PhantomData<T>,
}

/// Animations with generics
///
/// This serves as the main interface for other modules
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct Animations<T>
where
    T: Character,
{
    pub(crate) sprite: Sprite,
    pub(crate) idle: Handle<Animation>,
    pub(crate) walk: Option<Handle<Animation>>,
    pub(crate) jump: Option<Handle<Animation>>,
    pub(crate) fall: Option<Handle<Animation>>,
    _phantom: PhantomData<T>,
}

/// Current state of animation
#[derive(Default, Clone, Copy, PartialEq, Reflect, Debug)]
pub(crate) enum AnimationState {
    #[default]
    Idle,
    Walk,
    Jump,
    Fall,
}

/// Cache for animations
#[derive(Component, Default)]
pub(crate) struct AnimationCache {
    /// Used to determine next animation
    pub(crate) state: AnimationState,
    /// Used to determine if we should play sound again
    pub(crate) sound_frame: Option<usize>,
}
impl AnimationCache {
    /// Sets a new [`AnimationState`] if it has not already been set.
    pub(crate) fn set_new_state(&mut self, new: AnimationState) {
        if self.state != new {
            self.state = new;
        }
    }
}

/// Timer that tracks animation
#[derive(Component, Debug, Clone, PartialEq, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub(crate) struct AnimationTimer(pub(crate) Timer);

/// Rng for animations
#[derive(Component, Default)]
pub(crate) struct AnimationRng;
impl ForkedRng for AnimationRng {}
