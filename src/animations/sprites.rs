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
    let mut character_animations = SpriteAnimations::<T> {
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
    character_animations.insert_animations(
        idle_clips,
        &mut animations,
        &base_sheet,
        floating_sheet,
        AnimationRepeat::Loop,
    );
    if let Some(walk_clips) = walk_clips {
        character_animations.insert_animations(
            walk_clips,
            &mut animations,
            &base_sheet,
            floating_sheet,
            AnimationRepeat::Loop,
        );
    }
    if let Some(jump_clips) = jump_clips {
        character_animations.insert_animations(
            jump_clips,
            &mut animations,
            &base_sheet,
            floating_sheet,
            AnimationRepeat::Times(1),
        );
    }

    commands.insert_resource(character_animations);
}

/// Update animations.
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
    mut base_query: Query<(&mut SpritesheetAnimation, Option<&Children>), With<AnimationBase>>,
    mut floating_query: Query<&mut SpritesheetAnimation, Without<AnimationBase>>,
    animations: Res<SpriteAnimations<T>>,
) where
    T: Visible,
{
    for (mut audio_index, animation_state, timer, children) in character_query {
        let children: Vec<_> = children.iter().collect();
        let entity = children
            .iter()
            .find(|entity| base_query.contains(**entity))
            .expect(ERR_INVALID_CHILDREN);
        let (mut base_animation, children) =
            base_query.get_mut(*entity).expect(ERR_INVALID_CHILDREN);
        if timer.0.just_finished() {
            base_animation.reset();
        }
        animation_state.switch(&animations.base, &mut base_animation, &mut audio_index);

        if let Some(children) = children
            && let Some(entity) = children
                .iter()
                .find(|entity| floating_query.contains(*entity))
            && let Ok(mut floating_animation) = floating_query.get_mut(entity)
            && let Some(animation) = &&animations.floating
        {
            if timer.0.just_finished() {
                floating_animation.reset();
            }
            animation_state.switch(animation, &mut floating_animation, &mut audio_index);
        }
    }
}

/// Update [`AnimationOrientation`]s and flip [`Sprite`]s.
pub(super) fn update_animation_orientations<T>(
    character_query: Query<(&mut AnimationState, &FacingDirection, &Children), With<T>>,
    mut base_query: Query<(&mut Sprite, Option<&Children>), With<AnimationBase>>,
    mut floating_query: Query<&mut Sprite, Without<AnimationBase>>,
) where
    T: Visible,
{
    for (mut animation_state, direction, children) in character_query {
        let Some(orientation) = AnimationOrientation::try_from_vec2(direction.0) else {
            continue;
        };
        animation_state.0.1 = orientation;

        if animation_state.0.1 == AnimationOrientation::East {
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
