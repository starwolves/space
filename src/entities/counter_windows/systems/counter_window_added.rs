use bevy_ecs::{
    entity::Entity,
    prelude::Added,
    system::{Query, ResMut},
};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::{
    core::{
        atmospherics::{functions::get_atmos_index, resources::AtmosphericsResource},
        gridmap::{functions::gridmap_functions::world_to_cell_id, resources::Vec2Int},
    },
    entities::counter_windows::components::CounterWindow,
};

pub fn counter_window_added(
    counter_windows: Query<(Entity, &RigidBodyPositionComponent), Added<CounterWindow>>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (_airlock_entity, rigid_body_position_component) in counter_windows.iter() {
        let cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        if AtmosphericsResource::is_id_out_of_range(cell_id2) {
            continue;
        }
        let atmos_id = get_atmos_index(cell_id2);
        let atmospherics = atmospherics_resource
            .atmospherics
            .get_mut(atmos_id)
            .unwrap();

        atmospherics.blocked = true;
    }
}
