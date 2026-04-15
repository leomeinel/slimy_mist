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

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::audio::{AnimationAudioIndex, AnimationAudioMap};
    pub(crate) use super::{
        ANIMATION_DELAY_RANGE_SECS, AnimationAction, AnimationBase, AnimationClip, AnimationData,
        AnimationDataCache, AnimationHandle, AnimationOrientation, AnimationRng, AnimationState,
        AnimationTimer, SpriteAnimation, SpriteAnimations,
    };
}

use std::{borrow::Borrow, marker::PhantomData, ops::Range};

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde::Deserialize;

use crate::{
    animations::prelude::*, characters::prelude::*, core::prelude::*, log::prelude::*,
    render::prelude::*, screens::prelude::*, utils::prelude::*,
};

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
                    sprites::update_animation_orientations::<Player>,
                    audio::update_animation_sounds::<Player, PlayerAssets>,
                )
                    .chain(),
                (
                    sprites::update_animations::<Slime>,
                    sprites::update_animation_orientations::<Slime>,
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
                jump::move_sprite.before(PhysicsSet::SyncBackend),
                jump::switch_animation,
            )
                .chain()
                .in_set(AppSystems::Update),
        );
        app.add_systems(
            Update,
            tick_component_timers::<AnimationTimer>.in_set(AppSystems::TickTimers),
        );
    }
}

/// Animation delay [`Range`] in seconds
pub(crate) const ANIMATION_DELAY_RANGE_SECS: Range<f32> = 0.0..10.0;

/// Animation data deserialized from a ron file.
#[derive(Deserialize, Asset, TypePath, Default)]
pub(crate) struct AnimationData<T>
where
    T: Visible,
{
    pub(crate) atlas_columns: usize,
    pub(crate) atlas_rows: usize,
    #[serde(default)]
    pub(crate) idle_clips: [AnimationClip; 3],
    #[serde(default)]
    pub(crate) walk_clips: Option<[AnimationClip; 3]>,
    #[serde(default)]
    pub(crate) run_clips: Option<[AnimationClip; 3]>,
    #[serde(default)]
    pub(crate) jump_clips: Option<[AnimationClip; 3]>,
    #[serde(skip)]
    pub(crate) _phantom: PhantomData<T>,
}

/// Handle for [`AnimationData`].
#[derive(Resource)]
pub(crate) struct AnimationHandle<T>(pub(crate) Handle<AnimationData<T>>)
where
    T: Visible;

/// Cache for [`AnimationData`]
///
/// This is to allow easier access.
#[derive(Resource, Default)]
pub(crate) struct AnimationDataCache<T>
where
    T: Visible,
{
    pub(crate) atlas_columns: usize,
    pub(crate) atlas_rows: usize,
    pub(crate) idle_clips: [AnimationClip; 3],
    pub(crate) walk_clips: Option<[AnimationClip; 3]>,
    // FIXME: We should use fields prefixed with `_`
    pub(crate) _run_clips: Option<[AnimationClip; 3]>,
    pub(crate) jump_clips: Option<[AnimationClip; 3]>,
    pub(crate) _phantom: PhantomData<T>,
}

/// [`Sprite`] animations.
///
/// This stores the [`Sprite`] for the animation and a map of [`AnimationState`] to [`Handle<Animation>`].
#[derive(Resource, Default)]
pub(crate) struct SpriteAnimations<T>
where
    T: Visible,
{
    pub(crate) base: SpriteAnimation,
    pub(crate) floating: Option<SpriteAnimation>,
    // TODO: Think about if this should also affect the collision.
    //       Logically this makes sense, but would add extra complexity and for small
    //       offsets almost seems completely unnecessary.
    pub(crate) y_offset_map: HashMap<AnimationState, Option<f32>>,
    _phantom: PhantomData<T>,
}
impl<T> SpriteAnimations<T>
where
    T: Visible,
{
    pub(crate) fn insert_clips(
        &mut self,
        clips: &[AnimationClip],
        animations: &mut ResMut<Assets<Animation>>,
        base_sheet: &Spritesheet,
        floating_sheet: Option<&Spritesheet>,
        repetitions: AnimationRepeat,
    ) {
        for clip in clips {
            self.base.map.insert(
                clip.state,
                clip.create_animation(animations, base_sheet, repetitions),
            );
            if let Some(ref mut floating) = self.floating
                && let Some(floating_sheet) = floating_sheet
            {
                floating.map.insert(
                    clip.state,
                    clip.create_animation(animations, floating_sheet, repetitions),
                );
            }

            self.y_offset_map.insert(clip.state, clip.y_offset);
        }
    }
}

/// Marker [`Component`] for the animation base.
#[derive(Component)]
pub(crate) struct AnimationBase;

/// Animation for a single [`Sprite`].
#[derive(Default)]
pub(crate) struct SpriteAnimation {
    pub(crate) sprite: Sprite,
    pub(crate) map: HashMap<AnimationState, Handle<Animation>>,
}

/// Animation action.
#[derive(Deserialize, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect, Debug)]
pub(crate) enum AnimationAction {
    #[default]
    Idle,
    Walk,
    Jump,
}

/// Animation orientation in cardinal directions.
#[derive(Deserialize, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect, Debug)]
pub(crate) enum AnimationOrientation {
    #[default]
    South,
    North,
    /// Eastwards orientation.
    ///
    /// This can also be flipped to represent west.
    East,
}
impl AnimationOrientation {
    pub(crate) fn try_from_vec2(vec2: Vec2) -> Option<AnimationOrientation> {
        if vec2.x.abs() > vec2.y.abs() {
            Some(Self::East)
        } else if vec2.y != 0. {
            Some(if vec2.y > 0. {
                Self::North
            } else {
                Self::South
            })
        } else {
            None
        }
    }
}

/// Animation state containing [`AnimationAction`] and [`AnimationOrientation`].
#[derive(Component, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Debug)]
pub(crate) struct AnimationState(pub(crate) (AnimationAction, AnimationOrientation));
impl AnimationState {
    /// Sets a new [`AnimationState`] if it has not already been set.
    pub(crate) fn set_new_action(&mut self, new: AnimationAction) {
        if self.0.0 != new {
            self.0.0 = new;
        }
    }

    pub(crate) fn animation(&self, animation: &SpriteAnimation) -> Handle<Animation> {
        animation
            .map
            .get(&self.0)
            .expect(ERR_NONEXISTENT_ANIMATION)
            .clone()
    }

    pub(crate) fn switch(
        &self,
        animation: &SpriteAnimation,
        sprite_animation: &mut SpritesheetAnimation,
        audio_index: &mut AnimationAudioIndex,
    ) {
        let new_animation = self.animation(animation);
        if sprite_animation.animation != new_animation {
            sprite_animation.switch(new_animation);
            audio_index.0 = None;
        }
    }
}
impl Borrow<(AnimationAction, AnimationOrientation)> for AnimationState {
    fn borrow(&self) -> &(AnimationAction, AnimationOrientation) {
        &self.0
    }
}

/// Deserializable animation clip containing animation data for every [`AnimationState`].
#[derive(Deserialize, Clone, Debug, Default)]
pub(crate) struct AnimationClip {
    pub(crate) state: AnimationState,
    pub(crate) sprite_coords: Vec<(usize, usize)>,
    pub(crate) audio_indexes: Vec<usize>,
    pub(crate) frame_duration_ms: u32,
    pub(crate) y_offset: Option<f32>,
}
impl AnimationClip {
    pub(crate) fn create_animation(
        &self,
        animations: &mut ResMut<Assets<Animation>>,
        sprite_sheet: &Spritesheet,
        repetitions: AnimationRepeat,
    ) -> Handle<Animation> {
        animations.add(
            sprite_sheet
                .create_animation()
                .add_cells(self.sprite_coords.clone())
                .set_clip_duration(AnimationDuration::PerFrame(self.frame_duration_ms))
                .set_repetitions(repetitions)
                .build(),
        )
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
