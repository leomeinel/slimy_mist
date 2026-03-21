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
    mut global_animations: ResMut<Assets<Animation>>,
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

    // Idle animation: This is the only required animation
    let idle = animation_handle(
        &mut global_animations,
        &sprite_sheet,
        animation_data.idle_frames.as_ref(),
        animation_data.idle_interval_ms,
        AnimationRepeat::Loop,
    )
    .expect(ERR_INVALID_IDLE_ANIMATION_DATA);

    // Walk animation
    let walk = animation_handle(
        &mut global_animations,
        &sprite_sheet,
        animation_data.walk_frames.as_ref(),
        animation_data.walk_interval_ms,
        AnimationRepeat::Loop,
    );

    // Jump animation
    let jump = animation_data
        .jump_frames
        .as_ref()
        .map(|frames| {
            let interval_ms = (JUMP_DURATION_SECS * 500. / frames.len().max(1) as f32) as u32;
            animation_handle(
                &mut global_animations,
                &sprite_sheet,
                animation_data.jump_frames.as_ref(),
                Some(interval_ms),
                AnimationRepeat::Times(1),
            )
        })
        .unwrap_or_else(|| None);

    let sprite_layout_id = sprite
        .texture_atlas
        .as_ref()
        .expect(ERR_INVALID_TEXTURE_ATLAS)
        .layout
        .id();
    let texture_size = atlas_layouts
        .get(sprite_layout_id)
        .expect(ERR_INVALID_TEXTURE_ATLAS)
        .textures
        .first()
        .expect(ERR_INVALID_TEXTURE_ATLAS)
        .size();

    commands.insert_resource(ImageMeta::<T> {
        size: texture_size,
        ..default()
    });
    commands.insert_resource(Animations::<T> {
        sprite,
        idle,
        walk,
        jump,
        ..default()
    });
}

/// Animation handle customized via parameters
///
/// Returns [`Some`] for valid parameters
/// Returns [`None`] for invalid `row`, `frames` or `interval` parameter
fn animation_handle(
    global_animations: &mut ResMut<Assets<Animation>>,
    sprite_sheet: &Spritesheet,
    frames: Option<&Vec<(usize, usize)>>,
    interval: Option<u32>,
    repetitions: AnimationRepeat,
) -> Option<Handle<Animation>> {
    let (Some(frames), Some(interval)) = (frames, interval) else {
        warn_once!("{}", WARN_INCOMPLETE_ANIMATION_DATA);
        return None;
    };
    if frames.is_empty() {
        return None;
    }

    Some(
        global_animations.add(
            sprite_sheet
                .create_animation()
                .add_cells(frames.clone())
                .set_clip_duration(AnimationDuration::PerFrame(interval))
                .set_repetitions(repetitions)
                .build(),
        ),
    )
}

/// Update animations
///
/// ## Traits
///
/// - `T` must implement [`Character`].
pub(super) fn update_animations<T>(
    character_query: Query<(&mut AnimationCache, &AnimationTimer, &Children), With<T>>,
    mut animation_query: Query<&mut SpritesheetAnimation, With<SpritesheetAnimation>>,
    animations: Res<Animations<T>>,
) where
    T: Character,
{
    for (mut cache, timer, children) in character_query {
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
        match cache.state {
            AnimationState::Walk => {
                switch_to_new_animation(&mut animation, animations.walk.clone(), &mut cache)
            }
            AnimationState::Idle => {
                switch_to_new_animation(&mut animation, Some(animations.idle.clone()), &mut cache)
            }
            AnimationState::Jump => {
                switch_to_new_animation(&mut animation, animations.jump.clone(), &mut cache)
            }
        }
    }
}

/// Switches to new [`SpritesheetAnimation`] from [`Option<Handle<Animation>>`] if it has not already been switched to.
fn switch_to_new_animation(
    animation: &mut SpritesheetAnimation,
    new_animation: Option<Handle<Animation>>,
    cache: &mut AnimationCache,
) {
    let new_animation = new_animation.expect(ERR_NONEXISTENT_ANIMATION);

    if animation.animation != new_animation {
        animation.switch(new_animation);
        cache.sound_frame = None;
    }
}
