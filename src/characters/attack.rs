/*
 * File: attack.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::{platform::collections::HashSet, prelude::*};
use bevy_rapier2d::{parry::shape, prelude::*};
use ordered_float::OrderedFloat;

use crate::{characters::prelude::*, log::prelude::*, physics::prelude::*, render::prelude::*};

/// Direction the [`Character`] is aiming.
#[derive(Component, Deref, DerefMut)]
pub(crate) struct AimDirection(pub(crate) Vec2);
impl Default for AimDirection {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

/// Relevant data for an attack.
#[derive(Default, PartialEq, Eq, Hash)]
pub(crate) struct AttackData {
    pub(crate) name: &'static str,
    pub(crate) damage: OrderedFloat<f32>,
    /// Attack range in pixels.
    ///
    /// First value is width, second is height.
    pub(crate) range: (OrderedFloat<f32>, OrderedFloat<f32>),
    /// Cooldown in seconds after attack is done
    pub(crate) cooldown_secs: OrderedFloat<f32>,
}

/// [`Message`] that is written if the source [`Entity`] has attacked.
#[derive(Message)]
pub(crate) enum Attack {
    Melee(Entity),
    // TODO: Implement this.
    #[allow(dead_code)]
    Ranged(Entity),
}
impl From<&InitAttack> for Attack {
    fn from(attack: &InitAttack) -> Self {
        match attack {
            InitAttack::Melee(entity) => Self::Melee(*entity),
            InitAttack::Ranged(entity) => Self::Ranged(*entity),
        }
    }
}

/// [`Message`] that is written if the source [`Entity`] has attacked.
///
/// This initiates [`Attack`].
#[derive(Message)]
pub(crate) enum InitAttack {
    Melee(Entity),
    // TODO: Implement this.
    #[allow(dead_code)]
    Ranged(Entity),
}

/// [`EntityEvent`] that is triggered if the contained [`Entity`]'s next [`Attack`] should be delayed.
#[derive(EntityEvent)]
pub(crate) struct DelayAttack {
    pub(crate) entity: Entity,
    pub(crate) cooldown_secs: f32,
}

/// Stats for [`Attack`]
#[derive(Component, Default)]
pub(crate) struct AttackStats {
    pub(crate) _attacks: HashSet<AttackData>,
    pub(crate) damage_factor: f32,
    pub(crate) melee: Option<AttackData>,
    pub(crate) _ranged: Option<AttackData>,
}

/// Timer that tracks [`Attack`]s
#[derive(Component, Debug, Clone, PartialEq, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub(crate) struct AttackTimer(pub(crate) Timer);

/// Simple punch [`Attack`] with short range
pub(crate) fn punch() -> AttackData {
    AttackData {
        name: "punch",
        damage: OrderedFloat(1.),
        range: (OrderedFloat(8.), OrderedFloat(16.)),
        cooldown_secs: OrderedFloat(0.5),
    }
}

/// On [`Attack::Melee`], trigger [`Damage`] on [Entity]s within range.
pub(super) fn on_melee_attack<T>(
    mut reader: MessageReader<Attack>,
    target_query: Query<&Health>,
    origin_query: Query<(&Transform, &AimDirection, &AttackStats), With<T>>,
    mut commands: Commands,
    collision_data: Res<CollisionDataCache<T>>,
    rapier_context: ReadRapierContext,
    particle_handle: Res<ParticleHandle<ParticleMeleeAttack>>,
) where
    T: Visible,
{
    for attack in reader.read() {
        let Attack::Melee(entity) = attack else {
            continue;
        };

        let rapier_context = rapier_context.single().expect(ERR_INVALID_RAPIER_CONTEXT);
        let (transform, direction, stats) = origin_query.get(*entity).expect(ERR_INVALID_ATTACKER);
        let Some(melee) = &stats.melee else {
            warn_once!("{}", WARN_INVALID_ATTACK_DATA);
            return;
        };

        // Cast ray to determine boundary of `Collider`
        // NOTE: We have to add an offset to max_toi to ensure that the ray reaches the boundary.
        let max_toi = (collision_data.width / 2.).max(collision_data.height / 2.) + 1.;
        // Filter for the source itself
        let filter = &|e| e == *entity;
        let filter = QueryFilter::exclude_dynamic()
            .exclude_sensors()
            .predicate(filter);
        let pos = transform.translation.xy();
        let Some((_, extent)) = rapier_context.cast_ray(pos, direction.0, max_toi, false, filter)
        else {
            return;
        };

        // Collect all entities within attack range
        let shape_half_size = Vec2::new(*melee.range.0, *melee.range.1) / 2.;
        let offset = direction.0 * (extent + shape_half_size.x);
        let shape_pos = pos + offset;
        let shape_rot = direction.0.to_angle();
        let shape = shape::Cuboid::new(shape_half_size.into());
        // Filter for anything that is not the source
        let filter = QueryFilter::exclude_dynamic()
            .exclude_sensors()
            .exclude_rigid_body(*entity);
        let mut targets = Vec::new();
        rapier_context.intersect_shape(shape_pos, shape_rot, &shape, filter, |e| {
            if target_query.contains(e) {
                targets.push(e);
            }
            true
        });

        // Apply attack
        let damage = stats.damage_factor * *melee.damage;
        commands.trigger(Damage { targets, damage });
        commands.trigger(SpawnChildParticleOnce::<ParticleMeleeAttack>::new(
            *entity,
            offset.extend(OVERLAY_Z),
            particle_handle.handle.clone(),
        ));
    }
}

/// Insert [`AttackTimer`] to delay [`Attack`]s.
pub(super) fn on_delay_attack(event: On<DelayAttack>, mut commands: Commands) {
    // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
    commands
        .entity(event.entity)
        .try_insert(AttackTimer(Timer::from_seconds(
            event.cooldown_secs,
            TimerMode::Once,
        )));
}
