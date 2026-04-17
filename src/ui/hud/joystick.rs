/*
 * File: joystick.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * Heavily inspired by:
 * -  https://github.com/SergioRibera/virtual_joystick
 */

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;
use virtual_joystick::{
    JoystickFixed, NoAction, VirtualJoystickBundle, VirtualJoystickInteractionArea,
    VirtualJoystickNode, VirtualJoystickUIBackground, VirtualJoystickUIKnob,
};

use crate::{screens::prelude::*, ui::prelude::*};

pub(super) struct UiJoystickPlugin;
impl Plugin for UiJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<JoystickState<{ JoystickID::MOVEMENT }>>();

        // Toggle joystick
        app.add_systems(
            OnEnter(Screen::Gameplay),
            spawn_joystick::<{ JoystickID::MOVEMENT }>
                .in_set(HudSystems::Append)
                .run_if(
                    in_state(JoystickState::<{ JoystickID::MOVEMENT }>::Toggled(true))
                        .or(in_state(JoystickState::<{ JoystickID::MOVEMENT }>::Spawned)),
                ),
        );
        app.add_systems(
            OnEnter(JoystickState::<{ JoystickID::MOVEMENT }>::Toggled(true)),
            spawn_joystick::<{ JoystickID::MOVEMENT }>.in_set(HudSystems::Append),
        );
        app.add_systems(
            OnEnter(JoystickState::<{ JoystickID::MOVEMENT }>::Toggled(false)),
            despawn_joystick::<{ JoystickID::MOVEMENT }>.in_set(HudSystems::Append),
        );
    }
}

/// Tracks the current state of the joystick with `const ID`.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum JoystickState<const ID: u8> {
    None,
    Toggled(bool),
    Spawned,
}
impl<const ID: u8> Default for JoystickState<ID> {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn default() -> Self {
        Self::None
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    fn default() -> Self {
        Self::Toggled(true)
    }
}
impl<const ID: u8> JoystickState<ID> {
    pub(crate) fn is_active(&self) -> bool {
        match self {
            JoystickState::<ID>::Toggled(val) => *val,
            JoystickState::Spawned => true,
            JoystickState::None => false,
        }
    }
}

/// Assets for joystick
#[derive(AssetCollection, Resource)]
pub(crate) struct JoystickAssets {
    #[asset(path = "images/ui/joystick-knob.webp")]
    #[asset(image(sampler(filter = linear)))]
    knob_image: Handle<Image>,
    #[asset(path = "images/ui/joystick-background.webp")]
    #[asset(image(sampler(filter = linear)))]
    background_image: Handle<Image>,
}

/// Enum representation of a joystick ID to have a single source of truth for IDs.
///
/// This can be used as a [`VirtualJoystickID`](virtual_joystick::VirtualJoystickID) after casting to [`u8`].
pub(crate) struct JoystickID;
impl JoystickID {
    pub(crate) const MOVEMENT: u8 = 0;
}

/// Map of [`JoystickID`]s as [`u8`] mapped to their [`Entity`].
#[derive(Resource, Default)]
pub(crate) struct JoystickMap(pub(crate) HashMap<u8, Entity>);

/// Size of the joystick knob in pixels
const JOYSTICK_KNOB_SIZE: Vec2 = Vec2::splat(75.);
/// Size of the joystick background in pixels
const JOYSTICK_BACKGROUND_SIZE: Vec2 = Vec2::splat(150.);

/// Spawn joystick with with `const ID`.
fn spawn_joystick<const ID: u8>(
    hud_query: Query<(&Hud, Entity)>,
    mut commands: Commands,
    mut joystick_map: ResMut<JoystickMap>,
    mut next_state: ResMut<NextState<JoystickState<ID>>>,
    joystick_assets: Res<JoystickAssets>,
) {
    let Some((_, hud_entity)) = hud_query.iter().find(|(h, _)| **h == Hud::BottomLeft) else {
        return;
    };
    let entity = commands.spawn(joystick::<ID>(&joystick_assets)).id();
    commands.entity(hud_entity).add_child(entity);

    joystick_map.0.insert(ID, entity);
    (*next_state).set_if_neq(JoystickState::<ID>::Spawned);
}

/// Despawn joystick with `const ID`.
fn despawn_joystick<const ID: u8>(
    mut commands: Commands,
    mut joystick_map: ResMut<JoystickMap>,
    mut next_state: ResMut<NextState<JoystickState<ID>>>,
) {
    let Some(entity) = joystick_map.0.get(&ID) else {
        return;
    };
    commands.entity(*entity).despawn();

    joystick_map.0.remove(&ID);
    (*next_state).set_if_neq(JoystickState::<ID>::None);
}

/// Joystick with `const ID`.
fn joystick<const ID: u8>(joystick_assets: &JoystickAssets) -> impl Bundle {
    (
        VirtualJoystickBundle::new(
            VirtualJoystickNode::default()
                .with_id(ID)
                .with_behavior(JoystickFixed)
                .with_action(NoAction),
        )
        .set_style(Node {
            width: px(JOYSTICK_BACKGROUND_SIZE.x),
            height: px(JOYSTICK_BACKGROUND_SIZE.y),
            ..default()
        }),
        children![
            (
                VirtualJoystickInteractionArea,
                Node {
                    width: percent(100.),
                    height: percent(100.),
                    ..default()
                },
                NodeRect::default(),
            ),
            (
                VirtualJoystickUIBackground,
                // TODO: Think about just using colored Node.
                ImageNode {
                    color: JOYSTICK_IMAGE,
                    image: joystick_assets.background_image.clone(),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: px(JOYSTICK_BACKGROUND_SIZE.x),
                    height: px(JOYSTICK_BACKGROUND_SIZE.y),
                    ..default()
                },
                ZIndex(0),
            ),
            (
                VirtualJoystickUIKnob,
                // TODO: Think about just using colored Node.
                ImageNode {
                    color: JOYSTICK_IMAGE,
                    image: joystick_assets.knob_image.clone(),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: px(JOYSTICK_KNOB_SIZE.x),
                    height: px(JOYSTICK_KNOB_SIZE.y),
                    ..default()
                },
                ZIndex(1),
            ),
        ],
    )
}
