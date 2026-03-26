/*
 * File: sprites.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{
    animations::prelude::*, characters::prelude::*, images::prelude::*, log::prelude::*,
    render::prelude::*,
};

/// Setup the [`Animations`] struct and add animations
///
/// ## Traits
///
/// - `T` must implement [`Character`] and [`Visible`].
pub(super) fn setup_animations<T>(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
    animation_data: Res<AnimationDataCache<T>>,
    display_image: Res<DisplayImage<T>>,
    images: Res<Assets<Image>>,
) where
    T: Character + Visible,
{
    // Set sprite sheet and generate sprite from it
    let sprite_sheet = Spritesheet::new(
        &display_image.image,
        animation_data.atlas_columns,
        animation_data.atlas_rows,
    );
    let sprite = sprite_sheet
        .with_loaded_image(&images)
        .expect(ERR_NONEXISTENT_IMAGE)
        .sprite(&mut atlas_layouts);
    let mut character_animations = CharacterAnimations::<T> {
        sprite,
        ..default()
    };

    let (idle_clips, walk_clips, jump_clips) = (
        animation_data.idle_clips.as_ref(),
        animation_data.walk_clips.as_ref(),
        animation_data.jump_clips.as_ref(),
    );
    for clip in idle_clips {
        character_animations.map.insert(
            clip.state,
            clip.build_animation(&mut animations, &sprite_sheet, AnimationRepeat::Loop),
        );
    }
    if let Some(walk_clips) = walk_clips {
        for clip in walk_clips {
            character_animations.map.insert(
                clip.state,
                clip.build_animation(&mut animations, &sprite_sheet, AnimationRepeat::Loop),
            );
        }
    }
    if let Some(jump_clips) = jump_clips {
        for clip in jump_clips {
            character_animations.map.insert(
                clip.state,
                clip.build_animation(&mut animations, &sprite_sheet, AnimationRepeat::Times(1)),
            );
        }
    }

    commands.insert_resource(character_animations);
}

/// Update animations
///
/// ## Traits
///
/// - `T` must implement [`Character`].
pub(super) fn update_animations<T>(
    character_query: Query<
        (
            &mut AnimationAudioIndex,
            &mut AnimationState,
            &AnimationTimer,
            &Children,
        ),
        With<T>,
    >,
    mut animation_query: Query<&mut SpritesheetAnimation, With<SpritesheetAnimation>>,
    animations: Res<CharacterAnimations<T>>,
) where
    T: Character,
{
    for (mut audio_index, animation_state, timer, children) in character_query {
        let child = children
            .iter()
            .find(|e| animation_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let mut animation = animation_query.get_mut(child).expect(ERR_INVALID_CHILDREN);

        // Reset animation after timer has finished
        if timer.0.just_finished() {
            animation.reset();
        }

        // Match to current `AnimationState`
        animation_state.switch(&animations, &mut animation, &mut audio_index)
    }
}

/// Update [`AnimationOrientation`]s and flip [`Sprite`]s.
///
/// ## Traits
///
/// - `T` must implement [`Character`].
pub(super) fn update_animation_orientations<T>(
    character_query: Query<(&mut AnimationState, &FacingDirection, &Children), With<T>>,
    mut sprite_query: Query<&mut Sprite, With<SpritesheetAnimation>>,
) where
    T: Character,
{
    for (mut animation_state, direction, children) in character_query {
        let Some(orientation) = AnimationOrientation::try_from_vec2(direction.0) else {
            continue;
        };
        animation_state.0.1 = orientation;

        if animation_state.0.1 == AnimationOrientation::East {
            let child = children
                .iter()
                .find(|e| sprite_query.contains(*e))
                .expect(ERR_INVALID_CHILDREN);
            let mut sprite = sprite_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
            sprite.flip_x = direction.0.x < 0.;
        }
    }
}
