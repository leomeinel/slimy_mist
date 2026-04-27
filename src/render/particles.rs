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
            tick_component_timers::<ParticleTimer>.in_set(AppSystems::TickTimers),
        );

        app.add_observer(on_toggle_particle::<ParticleWalkingDust>);
        app.add_observer(on_spawn_particle_once::<ParticleBlood>);
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

/// Marker component for blood particles.
#[derive(Component, Default)]
pub(crate) struct ParticleBlood;
impl Particle for ParticleBlood {}

/// Marker component for [`Attack::Melee`] particles.
#[derive(Component, Default)]
pub(crate) struct ParticleMeleeAttack;
impl Particle for ParticleMeleeAttack {}

/// Marker component for walking dust particles.
#[derive(Component, Default)]
pub(crate) struct ParticleWalkingDust;
impl Particle for ParticleWalkingDust {
    fn is_active(&self, action: AnimationAction) -> bool {
        action == AnimationAction::Walk
    }
}

/// Toggle a [`Particle`].
#[derive(EntityEvent)]
pub(crate) struct ToggleParticle<T>
where
    T: Particle,
{
    pub(crate) entity: Entity,
    pub(crate) activate: bool,
    pub(crate) _phantom: PhantomData<T>,
}
impl<T> ToggleParticle<T>
where
    T: Particle,
{
    pub(crate) fn new(entity: Entity, activate: bool) -> Self {
        Self {
            entity,
            activate,
            _phantom: PhantomData,
        }
    }
}

/// Spawn a [`Particle`] once as child of `entity`.
///
/// If `entity` has been despawned, this will not spawn a [`Particle`].
#[derive(Event)]
pub(crate) struct SpawnParticleOnce<T>
where
    T: Particle,
{
    pub(crate) entity: Entity,
    pub(crate) offset: Vec3,
    pub(crate) handle: Handle<Particle2dEffect>,
    _phantom: PhantomData<T>,
}
impl<T> SpawnParticleOnce<T>
where
    T: Particle,
{
    pub(crate) fn new(entity: Entity, offset: Vec3, handle: Handle<Particle2dEffect>) -> Self {
        Self {
            entity,
            offset,
            handle,
            _phantom: PhantomData,
        }
    }
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
pub(crate) struct ParticleTimer(pub(crate) Timer);

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
        commands.entity(child).with_child((
            ParticleWalkingDust,
            ParticleTimer(Timer::from_seconds(WALKING_DUST_SECS, TimerMode::Repeating)),
            ParticleSpawner::default(),
            NoAutoAabb,
            ParticleSpawnerState {
                active: false,
                ..default()
            },
            ParticleEffectHandle(handle.handle.clone()),
            Transform::from_translation(Vec3::new(0., -y_offset, -LAYER_Z_DELTA)),
        ));
    }
}

/// Enable [`Particle`] on [`ToggleParticle`].
fn on_toggle_particle<T>(
    event: On<ToggleParticle<T>>,
    mut particle_query: Query<&mut ParticleSpawnerState, With<T>>,
) where
    T: Particle,
{
    let mut state = particle_query.get_mut(event.entity).unwrap();
    state.set_new_active(event.activate);
}

/// Spawn and despawn a [`Particle`] once.
fn on_spawn_particle_once<T>(event: On<SpawnParticleOnce<T>>, mut commands: Commands)
where
    T: Particle,
{
    if let Ok(mut entity_commands) = commands.get_entity(event.entity) {
        entity_commands.with_child((
            T::default(),
            OneShot::Despawn,
            ParticleSpawner::default(),
            NoAutoAabb,
            Transform::from_translation(event.offset),
            ParticleEffectHandle(event.handle.clone()),
        ));
    }
}
