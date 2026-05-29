// FIXME: This seems like a very hacky approach, but seems to be somewhat common practice.
//        Generally however it is recommended to use textures to achieve this, but this approach
//        currently provides the most consistency.

use bevy::prelude::*;

use crate::{
    characters::prelude::*,
    log::prelude::*,
    render::prelude::*,
    ui::{prelude::*, world::WorldUiNode},
};

/// Marker [`Component`] for any health bar rendered to [`WorldUi`].
#[derive(Component)]
pub(super) struct WorldUiHealthBar;

/// Spawn [`WorldUiHealthBar`] for all added `T` and add to [`WorldUiMap`].
pub(super) fn spawn_health_bar<T>(
    container: Single<Entity, With<WorldUi>>,
    visible_query: Query<Entity, Added<T>>,
    mut commands: Commands,
    mut map: ResMut<WorldUiMap>,
) where
    T: Visible,
{
    for entity in visible_query {
        let bar = commands.spawn(health_bar()).id();
        commands.entity(*container).add_child(bar);
        map.0.entry(entity).or_default().insert(bar);
    }
}

/// Health bar showing the current [`Health`].
fn health_bar() -> impl Bundle {
    let bar = BarBuilder::default()
        .with_position_type(PositionType::Absolute)
        .with_bar_background(HEALTH_BAR_BACKGROUND)
        .build();

    (
        WorldUiNode::health_bar(),
        WorldUiHealthBar,
        Visibility::Hidden,
        bar,
    )
}

/// Update [`WorldUiHealthBar`] from [`Health`] of all `T`.
pub(super) fn update_health_bar<T>(
    mut bar_container_query: Query<(&mut Visibility, &Children), With<WorldUiHealthBar>>,
    mut node_query: Query<&mut Node>,
    health_query: Query<(Entity, &Health), (Changed<Health>, With<T>)>,
    children_query: Query<&Children>,
    map: Res<WorldUiMap>,
) where
    T: Visible,
{
    for (entity, health) in health_query {
        let Some(entities) = map.0.get(&entity) else {
            continue;
        };
        let Some(entity) = entities.iter().find(|e| bar_container_query.contains(**e)) else {
            continue;
        };
        let Ok((mut visibility, children)) = bar_container_query.get_mut(*entity) else {
            continue;
        };
        let child = children
            .iter()
            .find(|e| children_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let children = children_query.get(child).expect(ERR_INVALID_CHILDREN);
        let child = children
            .iter()
            .find(|e| node_query.contains(*e))
            .expect(ERR_INVALID_CHILDREN);
        let mut mask_node = node_query.get_mut(child).expect(ERR_INVALID_CHILDREN);

        let mask_percent = (1. - health.fraction()) * 100.;
        *visibility = if mask_percent > 0. {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        mask_node.width = percent(mask_percent);
    }
}
