/*
 * File: particles.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_enoki::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{
    animations::prelude::*, characters::prelude::*, core::prelude::*, images::prelude::*,
    log::prelude::*, render::prelude::*, screens::prelude::*, utils::prelude::*,
};

pub(super) struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnokiPlugin);

        app.add_systems(
            OnEnter(Screen::Gameplay),
            add_walking_dust::<Player>.after(EnterGameplaySystems::Levels),
        );
        app.add_systems(
            Update,
            update_character_particles::<Player, ParticleWalkingDust>
                .after(EnterGameplaySystems::Levels)
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            tick_component_timers::<ParticleTimer>.in_set(AppSystems::TickTimers),
        );

        app.add_observer(on_spawn_particle_once::<ParticleMeleeAttack>);
    }
}

/// Applies to anything that is considered a particle.
pub(crate) trait Particle
where
    Self: Component + Default,
{
    fn is_active(&self, _state: AnimationState) -> bool {
        true
    }
}

trait ParticleSpawnerExt {
    fn set_new_active(&mut self, new: bool);
}
impl ParticleSpawnerExt for ParticleSpawnerState {
    fn set_new_active(&mut self, new: bool) {
        if self.active != new {
            self.active = new;
        }
    }
}

/// Marker component for walking dust particles.
#[derive(Component, Default)]
pub(crate) struct ParticleWalkingDust(AnimationState);
impl Particle for ParticleWalkingDust {
    fn is_active(&self, animation_state: AnimationState) -> bool {
        self.0 == animation_state
    }
}

/// Marker component for [`MeleeAttack`] particles.
#[derive(Component, Default)]
pub(crate) struct ParticleMeleeAttack;
impl Particle for ParticleMeleeAttack {}

/// Spawn a [`Particle`] once
#[derive(Event)]
pub(crate) struct SpawnParticleOnce {
    pub(crate) pos: Vec3,
    pub(crate) handle: Handle<Particle2dEffect>,
}

/// Handle for [`Particle2dEffect`] as a generic.
///
/// ## Traits
///
/// - `T` must implement [`Particle`] and is used as the associated particle type.
#[derive(Resource, Default)]
pub(crate) struct ParticleHandle<T>
where
    T: Particle,
{
    pub(crate) handle: Handle<Particle2dEffect>,
    pub(crate) _phantom: PhantomData<T>,
}

/// Timer that tracks particles
#[derive(Component, Debug, Clone, PartialEq, Reflect, Deref, DerefMut)]
#[reflect(Component)]
struct ParticleTimer(Timer);

/// Spawn and despawn a [`Particle`] once.
///
/// ## Traits
///
/// - `T` must implement [`Particle`] and is used as the associated particle type.
fn on_spawn_particle_once<T>(event: On<SpawnParticleOnce>, mut commands: Commands)
where
    T: Particle,
{
    commands.spawn((
        T::default(),
        OneShot::Despawn,
        ParticleSpawner::default(),
        NoAutoAabb,
        Transform::from_translation(event.pos),
        ParticleEffectHandle(event.handle.clone()),
    ));
}

/// Interval for [`ParticleWalkingDust`].
const WALKING_DUST_SECS: f32 = 0.5;

/// Add [`ParticleWalkingDust`].
///
/// ## Traits
///
/// - `T` must implement [`Visible`].
fn add_walking_dust<T>(
    animation_query: Query<(), With<SpritesheetAnimation>>,
    query: Query<&Children, With<T>>,
    mut commands: Commands,
    texture_info: Res<ImageMeta<T>>,
    handle: Res<ParticleHandle<ParticleWalkingDust>>,
) where
    T: Visible,
{
    let texture_offset = texture_info.size.y as f32 / 2.;

    for children in query {
        let child = children
            .iter()
            .find(|e| animation_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let particle = commands
            .spawn((
                ParticleWalkingDust(AnimationState::Walk),
                ParticleTimer(Timer::from_seconds(WALKING_DUST_SECS, TimerMode::Repeating)),
                ParticleSpawner::default(),
                NoAutoAabb,
                ParticleSpawnerState {
                    active: false,
                    ..default()
                },
                ParticleEffectHandle(handle.handle.clone()),
                Transform::from_translation(Vec3::new(0., -texture_offset, BACKGROUND_Z_DELTA)),
            ))
            .id();
        commands.entity(child).add_child(particle);
    }
}

/// Update particle for [`Character`]s
///
/// ## Traits
///
/// - `T` must implement [`Character`] and [`Visible`].
/// - `A` must implement [`Particle`].
fn update_character_particles<T, A>(
    animation_query: Query<&Children, With<SpritesheetAnimation>>,
    character_query: Query<(&mut AnimationCache, &Children), With<T>>,
    mut particle_query: Query<
        (
            &ParticleWalkingDust,
            &ParticleTimer,
            &mut ParticleSpawnerState,
        ),
        With<A>,
    >,
) where
    T: Character + Visible,
    A: Particle,
{
    for (cache, children) in character_query {
        let child = children
            .iter()
            .find(|e| animation_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let children = animation_query.get(child).expect(ERR_INVALID_CHILDREN);
        let child = children
            .iter()
            .find(|e| particle_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let (particle, timer, mut state) =
            particle_query.get_mut(child).expect(ERR_INVALID_CHILDREN);

        // Continue if timer has not finished
        if !timer.0.just_finished() {
            continue;
        }

        state.set_new_active(particle.is_active(cache.state));
    }
}
