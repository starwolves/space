use api::{
    data::{EntityDataResource, HandleToEntity},
    data_link::DataLink,
    entity_updates::EntityData,
    examinable::Examinable,
    gridmap::{
        cell_id_to_world, to_doryen_coordinates, GridMapType, GridmapData, GridmapDetails1,
        GridmapMain,
    },
    inventory::Inventory,
    network::ReliableServerMessage,
    sensable::Sensable,
    senser::Senser,
    tab_actions::TabActions,
};
use bevy::{
    math::Vec3,
    prelude::{warn, Entity, EventReader, EventWriter, Query, Res, Transform},
};
use networking::{
    messages::{InputTabDataEntity, InputTabDataMap},
    plugin::NetTabData,
};
use pawn::pawn::Pawn;

pub fn tab_data(
    mut entity_events: EventReader<InputTabDataEntity>,
    mut map_events: EventReader<InputTabDataMap>,
    mut net: EventWriter<NetTabData>,
    pawn_query: Query<(&Pawn, &Senser, &Transform, &Inventory, &DataLink)>,
    examinable_query: Query<(&Examinable, &Sensable, &Transform, Option<&TabActions>)>,
    gridmap_data: Res<GridmapData>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    entity_data_resource: Res<EntityDataResource>,
    entity_datas: Query<&EntityData>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in entity_events.iter() {
        let player_pawn_component;
        let pawn_body_position: Vec3;
        let player_inventory_component;
        let data_link_component;

        match pawn_query.get(event.player_entity) {
            Ok((pawn_c, _pawn_c2, pawn_c3, pawn_c4, pawn_c5)) => {
                player_pawn_component = pawn_c;
                pawn_body_position = pawn_c3.translation.into();
                player_inventory_component = pawn_c4;
                data_link_component = pawn_c5;
            }
            Err(_rr) => {
                warn!("Couldn't find Pawn component belonging to player.");
                continue;
            }
        }

        let mut tab_data = vec![];

        let mut tab_actions = vec![];

        let entity = Entity::from_bits(event.examine_entity_bits);

        for (_action_id, tab_action) in player_pawn_component.tab_actions.iter() {
            tab_actions.push(tab_action);
        }

        match examinable_query.get(entity) {
            Ok((
                _examinable_component,
                _sensable_component,
                _rigid_body_position_component_option,
                tab_actions_component_option,
            )) => match tab_actions_component_option {
                Some(tab_actions_component) => {
                    for tab_action in tab_actions_component.tab_actions.iter() {
                        tab_actions.push(tab_action);
                    }
                }
                None => {}
            },
            Err(_rr) => {}
        }

        for tab_action in tab_actions {
            let s = Some(event.examine_entity_bits);

            match examinable_query.get(entity) {
                Ok((
                    examinable_component,
                    sensable_component,
                    rigid_body_position_component,
                    _tab_actions_component_option,
                )) => {
                    let entity_translation = rigid_body_position_component.translation;

                    if sensable_component.sensed_by.contains(&event.player_entity) {
                        if (tab_action.prerequisite_check)(
                            tab_action.belonging_entity,
                            s,
                            None,
                            pawn_body_position.distance(entity_translation),
                            player_inventory_component,
                            &entity_data_resource,
                            &entity_datas,
                            &data_link_component,
                        ) {
                            tab_data.push(tab_action.into_net(
                                examinable_component.name.get_name(),
                                s,
                                None,
                            ));
                        }
                    }
                }
                Err(_rr) => {}
            }
        }

        match handle_to_entity.inv_map.get(&event.player_entity) {
            Some(handle) => {
                net.send(NetTabData {
                    handle: *handle,
                    message: ReliableServerMessage::TabData(tab_data),
                });
            }
            None => {}
        }
    }

    for event in map_events.iter() {
        let player_pawn_component;
        let player_senser_component;
        let player_body_position: Vec3;
        let player_inventory_component;
        let data_link_component;

        match pawn_query.get(event.player_entity) {
            Ok((pawn_c, pawn_c2, pawn_c3, pawn_c4, pawn_c5)) => {
                player_pawn_component = pawn_c;
                player_senser_component = pawn_c2;
                player_body_position = pawn_c3.translation;
                player_inventory_component = pawn_c4;
                data_link_component = pawn_c5;
            }
            Err(_rr) => {
                warn!("Couldn't find Pawn component belonging to player (2).");
                continue;
            }
        }

        let mut tab_data = vec![];

        for (_action_id, tab_action) in player_pawn_component.tab_actions.iter() {
            let cell_part_tuple = Some((
                event.gridmap_type.clone(),
                event.gridmap_cell_id.x,
                event.gridmap_cell_id.y,
                event.gridmap_cell_id.z,
            ));

            let cell_world_position = cell_id_to_world(event.gridmap_cell_id);

            let doryen_coords =
                to_doryen_coordinates(event.gridmap_cell_id.x, event.gridmap_cell_id.z);

            let this_map;
            let this_names;

            match event.gridmap_type {
                GridMapType::Main => {
                    this_map = &gridmap_main.grid_data;
                    this_names = &gridmap_data.main_text_names;
                }
                GridMapType::Details1 => {
                    this_map = &gridmap_details1.data;
                    this_names = &gridmap_data.details1_text_names;
                }
            }

            let tab_data_name;
            let mut cell_item = None;
            match this_map.get(&event.gridmap_cell_id) {
                Some(cell_data) => {
                    cell_item = Some(cell_data);
                    match this_names.get(&cell_data.item) {
                        Some(cell_name) => {
                            tab_data_name = cell_name.get_name();
                        }
                        None => {
                            tab_data_name = "Space";
                        }
                    }
                }
                None => {
                    // Empty space, ie a space tile.
                    tab_data_name = "Space";
                }
            }

            let passed_cell_tuple;
            match &cell_part_tuple {
                Some(x) => {
                    passed_cell_tuple = Some((x.0.clone(), x.1, x.2, x.3, cell_item));
                }
                None => {
                    passed_cell_tuple = None;
                }
            }

            if player_senser_component
                .fov
                .is_in_fov(doryen_coords.0, doryen_coords.1)
            {
                if (tab_action.prerequisite_check)(
                    tab_action.belonging_entity,
                    None,
                    passed_cell_tuple,
                    player_body_position.distance(cell_world_position),
                    player_inventory_component,
                    &entity_data_resource,
                    &entity_datas,
                    &data_link_component,
                ) {
                    tab_data.push(tab_action.into_net(tab_data_name, None, cell_part_tuple));
                }
            }
        }

        match handle_to_entity.inv_map.get(&event.player_entity) {
            Some(handle) => {
                net.send(NetTabData {
                    handle: *handle,
                    message: ReliableServerMessage::TabData(tab_data),
                });
            }
            None => {}
        }
    }
}
