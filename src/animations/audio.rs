/*
 * File: audio.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_spritesheet_animation::prelude::*;
use rand::seq::IndexedRandom as _;

use crate::{animations::prelude::*, audio::prelude::*, characters::prelude::*, log::prelude::*};

/// Update animation sounds
///
/// ## Traits
///
/// - `T` must implement [`Character`].
/// - `A` must implement [`CharacterAssets`]
pub(super) fn update_animation_sounds<T, A>(
    mut rng: Single<&mut WyRand, With<AnimationRng>>,
    character_query: Query<(&mut AnimationCache, &Children), With<T>>,
    animation_query: Query<&mut SpritesheetAnimation>,
    mut commands: Commands,
    animation_data: Res<AnimationDataCache<T>>,
    assets: Res<A>,
) where
    T: Character,
    A: CharacterAssets,
{
    let frame_set = (
        animation_data.walk_sound_frames.clone(),
        animation_data.jump_sound_frames.clone(),
        animation_data.fall_sound_frames.clone(),
    );

    for (mut cache, children) in character_query {
        let child = children
            .iter()
            .find(|e| animation_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let animation = animation_query.get(child).expect(ERR_INVALID_CHILDREN);

        // Continue if sound has already been played
        let current_frame = animation.progress.frame;
        if let Some(sound_frame) = cache.sound_frame
            && sound_frame == current_frame
        {
            continue;
        }

        // Match to current `AnimationState`
        let Some(sound) = (match cache.state {
            AnimationState::Walk => {
                choose_sound(&mut rng, &current_frame, &frame_set.0, assets.walk_sounds())
            }
            AnimationState::Jump => {
                choose_sound(&mut rng, &current_frame, &frame_set.1, assets.jump_sounds())
            }
            AnimationState::Fall => {
                choose_sound(&mut rng, &current_frame, &frame_set.2, assets.fall_sounds())
            }
            _ => None,
        }) else {
            cache.sound_frame = None;
            continue;
        };

        // Play sound
        commands.spawn(sound_effect(sound));
        cache.sound_frame = Some(current_frame);
    }
}

/// Choose a random sound customized via parameters for current frame.
///
/// Returns [`Some`] if current frame is a fall sound frame.
/// Returns [`None`] if current frame is not a fall sound frame or on missing data.
fn choose_sound(
    rng: &mut WyRand,
    current_frame: &usize,
    sound_frames: &Option<Vec<usize>>,
    sounds: &Option<Vec<Handle<AudioSource>>>,
) -> Option<Handle<AudioSource>> {
    // Return `None` if frame data is missing or does not contain current frame
    let Some(sound_frames) = sound_frames else {
        warn_once!("{}", WARN_INCOMPLETE_ANIMATION_DATA);
        return None;
    };
    if !sound_frames.contains(current_frame) {
        return None;
    }
    // Return `None` if asset data is missing
    let Some(sounds) = sounds else {
        warn_once!("{}", WARN_INCOMPLETE_ASSET_DATA);
        return None;
    };

    sounds.choose(rng).cloned()
}
