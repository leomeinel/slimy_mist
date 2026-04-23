/*
 * File: nav.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/bevyengine/bevy/blob/latest/examples/ui/auto_directional_navigation.rs
 */

use core::time::Duration;

use bevy::{
    camera::NormalizedRenderTarget,
    input_focus::{InputFocus, InputFocusVisible, directional_navigation::AutoNavigationConfig},
    math::CompassOctant,
    picking::{
        backend::HitData,
        pointer::{Location, PointerId},
    },
    prelude::*,
    ui::auto_directional_navigation::{AutoDirectionalNavigation, AutoDirectionalNavigator},
};

use crate::{input::prelude::*, log::prelude::*, ui::prelude::*};

pub(super) struct UiInputNavPlugin;
impl Plugin for UiInputNavPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AutoNavigationConfig {
            min_alignment_factor: 0.5,
            // NOTE: Currently this is chosen so that users can navigate horizontally in the settings menu without
            //       reaching the back button. This is somewhat arbitrary, but works reliably.
            max_search_distance: Some(95.),
            prefer_aligned: false,
        });

        app.add_systems(OnEnter(OverrideInteraction(true)), set_input_focus);
        app.add_systems(
            OnEnter(OverrideInteraction(false)),
            reset_interaction_overrides,
        );
        app.add_systems(
            PreUpdate,
            (
                override_interaction.run_if(
                    in_state(OverrideInteraction(false))
                        .and(any_with_component::<AutoDirectionalNavigation>),
                ),
                (
                    reset_override,
                    override_interaction_on_release,
                    override_interaction_on_focus,
                    navigate,
                )
                    .run_if(in_state(OverrideInteraction(true)))
                    .chain(),
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (hover_focused, click_focused).run_if(in_state(OverrideInteraction(true))),
        );

        app.add_observer(reset_override_on_remove_nav);
        app.add_observer(override_interaction_on_click);
    }
}

/// Set initial focus to top left-most [`AutoDirectionalNavigation`].
pub(super) fn set_input_focus(
    query: Query<(Entity, &UiGlobalTransform), With<AutoDirectionalNavigation>>,
    mut input_focus: ResMut<InputFocus>,
) {
    if let Some(button) = query
        .iter()
        .min_by(|(_, transform), (_, other)| {
            transform
                .translation
                .y
                .total_cmp(&other.translation.y)
                .then_with(|| transform.translation.x.total_cmp(&other.translation.x))
        })
        .map(|(e, _)| e)
    {
        input_focus.set(button);
    }
}

/// Reset all [`InteractionOverride`]s.
pub(super) fn reset_interaction_overrides(
    query: Query<&mut InteractionOverride, With<AutoDirectionalNavigation>>,
) {
    for mut interaction_override in query {
        interaction_override.set_new(Interaction::None);
    }
}

/// Enable [`OverrideInteraction`] if [`UiNavActionSet`] is not empty.
pub(super) fn override_interaction(
    mut next_state: ResMut<NextState<OverrideInteraction>>,
    action_set: Res<UiNavActionSet>,
) {
    if !action_set.0.is_empty() {
        (*next_state).set_if_neq(OverrideInteraction(true));
    }
}

/// Set [`OverrideInteraction`] to false if any [`Interaction`] with [`AutoDirectionalNavigation`] is not [`Interaction::None`].
pub(super) fn reset_override(
    query: Query<&Interaction, With<AutoDirectionalNavigation>>,
    mut next_state: ResMut<NextState<OverrideInteraction>>,
) {
    if query.iter().any(|i| *i != Interaction::None) {
        (*next_state).set_if_neq(OverrideInteraction(false));
    }
}

/// Set correct [`InteractionOverride`] for focused [`AutoDirectionalNavigation`]s.
pub(super) fn override_interaction_on_focus(
    query: Query<(Entity, &mut InteractionOverride), With<AutoDirectionalNavigation>>,
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
) {
    if !input_focus_visible.0 {
        return;
    }
    for (entity, mut interaction_override) in query {
        if input_focus.0 == Some(entity) {
            interaction_override.set_new_if_current(Interaction::None, Interaction::Hovered);
        } else {
            interaction_override.set_new_if_current(Interaction::Hovered, Interaction::None);
        }
    }
}

/// Set correct [`InteractionOverride`] for selected [`AutoDirectionalNavigation`]s on press.
pub(super) fn override_interaction_on_click(
    event: On<Pointer<Click>>,
    mut query: Query<&mut InteractionOverride, With<AutoDirectionalNavigation>>,
    input_focus_visible: Res<InputFocusVisible>,
) {
    if !input_focus_visible.0 {
        return;
    }
    if let Ok(mut interaction_override) = query.get_mut(event.entity) {
        interaction_override.set_new_if_current(Interaction::Hovered, Interaction::Pressed);
    };
}

/// Set correct [`InteractionOverride`] for selected [`AutoDirectionalNavigation`]s on release.
pub(super) fn override_interaction_on_release(
    query: Query<&mut InteractionOverride, With<AutoDirectionalNavigation>>,
    action_set: Res<UiNavActionSet>,
    input_focus_visible: Res<InputFocusVisible>,
) {
    if !input_focus_visible.0 || !action_set.0.contains(&UiNavAction::Select(false)) {
        return;
    }
    for mut interaction_override in query {
        interaction_override.set_new_if_current(Interaction::Pressed, Interaction::None);
    }
}

/// Navigate to [`UiNavActionSet::direction`].
pub(super) fn navigate(mut navigator: AutoDirectionalNavigator, action_set: Res<UiNavActionSet>) {
    let direction = action_set.direction().map(CompassOctant::from);
    if let Some(direction) = direction
        && let Err(_e) = navigator.navigate(direction)
    {
        let directions = match direction {
            CompassOctant::North => &[CompassOctant::NorthWest, CompassOctant::NorthEast],
            CompassOctant::East => &[CompassOctant::SouthEast, CompassOctant::NorthEast],
            CompassOctant::South => &[CompassOctant::SouthEast, CompassOctant::SouthWest],
            CompassOctant::West => &[CompassOctant::NorthWest, CompassOctant::SouthWest],
            _ => unreachable!(),
        };
        if !directions.iter().any(|d| navigator.navigate(*d).is_ok()) {
            warn!("{}", WARN_INVALID_UI_NAV);
        }
    }
}

/// Trigger [`Pointer<Over>`] on focused [`Entity`]s.
pub(super) fn hover_focused(
    mut commands: Commands,
    input_focus: Res<InputFocus>,
    mut last_entity: Local<Option<Entity>>,
) {
    if input_focus.0 != *last_entity
        && let Some(entity) = input_focus.0
    {
        // NOTE: Since we only need to trigger the pointer hover for the entity,
        //       we are mostly using placeholder values.
        commands.trigger(Pointer::<Over> {
            entity,
            pointer_id: PointerId::Mouse,
            pointer_location: Location {
                target: NormalizedRenderTarget::None {
                    width: 0,
                    height: 0,
                },
                position: Vec2::ZERO,
            },
            event: Over {
                hit: HitData {
                    camera: Entity::PLACEHOLDER,
                    depth: 0.0,
                    position: None,
                    normal: None,
                },
            },
        });
        *last_entity = Some(entity);
    }
}

/// Trigger [`Pointer<Click>`] on focused [`Entity`]s mapped to [`UiNavAction::Select`] in [`UiNavActionSet`].
pub(super) fn click_focused(
    mut commands: Commands,
    action_set: Res<UiNavActionSet>,
    input_focus: Res<InputFocus>,
) {
    if action_set.0.contains(&UiNavAction::Select(true))
        && let Some(entity) = input_focus.0
    {
        // NOTE: Since we only need to trigger the pointer click for the entity,
        //       we are mostly using placeholder values.
        commands.trigger(Pointer::<Click> {
            entity,
            pointer_id: PointerId::Mouse,
            pointer_location: Location {
                target: NormalizedRenderTarget::None {
                    width: 0,
                    height: 0,
                },
                position: Vec2::ZERO,
            },
            event: Click {
                button: PointerButton::Primary,
                hit: HitData {
                    camera: Entity::PLACEHOLDER,
                    depth: 0.0,
                    position: None,
                    normal: None,
                },
                duration: Duration::from_secs_f32(0.1),
            },
        });
    }
}

/// Set [`OverrideInteraction`] to false.
pub(super) fn reset_override_on_remove_nav(
    _: On<Remove, AutoDirectionalNavigation>,
    mut next_state: ResMut<NextState<OverrideInteraction>>,
) {
    (*next_state).set_if_neq(OverrideInteraction(false));
}
