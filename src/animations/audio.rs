use std::marker::PhantomData;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_prng::WyRand;
use bevy_spritesheet_animation::prelude::*;
use rand::seq::IndexedRandom as _;

use crate::{
    animations::prelude::*, audio::prelude::*, characters::prelude::*, log::prelude::*,
    render::prelude::*,
};

/// Animation audio index that indicates the current/last played audio frame.
#[derive(Component, Default)]
pub(crate) struct AnimationAudioIndex(pub(crate) Option<usize>);

/// Animation audio map.
///
/// This stores a map of [`AnimationState`] to audio indexes.
#[derive(Resource, Default)]
pub(crate) struct AnimationAudioMap<T>
where
    T: Visible,
{
    pub(crate) map: HashMap<AnimationState, Vec<usize>>,
    pub(crate) _phantom: PhantomData<T>,
}

/// Update animation sounds
pub(super) fn update_animation_sounds<T, A>(
    mut rng: Single<&mut WyRand, With<AnimationRng>>,
    character_query: Query<(&mut AnimationAudioIndex, &AnimationState, &Children), With<T>>,
    base_query: Query<&mut SpritesheetAnimation, With<AnimationBase>>,
    mut commands: Commands,
    audio_map: Res<AnimationAudioMap<T>>,
    assets: Res<A>,
) where
    T: Visible,
    A: CharacterAssets,
{
    for (mut audio_index, state, children) in character_query {
        let child = children
            .iter()
            .find(|e| base_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let animation = base_query.get(child).expect(ERR_INVALID_CHILDREN);

        // Continue if sound has already been played
        let current_frame = animation.progress.frame;
        if let Some(sound_frame) = audio_index.0
            && sound_frame == current_frame
        {
            continue;
        }

        let Some(sound) = choose_sound(
            &mut rng,
            &current_frame,
            state,
            &audio_map.map,
            assets.sounds(state.0.0),
        ) else {
            audio_index.0 = None;
            continue;
        };

        // Play sound
        commands.spawn(sound_effect(sound));
        audio_index.0 = Some(current_frame);
    }
}

/// Choose a random sound customized via parameters for current frame.
///
/// Returns [`Some`] if current frame is a sound frame.
/// Returns [`None`] if current frame is not a sound frame or on missing data.
fn choose_sound(
    rng: &mut WyRand,
    current_frame: &usize,
    state: &AnimationState,
    audio_map: &HashMap<AnimationState, Vec<usize>>,
    sounds: &Option<Vec<Handle<AudioSource>>>,
) -> Option<Handle<AudioSource>> {
    let Some(audio_indexes) = audio_map.get(state) else {
        warn_once!("{}", WARN_INCOMPLETE_ANIMATION_DATA);
        return None;
    };
    if !audio_indexes.contains(current_frame) {
        return None;
    }
    let Some(sounds) = sounds else {
        warn_once!("{}", WARN_INCOMPLETE_ASSET_DATA);
        return None;
    };

    sounds.choose(rng).cloned()
}
