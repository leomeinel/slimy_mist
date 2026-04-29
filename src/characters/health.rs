use bevy::prelude::*;

use crate::render::prelude::*;

/// Health that determines if a [`Component`] should be despawned.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Health {
    pub(crate) max: f32,
    pub(crate) current: f32,
}
impl Health {
    pub(crate) fn new(max: f32) -> Self {
        Self { max, current: max }
    }
    pub(crate) fn fraction(&self) -> f32 {
        if self.max > 0. {
            (self.current / self.max).clamp(0., 1.)
        } else {
            0.
        }
    }
    pub(crate) fn is_alive(&self) -> bool {
        self.current > 0.
    }
}

/// Apply damage to [`Health`].
#[derive(Event)]
pub(crate) struct Damage {
    pub(crate) targets: Vec<Entity>,
    pub(crate) damage: f32,
}

/// Apply [`Damage`] to [`Health`] and handle particles and despawning.
pub(super) fn on_damage(
    event: On<Damage>,
    mut target_query: Query<(&mut Health, &Transform)>,
    mut commands: Commands,
    particle_blood: Res<ParticleHandle<ParticleBlood>>,
    particle_death: Res<ParticleHandle<ParticleDeath>>,
) {
    for entity in &event.targets {
        let Ok((mut health, transform)) = target_query.get_mut(*entity) else {
            continue;
        };

        health.current -= event.damage;
        if health.is_alive() {
            commands.trigger(SpawnChildParticleOnce::<ParticleBlood>::new(
                *entity,
                Vec3::new(0., 0., -LAYER_Z_DELTA),
                particle_blood.handle.clone(),
            ));
        } else {
            // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
            commands.entity(*entity).try_despawn();
            commands.trigger(SpawnParticleOnce::<ParticleDeath>::new(
                transform.translation.xy().extend(OVERLAY_Z),
                particle_death.handle.clone(),
            ));
        }
    }
}
