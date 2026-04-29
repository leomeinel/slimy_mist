use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalRng, traits::ForkableSeed as _};

/// Applies to any rng that is forked from [`GlobalRng`]
pub(crate) trait ForkedRng
where
    Self: Component + Default,
{
}

/// Spawn [`ForkedRng`] by forking [`GlobalRng`]
pub(crate) fn setup_rng<T>(mut global: Single<&mut WyRand, With<GlobalRng>>, mut commands: Commands)
where
    T: ForkedRng,
{
    commands.spawn((T::default(), global.fork_seed()));
}
