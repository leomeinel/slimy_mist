pub(super) mod effects;
pub(super) mod materials;

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_enoki::prelude::*;

use crate::{
    animations::prelude::*, characters::prelude::*, core::prelude::*, levels::prelude::*,
    render::prelude::*, screens::prelude::*, utils::prelude::*,
};

pub(super) struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EnokiPlugin,
            Particle2dMaterialPlugin::<BloodParticleMaterial>::default(),
            Particle2dMaterialPlugin::<DeathParticleMaterial>::default(),
            Particle2dMaterialPlugin::<DustTrailParticleMaterial>::default(),
            Particle2dMaterialPlugin::<MeleeParticleMaterial>::default(),
        ));

        app.add_systems(
            OnEnter(Screen::Gameplay),
            effects::add_dust_trail::<Player, { AnimationAction::WALK }>
                .after(EnterGameplaySystems::Levels),
        );
        app.add_systems(
            Update,
            effects::toggle_dust_trail::<Player>.run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            tick_component_timers::<ParticleTimer>.in_set(AppSystems::TickTimers),
        );

        app.add_observer(on_spawn_child_particle_once::<BloodParticle, BloodParticleMaterial>);
        app.add_observer(on_spawn_particle_once::<DeathParticle, DeathParticleMaterial, Overworld>);
        app.add_observer(on_toggle_particle::<DustTrailParticle>);
        app.add_observer(on_spawn_child_particle_once::<MeleeParticle, MeleeParticleMaterial>);
    }
}

/// Applies to anything that is considered a particle.
pub(crate) trait Particle
where
    Self: Component + Default,
{
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
pub(crate) struct SpawnChildParticleOnce<T>
where
    T: Particle,
{
    pub(crate) entity: Entity,
    pub(crate) offset: Vec3,
    pub(crate) handle: Handle<Particle2dEffect>,
    _phantom: PhantomData<T>,
}
impl<T> SpawnChildParticleOnce<T>
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

/// Spawn a [`Particle`] once at `pos`.
#[derive(Event, Default)]
pub(crate) struct SpawnParticleOnce<T>
where
    T: Particle,
{
    pub(crate) pos: Vec3,
    pub(crate) handle: Handle<Particle2dEffect>,
    _phantom: PhantomData<T>,
}
impl<T> SpawnParticleOnce<T>
where
    T: Particle,
{
    pub(crate) fn new(pos: Vec3, handle: Handle<Particle2dEffect>) -> Self {
        Self {
            pos,
            handle,
            ..default()
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

/// Spawn and despawn a [`Particle`] as child once.
fn on_spawn_child_particle_once<T, A>(
    event: On<SpawnChildParticleOnce<T>>,
    mut commands: Commands,
    material: Res<Particle2dMaterialHandle<A>>,
) where
    T: Particle,
    A: Particle2dMaterial + Default,
{
    if let Ok(mut entity_commands) = commands.get_entity(event.entity) {
        entity_commands.with_child((
            T::default(),
            OneShot::Despawn,
            ParticleSpawner(material.0.clone()),
            NoAutoAabb,
            Transform::from_translation(event.offset),
            ParticleEffectHandle(event.handle.clone()),
        ));
    }
}

/// Spawn and despawn a [`Particle`] once at a specific position.
fn on_spawn_particle_once<T, A, B>(
    event: On<SpawnParticleOnce<T>>,
    level: Single<Entity, With<B>>,
    mut commands: Commands,
    material: Res<Particle2dMaterialHandle<A>>,
) where
    T: Particle,
    A: Particle2dMaterial + Default,
    B: Level,
{
    commands.entity(*level).with_child((
        T::default(),
        OneShot::Despawn,
        ParticleSpawner(material.0.clone()),
        NoAutoAabb,
        Transform::from_translation(event.pos),
        ParticleEffectHandle(event.handle.clone()),
    ));
}
