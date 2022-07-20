use std::sync::Arc;

use bevy::prelude::{Entity, Query};
use shared::{
    data::EntityDataResource,
    data_link::DataLink,
    entity_updates::EntityData,
    gridmap::{CellData, GridMapType},
    inventory::Inventory,
    pawn::REACH_DISTANCE,
    tab_actions::TabAction,
};

pub fn get_tab_action(id: &str) -> Option<TabAction> {
    let result;

    if id == "actions::pawn/examine" {
        result = Some(TabAction {
            id: id.to_string(),
            text: "Examine".to_string(),
            tab_list_priority: u8::MAX,
            prerequisite_check: Arc::new(examine_tab_prerequisite_check),
            belonging_entity: None,
        });
    } else if id == "actions::inventory/pickup" {
        result = Some(TabAction {
            id: id.to_string(),
            text: "Pickup".to_string(),
            tab_list_priority: u8::MAX - 1,
            prerequisite_check: Arc::new(pickup_tab_prerequisite_check),
            belonging_entity: None,
        });
    } else {
        result = None;
    }

    result
}

pub fn examine_tab_prerequisite_check(
    _self_tab_entity: Option<Entity>,
    entity_id_bits_option: Option<u64>,
    cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    _distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    cell_id_option.is_some() || entity_id_bits_option.is_some()
}

pub fn pickup_tab_prerequisite_check(
    _self_tab_entity: Option<Entity>,
    entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    distance < REACH_DISTANCE
        && entity_id_bits_option.is_some()
        && inventory_component.get_active_slot_entity().is_none()
}
