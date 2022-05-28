use bevy_ecs::{entity::Entity, system::Query};

use crate::core::{
    data_link::components::DataLink,
    entity::{components::EntityData, resources::EntityDataResource},
    gridmap::resources::CellData,
    inventory::components::Inventory,
    networking::resources::GridMapType,
    pawn::functions::can_reach_entity::REACH_DISTANCE,
};

pub fn construct_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    distance < REACH_DISTANCE && cell_id_option.is_some()
}

pub fn deconstruct_action(
    _self_tab_entity: Option<Entity>,
    entity_id_bits_option: Option<u64>,
    cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    entity_data_resource: &EntityDataResource,
    entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    match entity_id_bits_option {
        Some(bits) => {
            let entity = Entity::from_bits(bits);

            let mut deconstructable = false;

            match entity_datas.get(entity) {
                Ok(entity_data) => {
                    let entity_properties = entity_data_resource
                        .data
                        .get(
                            *entity_data_resource
                                .name_to_id
                                .get(&entity_data.entity_name)
                                .unwrap(),
                        )
                        .unwrap();

                    deconstructable = entity_properties.grid_item.is_some();
                }
                Err(_) => {}
            }

            distance < REACH_DISTANCE && deconstructable
        }
        None => {
            distance < REACH_DISTANCE
                && cell_id_option.is_some()
                && cell_id_option.unwrap().4.is_some()
        }
    }
}

pub fn construction_option_action(
    self_tab_entity_option: Option<Entity>,
    belonging_entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    _distance: f32,
    inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    let is_self;

    match belonging_entity_id_bits_option {
        Some(e) => {
            let entity = Entity::from_bits(e);

            match self_tab_entity_option {
                Some(self_tab_entity) => {
                    if self_tab_entity != entity {
                        is_self = false;
                    } else {
                        if inventory_component.has_item(entity) {
                            is_self = true;
                        } else {
                            is_self = false;
                        }
                    }
                }
                None => {
                    is_self = false;
                }
            }
        }
        None => {
            is_self = false;
        }
    }

    is_self
}
