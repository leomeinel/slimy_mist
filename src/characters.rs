/*
 * File: characters.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Characters

mod attack;
mod collision;
mod health;
mod movement;
mod nav;
mod npc;
mod player;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::attack::{
        AimDirection, Attack, AttackData, AttackStats, AttackTimer, DelayAttack, InitAttack, punch,
    };
    pub(crate) use super::collision::{CollisionData, CollisionDataCache, CollisionHandle};
    pub(crate) use super::health::{Damage, Health};
    pub(crate) use super::movement::{
        FacingDirection, JUMP_DURATION_SECS, JumpHeight, JumpTimer, WalkSpeed,
    };
    pub(crate) use super::nav::{NavTarget, Navigator, Path};
    pub(crate) use super::npc::{Npc, Slime, SlimeAssets};
    pub(crate) use super::player::{Player, PlayerAssets};
    pub(crate) use super::{
        Character, CharacterAssets, CharacterDimensions, CharacterShadow, SpawnCharacter,
        StaticShadow, impl_character_assets,
    };
}

use std::marker::PhantomData;

use bevy::{prelude::*, reflect::Reflectable};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_prng::WyRand;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use rand::RngExt as _;

use crate::{
    animations::prelude::*, characters::prelude::*, core::prelude::*, levels::prelude::*,
    procgen::prelude::*, render::prelude::*, screens::prelude::*, utils::prelude::*,
};

pub(super) struct CharactersPlugin;
impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterAnimations<Slime>>();
        app.init_resource::<CharacterAnimations<Player>>();

        app.add_systems(
            Update,
            (
                (
                    nav::find_path::<OverworldProcGen>,
                    nav::refresh_path::<OverworldProcGen>,
                )
                    .run_if(in_state(DespawnProcGen(false))),
                nav::apply_path.in_set(PausableSystems),
            )
                .run_if(in_state(ProcGenInit(true)).and(in_state(Screen::Gameplay)))
                .chain()
                .in_set(AppSystems::Update),
        );
        app.add_systems(
            Update,
            (
                player::on_init_attack,
                movement::update_facing_direction,
                attack::on_melee_attack::<Player>,
            )
                .run_if(in_state(Screen::Gameplay))
                .chain(),
        );
        app.add_systems(
            Update,
            (
                tick_component_timers::<attack::AttackTimer>,
                tick_component_timers::<movement::JumpTimer>,
            )
                .in_set(AppSystems::TickTimers),
        );

        app.add_observer(attack::on_delay_attack);
        app.add_observer(health::on_damage);
        app.add_observer(nav::on_stop_nav);
        app.add_observer(on_spawn_character::<Player, Overworld>);
        app.add_observer(on_spawn_character::<Slime, Overworld>);
    }
}

/// Applies to anything that stores character assets
pub(crate) trait CharacterAssets
where
    Self: AssetCollection + Resource + Default + Reflectable,
{
    fn sounds(&self, action: AnimationAction) -> &Option<Vec<Handle<AudioSource>>>;
}
macro_rules! impl_character_assets {
    ($type: ty) => {
        impl CharacterAssets for $type {
            fn sounds(&self, action: AnimationAction) -> &Option<Vec<Handle<AudioSource>>> {
                match action {
                    AnimationAction::Idle => &self.idle_sounds,
                    AnimationAction::Walk => &self.walk_sounds,
                    AnimationAction::Jump => &self.jump_sounds,
                }
            }
        }
    };
}
pub(crate) use impl_character_assets;

/// Applies to any character [`Component`]
pub(crate) trait Character
where
    Self: Component + Default + Reflectable,
{
    fn container_bundle(pos: Vec2, animation_delay: f32, offset: f32) -> impl Bundle;

    fn animation_bundle(animation: &CharacterAnimation) -> impl Bundle {
        (
            animation.sprite.clone(),
            SpritesheetAnimation::new(AnimationState::default().animation(animation)),
        )
    }

    fn collider(shape: String, width: f32, height: f32) -> Collider {
        match shape.as_str() {
            "ball" => Collider::ball(width / 2.),
            "capsule" => {
                if width > height {
                    Collider::capsule_x((height - width) / 2., height / 2.)
                } else {
                    Collider::capsule_y((width - height) / 2., width / 2.)
                }
            }
            _ => Collider::cuboid(width / 2., height / 2.),
        }
    }

    fn shadow_bundle(shadow: &StaticShadow, height: f32) -> impl Bundle {
        (
            Mesh2d(shadow.mesh.clone()),
            // FIXME: Using `LightOccluder2d` might be a good idea instead, but we will
            //        have to wait for occluder support in `bevy_fast_light`.
            MeshMaterial2d(shadow.material.clone()),
            Transform::from_xyz(0., -height / 4., BACKGROUND_Z_DELTA),
        )
    }
}

/// Dimensions for all [`Character`]s of type `T`.
///
/// This is related to [`CollisionData`].
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct CharacterDimensions<T>
where
    T: Character,
{
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) _phantom: PhantomData<T>,
}

/// Shadow for all [`Character`]s of type `T`.
///
/// This is related to [`CollisionData`]
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct CharacterShadow<T>
where
    T: Character,
{
    pub(crate) shadow: StaticShadow,
    pub(crate) _phantom: PhantomData<T>,
}

/// Artificial shadow.
#[derive(Default)]
pub(crate) struct StaticShadow {
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) material: Handle<ColorMaterial>,
}

/// [`EntityEvent`] for spawning a [`Character`].
///
/// ## Traits
///
/// - `T` must implement [`Character`].
/// - `A` must implement [`Level`].
#[derive(EntityEvent)]
pub(crate) struct SpawnCharacter<T, A>
where
    T: Character,
    A: Level,
{
    pub(crate) entity: Entity,
    pub(crate) pos: Vec2,
    pub(crate) _phantom: PhantomData<(T, A)>,
}

/// Spawn a single [`Character`].
///
/// ## Traits
///
/// - `T` must implement [`Character`].
/// - `A` must implement [`Level`].
fn on_spawn_character<T, A>(
    event: On<SpawnCharacter<T, A>>,
    mut animation_rng: Single<&mut WyRand, With<AnimationRng>>,
    level: Single<Entity, With<A>>,
    mut commands: Commands,
    character_animations: Res<CharacterAnimations<T>>,
    character_dimensions: Res<CharacterDimensions<T>>,
    collision_data: Res<CollisionDataCache<T>>,
    shadow: Res<CharacterShadow<T>>,
) where
    T: Character,
    A: Level,
{
    let animation_delay = animation_rng.random_range(ANIMATION_DELAY_RANGE_SECS);
    let (collider_shape, collider_width, collider_height, collider_offset) = (
        collision_data.shape.clone(),
        collision_data.width,
        collision_data.height,
        collision_data.offset,
    );

    let entity = commands
        .entity(event.entity)
        .insert((
            T::container_bundle(event.pos, animation_delay, collider_offset),
            T::collider(collider_shape, collider_width, collider_height),
            children![T::shadow_bundle(
                &shadow.shadow,
                character_dimensions.height
            )],
        ))
        .with_children(|commands| {
            let mut animation = commands.spawn((
                T::animation_bundle(&character_animations.base),
                Transform::from_xyz(0., -collider_offset, 0.),
                AnimationBase,
            ));
            if let Some(floating) = &character_animations.floating {
                animation.with_children(|commands| {
                    commands.spawn((
                        T::animation_bundle(floating),
                        Transform::from_xyz(0., 0., LAYER_Z_DELTA),
                    ));
                });
            }
        })
        .id();

    // Add `entity` to level so that level handles despawning
    commands.entity(*level).add_child(entity);
}
