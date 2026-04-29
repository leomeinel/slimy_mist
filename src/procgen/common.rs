use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{levels::prelude::*, procgen::prelude::*, render::prelude::*};

/// Spawn objects in every chunk contained in [`ProcGenCache<A>`]
pub(super) fn spawn_objects<T, A, B>(
    level: Single<Entity, With<B>>,
    mut commands: Commands,
    chunk_cache: Res<ProcGenCache<A>>,
    object_cache: Res<ProcGenCache<T>>,
) where
    T: ProcGenerated,
    A: ProcGenerated,
    B: Level,
{
    for (_, chunk_pos) in &chunk_cache.chunk_positions {
        // Continue if chunk has already been stored
        if object_cache
            .chunk_positions
            .values()
            .any(|&v| v == *chunk_pos)
        {
            continue;
        }

        commands.trigger(ProcGen::<T> {
            entity: *level,
            chunk_pos: *chunk_pos,
            _phantom: PhantomData,
        });
    }
}

/// Collect procedurally generated [`Entity`]s to despawn outside of [`PROCGEN_DISTANCE`].
///
/// `const PROCEED` determines whether we should proceed to the next state.
pub(super) fn collect_to_despawn<T, A, const PROCEED: bool>(
    camera: Single<&Transform, (Changed<Transform>, With<CanvasCamera>, Without<T>)>,
    query: Query<(Entity, &Transform), (With<T>, Without<CanvasCamera>)>,
    mut cache: ResMut<ProcGenCache<T>>,
    mut next_state: ResMut<NextState<ProcGenState>>,
    level_dimensions: Res<LevelDimensions<A>>,
) where
    T: ProcGenerated,
    A: ProcGenerated,
{
    let chunk_size_px = level_dimensions.chunk_size_px;
    cache.camera_chunk_pos = (camera.translation.xy() / chunk_size_px).floor().as_ivec2();

    // Add entities outside of `PROCGEN_DISTANCE` to `to_despawn`
    for (entity, transform) in query {
        let chunk_pos = (transform.translation.xy() / chunk_size_px)
            .floor()
            .as_ivec2();

        // NOTE: We are using `chebyshev_distance` because we are spawning in a square.
        if cache.camera_chunk_pos.chebyshev_distance(chunk_pos) > PROCGEN_DISTANCE as u32 {
            cache.to_despawn.insert(entity);
        }
    }

    // Transition state if required
    if PROCEED && (!cache.to_despawn.is_empty() || query.is_empty()) {
        (*next_state).set_if_neq(ProcGenState::Spawn);
    }
}

/// Despawn procedurally generated [`Entity`]s from [`ProcGenCache<T>::to_despawn`] and remove entries in [`ProcGenCache<T>::chunk_positions`].
pub(super) fn despawn<T>(
    mut commands: Commands,
    mut cache: ResMut<ProcGenCache<T>>,
    mut next_state: ResMut<NextState<DespawnProcGen>>,
) where
    T: ProcGenerated,
{
    // Despawn collected entities
    let entities: Vec<_> = cache.to_despawn.drain().collect();
    for entity in entities {
        cache.chunk_positions.remove(&entity);
        // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
        commands.entity(entity).try_despawn();
    }

    (*next_state).set_if_neq(DespawnProcGen(false));
}

/// Enable [`DespawnProcGen`] if [`ProcGenCache<T>::to_despawn`] is not empty.
pub(super) fn set_despawning<T>(
    mut next_state: ResMut<NextState<DespawnProcGen>>,
    cache: Res<ProcGenCache<T>>,
) where
    T: ProcGenerated,
{
    if !cache.to_despawn.is_empty() {
        (*next_state).set_if_neq(DespawnProcGen(true));
    }
}
