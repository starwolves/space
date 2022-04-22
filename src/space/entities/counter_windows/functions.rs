use bevy_ecs::{entity::Entity, system::Query};

use crate::space::core::{
    data_link::components::{DataLink, DataLinkType},
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
    _data_link_component: &DataLink,
) -> bool {
    distance < 3.
}

pub fn lock_open_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    data_link_component: &DataLink,
) -> bool {
    distance < 30.
        && data_link_component
            .links
            .contains(&DataLinkType::RemoteLock)
}

pub fn lock_closed_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    data_link_component: &DataLink,
) -> bool {
    distance < 30.
        && data_link_component
            .links
            .contains(&DataLinkType::RemoteLock)
}
