use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

pub(super) mod health;

use crate::{characters::prelude::*, render::prelude::*, screens::prelude::*};

pub(super) struct WorldUiPlugin;
impl Plugin for WorldUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), spawn_world_ui);

        app.add_systems(
            Update,
            (
                health::spawn_health_bar::<Slime>,
                despawn_world_ui_children::<Slime>,
                health::update_health_bar::<Slime>,
                move_and_scale_world_ui,
            )
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

/// UI that contains elements mapped to the world.
#[derive(Component)]
pub(crate) struct WorldUi;

/// Relevant [`Node`] fields for [`WorldUi`] children.
///
/// Any field of this is in world units and therefore meant to be in pixels.
#[derive(Component)]
pub(crate) struct WorldUiNode {
    width: Val,
    height: Val,
    padding: Val,
}
impl WorldUiNode {
    fn health_bar() -> Self {
        Self {
            width: Val::Px(10.5),
            height: Val::Px(3.),
            padding: Val::Px(0.75),
        }
    }
}

/// World [`Entity`]s mapped to their corresponding [`WorldUi`] child [`Entity`]s.
#[derive(Resource, Default)]
pub(crate) struct WorldUiMap(pub(crate) HashMap<Entity, HashSet<Entity>>);

/// Anchor points for [`WorldUi`] to be able to display elements correctly.
#[derive(Component)]
pub(crate) enum WorldUiAnchor {
    HealthBar(Vec2),
}

/// Spawn [`WorldUi`].
fn spawn_world_ui(mut commands: Commands) {
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

/// Despawn [`WorldUi`] children for all removed `T` and remove from [`WorldUiMap`].
fn despawn_world_ui_children<T>(
    mut removed_visible: RemovedComponents<T>,
    mut commands: Commands,
    mut map: ResMut<WorldUiMap>,
) where
    T: Visible,
{
    for entity in removed_visible.read() {
        let Some(entities) = map.0.remove(&entity) else {
            continue;
        };
        for entity in entities {
            // NOTE: Using try here is necessary since the entity might have been despawned elsewhere.
            commands.entity(entity).try_despawn();
        }
    }
}

/// Move and scale [`WorldUi`] children.
fn move_and_scale_world_ui(
    camera: Single<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    projection: Single<&Projection, With<CanvasCamera>>,
    mut bar_node_query: Query<(&mut Node, &ComputedNode, &Visibility, &WorldUiNode)>,
    transform_query: Query<(Entity, &GlobalTransform, &WorldUiAnchor)>,
    map: Res<WorldUiMap>,
    ui_scale: Res<UiScale>,
) {
    let (camera, camera_transform) = *camera;
    let Projection::Orthographic(projection) = &**projection else {
        return;
    };
    let world_ui_scale = 1. / (projection.scale * ui_scale.0);

    for (entity, transform, offset) in transform_query {
        let Some(entities) = map.0.get(&entity) else {
            continue;
        };
        let WorldUiAnchor::HealthBar(offset) = offset;
        let world_position = transform.translation().xy() + offset;
        let Ok(ui_position) = camera.world_to_viewport(camera_transform, world_position.extend(0.))
        else {
            continue;
        };

        for entity in entities {
            let Ok((mut bar_node, computed, visibility, world_ui_node)) =
                bar_node_query.get_mut(*entity)
            else {
                continue;
            };
            if visibility == Visibility::Hidden {
                continue;
            }

            bar_node.width = world_ui_node.width * world_ui_scale;
            bar_node.height = world_ui_node.height * world_ui_scale;
            bar_node.padding = UiRect::all(world_ui_node.padding * world_ui_scale);

            let x_offset = -(computed.size.x * computed.inverse_scale_factor) / 2.;
            bar_node.left = px(ui_position.x / ui_scale.0 + x_offset);
            bar_node.top = px(ui_position.y / ui_scale.0);
        }
    }
}
