use bevy_ecs::{entity::Entity, system::Query};

use crate::space::core::{
    entity::{components::EntityData, resources::EntityDataResource},
    gridmap::resources::CellData,
    inventory::components::Inventory,
    networking::resources::GridMapType,
};

pub fn toggle_open_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
) -> bool {
    distance < 3.
}
