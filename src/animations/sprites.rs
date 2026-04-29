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

/// Setup [`SpriteAnimations`] and add animations.
pub(super) fn setup_animations<T>(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
    animation_data: Res<AnimationDataCache<T>>,
    layers: Res<DisplayLayers<T>>,
    images: Res<Assets<Image>>,
) where
    T: Visible,
{
    let base_sheet = Spritesheet::new(
        &layers.base,
        animation_data.atlas_columns,
        animation_data.atlas_rows,
    );
    let floating_sheet = layers
        .floating
        .as_ref()
        .map(|i| Spritesheet::new(i, animation_data.atlas_columns, animation_data.atlas_rows));
    let floating_sheet = floating_sheet.as_ref();
    let mut sprite_animations = SpriteAnimations::<T> {
        base: SpriteAnimation {
            sprite: base_sheet
                .with_loaded_image(&images)
                .expect(ERR_NONEXISTENT_IMAGE)
                .sprite(&mut atlas_layouts),
            ..default()
        },
        floating: floating_sheet.map(|s| SpriteAnimation {
            sprite: s
                .with_loaded_image(&images)
                .expect(ERR_NONEXISTENT_IMAGE)
                .sprite(&mut atlas_layouts),
            ..default()
        }),
        ..default()
    };

    let (idle_clips, walk_clips, jump_clips) = (
        animation_data.idle_clips.as_ref(),
        animation_data.walk_clips.as_ref(),
        animation_data.jump_clips.as_ref(),
    );
    // NOTE: This asserts that each direction of the clip is the same length.
    assert!(
        idle_clips
            .windows(2)
            .all(|c| { c[0].sprite_coords.len() == c[1].sprite_coords.len() })
    );
    sprite_animations.insert_clips(
        idle_clips,
        &mut animations,
        &base_sheet,
        floating_sheet,
        AnimationRepeat::Loop,
    );
    if let Some(walk_clips) = walk_clips {
        // NOTE: This asserts that each direction of the clip is the same length.
        assert!(
            walk_clips
                .windows(2)
                .all(|c| { c[0].sprite_coords.len() == c[1].sprite_coords.len() })
        );
        sprite_animations.insert_clips(
            walk_clips,
            &mut animations,
            &base_sheet,
            floating_sheet,
            AnimationRepeat::Loop,
        );
    }
    if let Some(jump_clips) = jump_clips {
        // NOTE: This asserts that each direction of the clip is the same length.
        assert!(
            jump_clips
                .windows(2)
                .all(|c| { c[0].sprite_coords.len() == c[1].sprite_coords.len() })
        );
        sprite_animations.insert_clips(
            jump_clips,
            &mut animations,
            &base_sheet,
            floating_sheet,
            AnimationRepeat::Times(1),
        );
    }

    commands.insert_resource(sprite_animations);
}

/// Update animations.
pub(super) fn update_animations<T>(
    container_query: Query<
        (
            &mut LastAnimationAction,
            &mut AnimationAudioIndex,
            &mut AnimationYOffset,
            &AnimationState,
            Option<&AnimationTimer>,
            &Children,
        ),
        With<T>,
    >,
    mut base_query: Query<
        (&mut SpritesheetAnimation, &mut Transform, Option<&Children>),
        With<AnimationBase>,
    >,
    mut floating_query: Query<&mut SpritesheetAnimation, Without<AnimationBase>>,
    sprite_animations: Res<SpriteAnimations<T>>,
) where
    T: Visible,
{
    for (mut last_action, mut audio_index, mut y_offset, state, timer, children) in container_query
    {
        let children: Vec<_> = children.iter().collect();
        let child_entity = children
            .iter()
            .find(|e| base_query.contains(**e))
            .expect(ERR_INVALID_CHILDREN);
        let (mut base_animation, mut transform, children) = base_query
            .get_mut(*child_entity)
            .expect(ERR_INVALID_CHILDREN);
        if timer.is_some_and(|t| t.0.just_finished()) {
            base_animation.reset();
        }

        state.switch(
            &sprite_animations.base,
            &mut base_animation,
            &mut last_action.base,
            &mut audio_index,
        );
        if let Some(next_y_offset) = sprite_animations
            .y_offset_map
            .get(state)
            .and_then(|o| o.as_ref())
        {
            transform.translation.y += *next_y_offset - y_offset.0;
            y_offset.0 = *next_y_offset;
        }

        if let Some(children) = children
            && let Some(entity) = children.iter().find(|e| floating_query.contains(*e))
            && let Ok(mut floating_animation) = floating_query.get_mut(entity)
            && let Some(animation) = &&sprite_animations.floating
        {
            if timer.is_some_and(|t| t.0.just_finished()) {
                floating_animation.reset();
            }
            state.switch(
                animation,
                &mut floating_animation,
                &mut last_action.floating,
                &mut audio_index,
            );
        }
    }
}

/// Update [`AnimationOrientation`]s and flip [`Sprite`]s.
pub(super) fn update_animation_orientations<T>(
    container_query: Query<(&mut AnimationState, &FacingDirection, &Children), With<T>>,
    mut base_query: Query<(&mut Sprite, Option<&Children>), With<AnimationBase>>,
    mut floating_query: Query<&mut Sprite, Without<AnimationBase>>,
) where
    T: Visible,
{
    for (mut state, direction, children) in container_query {
        let Some(orientation) = AnimationOrientation::try_from_vec2(direction.0) else {
            continue;
        };
        state.0.1 = orientation;

        if state.0.1 == AnimationOrientation::East {
            let child = children
                .iter()
                .find(|e| base_query.contains(*e))
                .expect(ERR_INVALID_CHILDREN);
            let (mut base_sprite, children) =
                base_query.get_mut(child).expect(ERR_INVALID_CHILDREN);
            base_sprite.flip_x = direction.0.x < 0.;

            if let Some(children) = children
                && let Some(child) = children.iter().find(|e| floating_query.contains(*e))
                && let Ok(mut floating_sprite) = floating_query.get_mut(child)
            {
                floating_sprite.flip_x = direction.0.x < 0.;
            }
        }
    }
}
