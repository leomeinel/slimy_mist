/*
 * File: characters.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Characters

pub(crate) mod attack;
pub(crate) mod health;
pub(crate) mod nav;
pub(crate) mod npc;
pub(crate) mod player;

use std::marker::PhantomData;

use bevy::{prelude::*, reflect::Reflectable};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_prng::WyRand;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::SpritesheetAnimation;
use rand::RngExt as _;

use crate::{
    AppSystems,
    animations::{ANIMATION_DELAY_RANGE_SECS, AnimationRng, Animations},
    camera::BACKGROUND_Z_DELTA,
    characters::{npc::Slime, player::Player},
    levels::{Level, overworld::Overworld},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    // Add child plugins
    app.add_plugins((
        attack::plugin,
        health::plugin,
        nav::plugin,
        npc::plugin,
        player::plugin,
    ));

    // Tick timers
    app.add_systems(Update, tick_jump_timer.in_set(AppSystems::TickTimers));

    // Update movement facing
    app.add_systems(PostUpdate, update_facing.run_if(in_state(Screen::Gameplay)));

    // Spawn characters
    app.add_observer(on_spawn_character::<Player, Overworld>);
    app.add_observer(on_spawn_character::<Slime, Overworld>);
}

/// Jumping duration in seconds
pub(crate) const JUMP_DURATION_SECS: f32 = 1.;

/// Applies to anything that stores character assets
pub(crate) trait CharacterAssets
where
    Self: AssetCollection + Resource + Default + Reflectable,
{
    fn walk_sounds(&self) -> &Option<Vec<Handle<AudioSource>>>;
    fn jump_sounds(&self) -> &Option<Vec<Handle<AudioSource>>>;
    fn fall_sounds(&self) -> &Option<Vec<Handle<AudioSource>>>;
}
#[macro_export]
macro_rules! impl_character_assets {
    ($type: ty) => {
        impl CharacterAssets for $type {
            fn walk_sounds(&self) -> &Option<Vec<Handle<AudioSource>>> {
                &self.walk_sounds
            }
            fn jump_sounds(&self) -> &Option<Vec<Handle<AudioSource>>> {
                &self.jump_sounds
            }
            fn fall_sounds(&self) -> &Option<Vec<Handle<AudioSource>>> {
                &self.fall_sounds
            }
        }
    };
}

/// Applies to any character [`Component`]
pub(crate) trait Character
where
    Self: Component + Default + Reflectable,
{
    fn container_bundle(&self, animation_delay: f32, pos: Vec2) -> impl Bundle;

    fn animation_bundle(&self, animations: &Res<Animations<Self>>) -> impl Bundle {
        (
            animations.sprite.clone(),
            SpritesheetAnimation::new(animations.idle.clone()),
        )
    }

    fn shadow_bundle(&self, height: f32, shadow: &StaticShadow) -> impl Bundle {
        (
            Mesh2d(shadow.mesh.clone()),
            // FIXME: Using `LightOccluder2d` might be a good idea instead, but we will
            //        have to wait for occluder support in `bevy_fast_light`.
            MeshMaterial2d(shadow.material.clone()),
            Transform::from_xyz(0., -height / 2., BACKGROUND_Z_DELTA),
        )
    }
}

/// Collision data deserialized from a ron file as a generic
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(serde::Deserialize, Asset, TypePath, Default)]
pub(crate) struct CollisionData<T>
where
    T: Character,
{
    #[serde(default)]
    pub(crate) shape: Option<String>,
    #[serde(default)]
    pub(crate) width: Option<f32>,
    #[serde(default)]
    pub(crate) height: Option<f32>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Handle for [`CollisionData`] as a generic
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource)]
pub(crate) struct CollisionHandle<T>(pub(crate) Handle<CollisionData<T>>)
where
    T: Character;

/// Cache for [`CollisionData`]
///
/// This is to allow easier access.
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct CollisionDataCache<T>
where
    T: Character,
{
    pub(crate) shape: String,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) _phantom: PhantomData<T>,
}

/// Cache for data that is related to [`CollisionData`]
///
/// This is to allow easier access.
///
/// ## Traits
///
/// - `T` must implement [`Character`].
#[derive(Resource, Default)]
pub(crate) struct CollisionDataRelatedCache<T>
where
    T: Character,
{
    pub(crate) shadow: StaticShadow,
    pub(crate) _phantom: PhantomData<T>,
}

/// Artificial shadow that is not affected by lighting.
#[derive(Default)]
pub(crate) struct StaticShadow {
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) material: Handle<ColorMaterial>,
}

// FIXME: This should be influenced by aiming direction and should determine sprite flip.
//        Movement direction should be secondary.
/// Direction the [`Character`] is facing.
#[derive(Component)]
pub(crate) struct FacingDirection(pub(crate) Vec2);
impl Default for FacingDirection {
    fn default() -> Self {
        Self(Vec2::X)
    }
}

/// [`Character`] jump height.
#[derive(Component, Default)]
pub(crate) struct JumpHeight(pub(crate) f32);

/// [`Character`] walking speed.
#[derive(Component)]
pub(crate) struct WalkSpeed(pub(crate) f32);

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

/// Timer that tracks jumping
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub(crate) struct JumpTimer(Timer);
impl Default for JumpTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            JUMP_DURATION_SECS / 2.,
            TimerMode::Once,
        ))
    }
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
    animations: Res<Animations<T>>,
    collision_data: Res<CollisionDataCache<T>>,
    collision_data_related: Res<CollisionDataRelatedCache<T>>,
) where
    T: Character,
    A: Level,
{
    let character = T::default();
    let animation_delay = animation_rng.random_range(ANIMATION_DELAY_RANGE_SECS);
    let (shape, width, height) = (
        collision_data.shape.clone(),
        collision_data.width,
        collision_data.height,
    );

    let container = commands
        .entity(event.entity)
        .insert((
            character.container_bundle(animation_delay, event.pos),
            character_collider(shape, width, height),
        ))
        .id();
    let animation = commands.spawn(character.animation_bundle(&animations)).id();
    let shadow = commands
        .spawn(character.shadow_bundle(height, &collision_data_related.shadow))
        .id();

    // Add entity to level so that level handles despawning
    commands
        .entity(container)
        .add_children(&[animation, shadow]);
    commands.entity(*level).add_child(container);
}

/// [`Collider`] for different shapes
pub(crate) fn character_collider(shape: String, width: f32, height: f32) -> Collider {
    // Set correct collider for each shape
    // NOTE: For capsules, we just assume that the values are correct, meaning that for x: `width < height` and for y: `width > height`
    match shape.as_str() {
        "ball" => Collider::ball(width / 2.),
        "capsule_x" => Collider::capsule_x((height - width) / 2., height / 2.),
        "capsule_y" => Collider::capsule_y((width - height) / 2., width / 2.),
        _ => Collider::cuboid(width / 2., height / 2.),
    }
}

/// Update [`FacingDirection`] from [`KinematicCharacterControllerOutput::desired_translation`].
fn update_facing(
    query: Query<
        (&mut FacingDirection, &KinematicCharacterControllerOutput),
        Changed<KinematicCharacterControllerOutput>,
    >,
) {
    for (mut facing, controller_output) in query {
        // NOTE: This only checks for desired movement, not actual movement. This is to ensure that
        //       even if a character can't move, it can still change its' facing direction.
        if controller_output.desired_translation != Vec2::ZERO {
            facing.0 = controller_output.desired_translation.normalize_or_zero();
        }
    }
}

/// Tick [`JumpTimer`]
fn tick_jump_timer(mut query: Query<&mut JumpTimer>, time: Res<Time>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}
