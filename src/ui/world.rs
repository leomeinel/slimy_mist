use bevy::prelude::*;

pub(super) mod health;

use crate::{characters::prelude::*, screens::prelude::*};

pub(super) struct WorldUiPlugin;
impl Plugin for WorldUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), spawn_world_ui);

        app.add_systems(
            Update,
            (
                health::spawn_health_bar::<Slime>,
                health::despawn_health_bar::<Slime>,
                health::update_health_bar::<Slime>,
                health::move_and_scale_health_bar::<Slime>,
            )
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

/// UI that contains elements mapped to the world.
#[derive(Component)]
pub(crate) struct WorldUi;

/// Anchor points for [`WorldUi`] to be able to display elements correctly.
#[derive(Component)]
pub(crate) enum WorldUiAnchor {
    HealthBar(Vec2),
}

/// Spawn [`WorldUi`].
pub(super) fn spawn_world_ui(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        WorldUi,
        Pickable::IGNORE,
        GlobalZIndex(-1),
    ));
}
