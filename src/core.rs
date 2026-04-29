pub(crate) mod prelude {
    pub(crate) use super::{AppSystems, MIN_SIDE_SCALE_THRESHOLD, PausableSystems, Pause};
}

use bevy::prelude::*;

pub(super) struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Pause>();

        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );
    }
}

/// Scale threshold for the minimum length of a window side.
///
/// This is used for scaling [`UiScale`] and [`Projection`].
pub(crate) const MIN_SIDE_SCALE_THRESHOLD: f32 = 720.;

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(crate) enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Tracks whether the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) struct Pause(pub(crate) bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct PausableSystems;
