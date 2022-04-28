use bevy_ecs::{
    entity::Entity,
    prelude::Added,
    system::{Query, ResMut},
};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::{
    core::{
        entity::components::{DefaultMapEntity, EntityData},
        gridmap::{
            functions::gridmap_functions::world_to_cell_id,
            resources::{EntityGridData, GridmapMain, Vec2Int},
        },
        map::resources::{MapData, GREEN_MAP_TILE_ENTRANCE},
    },
    entities::air_locks::components::AirLock,
};

pub fn air_lock_default_map_added(
    airlock_windows: Query<
        (
            Entity,
            &RigidBodyPositionComponent,
            &DefaultMapEntity,
            &EntityData,
        ),
        Added<AirLock>,
    >,
    mut map_data: ResMut<MapData>,
    mut gridmap_main: ResMut<GridmapMain>,
) {
    for (airlock_entity, rigid_body_position_component, _, entity_data_component) in
        airlock_windows.iter()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        map_data.data.insert(cell_id2, GREEN_MAP_TILE_ENTRANCE);

        gridmap_main.entity_data.insert(
            cell_id,
            EntityGridData {
                entity: airlock_entity,
                entity_name: entity_data_component.entity_name.to_string(),
            },
        );
    }
}
