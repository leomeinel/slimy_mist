use bevy::{math::FloatPow, prelude::*};
use bevy_enoki::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animations::prelude::*, characters::prelude::*, images::prelude::*, log::prelude::*,
    render::prelude::*,
};

/// Marker component for blood particles.
#[derive(Component, Default)]
pub(crate) struct BloodParticle;
impl Particle for BloodParticle {}

/// Marker component for death particles.
#[derive(Component, Default)]
pub(crate) struct DeathParticle;
impl Particle for DeathParticle {}

/// Marker component for dust trail particles.
///
/// This
#[derive(Component, Default)]
pub(crate) struct DustTrailParticle(pub(crate) AnimationAction);
impl Particle for DustTrailParticle {}

/// Marker component for [`Attack::Melee`] particles.
#[derive(Component, Default)]
pub(crate) struct MeleeParticle;
impl Particle for MeleeParticle {}

/// Interval for [`DustTrailParticle`].
const DUST_TRAIL_SECS: f32 = 0.5;

/// Add [`DustTrailParticle`].
pub(super) fn add_dust_trail<T, const ANIMATION_ACTION: u8>(
    base_query: Query<(), With<AnimationBase>>,
    query: Query<&Children, With<T>>,
    mut commands: Commands,
    cel_size: Res<CelSize<T>>,
    material: Res<Particle2dMaterialHandle<DustTrailParticleMaterial>>,
    particle: Res<ParticleHandle<DustTrailParticle>>,
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
            DustTrailParticle(
                AnimationAction::try_from(ANIMATION_ACTION).expect(ERR_INVALID_ENUM_PRIMITVE),
            ),
            ParticleTimer(Timer::from_seconds(DUST_TRAIL_SECS, TimerMode::Repeating)),
            ParticleSpawner(material.0.clone()),
            NoAutoAabb,
            ParticleSpawnerState {
                active: false,
                ..default()
            },
            ParticleEffectHandle(particle.handle.clone()),
            Transform::from_translation(Vec3::new(0., -y_offset, -LAYER_Z_DELTA)),
        ));
    }
}

/// Minimum fraction of [`WalkSpeed`] for which to activate [`Particle`]s.
const MIN_WALK_SPEED_FRAC: f32 = 0.75;

/// Toggle [`DustTrailParticle`] for `T`.
///
/// This triggers [`ToggleParticle`] on certain conditions.
pub(super) fn toggle_dust_trail<T>(
    base_query: Query<&Children, With<AnimationBase>>,
    character_query: Query<
        (
            &mut AnimationState,
            &KinematicCharacterControllerOutput,
            &WalkSpeed,
            &Children,
        ),
        With<T>,
    >,
    particle_query: Query<(Entity, &DustTrailParticle, &ParticleTimer)>,
    mut commands: Commands,
    time: Res<Time>,
) where
    T: Visible,
{
    for (state, controller_output, walk_speed, children) in character_query {
        let child = children
            .iter()
            .find(|e| base_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let children = base_query.get(child).expect(ERR_INVALID_CHILDREN);

        let child = children
            .iter()
            .find(|e| particle_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let (entity, particle, timer) = particle_query.get(child).expect(ERR_INVALID_CHILDREN);

        if timer.0.just_finished() {
            let activate = state.0.0 == particle.0
                && controller_output.desired_translation.length_squared()
                    > ((walk_speed.0 * MIN_WALK_SPEED_FRAC) * time.delta_secs()).squared();

            commands.trigger(ToggleParticle::<DustTrailParticle>::new(entity, activate));
        }
    }
}
