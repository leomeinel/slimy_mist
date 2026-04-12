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
            update_particles::<Player, ParticleWalkingDust>
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
    fn is_active(&self, _action: AnimationAction) -> bool {
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
pub(crate) struct ParticleWalkingDust(AnimationAction);
impl Particle for ParticleWalkingDust {
    fn is_active(&self, action: AnimationAction) -> bool {
        self.0 == action
    }
}

/// Marker component for [`Attack::Melee`] particles.
#[derive(Component, Default)]
pub(crate) struct ParticleMeleeAttack;
impl Particle for ParticleMeleeAttack {}

/// Spawn a [`Particle`] once
#[derive(Event)]
pub(crate) struct SpawnParticleOnce {
    pub(crate) pos: Vec3,
    pub(crate) handle: Handle<Particle2dEffect>,
}

/// Handle for [`Particle2dEffect`].
#[derive(Resource, Default)]
pub(crate) struct ParticleHandle<T>
where
    T: Particle,
{
    pub(crate) handle: Handle<Particle2dEffect>,
    pub(crate) _phantom: PhantomData<T>,
}

/// Timer that tracks particles.
#[derive(Component, Debug, Clone, PartialEq, Reflect, Deref, DerefMut)]
#[reflect(Component)]
struct ParticleTimer(Timer);

/// Spawn and despawn a [`Particle`] once.
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
fn add_walking_dust<T>(
    base_query: Query<(), With<AnimationBase>>,
    query: Query<&Children, With<T>>,
    mut commands: Commands,
    cel_size: Res<CelSize<T>>,
    handle: Res<ParticleHandle<ParticleWalkingDust>>,
) where
    T: Visible,
{
    let y_offset = cel_size.size.y as f32 / 2.;

    for children in query {
        let child = children
            .iter()
            .find(|e| base_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let particle = commands
            .spawn((
                ParticleWalkingDust(AnimationAction::Walk),
                ParticleTimer(Timer::from_seconds(WALKING_DUST_SECS, TimerMode::Repeating)),
                ParticleSpawner::default(),
                NoAutoAabb,
                ParticleSpawnerState {
                    active: false,
                    ..default()
                },
                ParticleEffectHandle(handle.handle.clone()),
                Transform::from_translation(Vec3::new(0., -y_offset, -LAYER_Z_DELTA)),
            ))
            .id();
        commands.entity(child).add_child(particle);
    }
}

/// Update particle for type `T`.
fn update_particles<T, A>(
    base_query: Query<&Children, With<AnimationBase>>,
    character_query: Query<(&mut AnimationState, &Children), With<T>>,
    mut particle_query: Query<
        (
            &ParticleWalkingDust,
            &ParticleTimer,
            &mut ParticleSpawnerState,
        ),
        With<A>,
    >,
) where
    T: Visible,
    A: Particle,
{
    for (animation_state, children) in character_query {
        let child = children
            .iter()
            .find(|e| base_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let children = base_query.get(child).expect(ERR_INVALID_CHILDREN);
        let child = children
            .iter()
            .find(|e| particle_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let (particle, timer, mut state) =
            particle_query.get_mut(child).expect(ERR_INVALID_CHILDREN);

        if !timer.0.just_finished() {
            continue;
        }

        state.set_new_active(particle.is_active(animation_state.0.0));
    }
}
