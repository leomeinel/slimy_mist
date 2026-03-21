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
        Attack, AttackData, AttackStats, AttackTimer, AttackType, MeleeAttack, punch,
    };
    pub(crate) use super::collision::{
        CollisionData, CollisionDataCache, CollisionDataRelatedCache, CollisionHandle,
        character_collider,
    };
    pub(crate) use super::health::{Damage, Health};
    pub(crate) use super::movement::{
        FacingDirection, JUMP_DURATION_SECS, JumpHeight, JumpTimer, WalkSpeed,
    };
    pub(crate) use super::nav::{NavTarget, NavTargetPosMap, Navigator, Path};
    pub(crate) use super::npc::{Npc, Slime, SlimeAssets};
    pub(crate) use super::player::{Player, PlayerAssets};
    pub(crate) use super::{
        Character, CharacterAssets, SpawnCharacter, StaticShadow, impl_character_assets,
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
        app.init_resource::<Animations<Slime>>();
        app.init_resource::<Animations<Player>>();

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
                .in_set(AppSystems::Update),
        );
        app.add_systems(
            Update,
            (
                player::apply_jump.before(PhysicsSet::SyncBackend),
                player::limit_jump,
            )
                .chain()
                .in_set(AppSystems::Update),
        );
        app.add_systems(
            PostUpdate,
            movement::update_facing.run_if(in_state(Screen::Gameplay)),
        );

        app.add_systems(
            Update,
            (
                tick_component_timer::<attack::AttackTimer>,
                tick_component_timer::<movement::JumpTimer>,
            )
                .in_set(AppSystems::TickTimers),
        );

        app.add_observer(attack::on_melee_attack::<Player>);
        app.add_observer(attack::on_delay_attack);
        app.add_observer(health::on_damage);
        app.add_observer(on_spawn_character::<Player, Overworld>);
        app.add_observer(on_spawn_character::<Slime, Overworld>);
    }
}

/// Applies to anything that stores character assets
pub(crate) trait CharacterAssets
where
    Self: AssetCollection + Resource + Default + Reflectable,
{
    fn walk_sounds(&self) -> &Option<Vec<Handle<AudioSource>>>;
    fn jump_sounds(&self) -> &Option<Vec<Handle<AudioSource>>>;
    fn fall_sounds(&self) -> &Option<Vec<Handle<AudioSource>>>;
}
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
pub(crate) use impl_character_assets;

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
