use bevy_app::{EventReader, EventWriter};
use bevy_ecs::{
    entity::Entity,
    system::{Query, Res},
};
use bevy_log::warn;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space::core::{
    entity::{
        components::{EntityData, Examinable, Sensable},
        resources::EntityDataResource,
    },
    gridmap::{
        functions::gridmap_functions::cell_id_to_world,
        resources::{to_doryen_coordinates, GridmapData, GridmapDetails1, GridmapMain},
    },
    inventory::components::Inventory,
    networking::resources::{GridMapType, ReliableServerMessage},
    pawn::{
        components::{Pawn, Senser},
        events::{InputTabDataEntity, InputTabDataMap, NetTabData},
    },
};

pub fn tab_data(
    mut entity_events: EventReader<InputTabDataEntity>,
    mut map_events: EventReader<InputTabDataMap>,
    mut net: EventWriter<NetTabData>,

    pawn_query: Query<(&Pawn, &Senser, &RigidBodyPositionComponent, &Inventory)>,
    examinable_query: Query<(&Examinable, &Sensable, &RigidBodyPositionComponent)>,
    gridmap_data: Res<GridmapData>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    entity_data_resource: Res<EntityDataResource>,
    entity_datas: Query<&EntityData>,
) {
    for event in entity_events.iter() {
        let player_pawn_component;
        let pawn_body_position: Vec3;
        let player_inventory_component;

        match pawn_query.get(event.player_entity) {
            Ok((pawn_c, _pawn_c2, pawn_c3, pawn_c4)) => {
                player_pawn_component = pawn_c;
                pawn_body_position = pawn_c3.position.translation.into();
                player_inventory_component = pawn_c4;
            }
            Err(_rr) => {
                warn!("Couldn't find Pawn component belonging to player.");
                continue;
            }
        }

        let mut tab_data = vec![];

        for (_action_id, tab_action) in player_pawn_component.tab_actions.iter() {
            let entity = Entity::from_bits(event.examine_entity_bits);

            let s = Some(event.examine_entity_bits);

            match examinable_query.get(entity) {
                Ok((examinable_component, sensable_component, rigid_body_position_component)) => {
                    if sensable_component.sensed_by.contains(&event.player_entity) {
                        if (tab_action.prerequisite_check)(
                            tab_action.belonging_entity,
                            s,
                            None,
                            pawn_body_position.distance(
                                rigid_body_position_component.position.translation.into(),
                            ),
                            player_inventory_component,
                            &entity_data_resource,
                            &entity_datas,
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

        net.send(NetTabData {
            handle: event.handle,
            message: ReliableServerMessage::TabData(tab_data),
        });
    }

    for event in map_events.iter() {
        let player_pawn_component;
        let player_senser_component;
        let player_body_position: Vec3;
        let player_inventory_component;

        match pawn_query.get(event.player_entity) {
            Ok((pawn_c, pawn_c2, pawn_c3, pawn_c4)) => {
                player_pawn_component = pawn_c;
                player_senser_component = pawn_c2;
                player_body_position = pawn_c3.position.translation.into();
                player_inventory_component = pawn_c4;
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

            match event.gridmap_type {
                GridMapType::Main => {
                    this_map = &gridmap_main.grid_data;
                }
                GridMapType::Details1 => {
                    this_map = &gridmap_details1.data;
                }
            }

            let tab_data_name;
            let mut cell_item = None;
            match this_map.get(&event.gridmap_cell_id) {
                Some(cell_data) => {
                    cell_item = Some(cell_data);
                    match gridmap_data.main_text_names.get(&cell_data.item) {
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
                ) {
                    tab_data.push(tab_action.into_net(tab_data_name, None, cell_part_tuple));
                }
            }
        }

        net.send(NetTabData {
            handle: event.handle,
            message: ReliableServerMessage::TabData(tab_data),
        });
    }
}
