use bevy::prelude::{Entity, EventReader, EventWriter, Query, Res, warn};

use crate::space_core::{components::{examinable::Examinable, pawn::Pawn, sensable::Sensable, senser::Senser}, events::{general::{tab_data_entity::InputTabDataEntity, tab_data_map::InputTabDataMap}, net::net_tab_data_entity::NetTabData}, resources::{doryen_fov::to_doryen_coordinates, gridmap_data::GridmapData, gridmap_details1::GridmapDetails1, gridmap_main::GridmapMain, network_messages::ReliableServerMessage}};

pub fn tab_data(

    mut entity_events : EventReader<InputTabDataEntity>,
    mut map_events : EventReader<InputTabDataMap>,
    mut net : EventWriter<NetTabData>,

    pawn_query : Query<(&Pawn, &Senser)>,
    examinable_query : Query<(&Examinable, &Sensable)>,
    gridmap_data : Res<GridmapData>,
    gridmap_main : Res<GridmapMain>,
    gridmap_details1 : Res<GridmapDetails1>,

) {


    for event in entity_events.iter() {

        let player_pawn_component;

        match pawn_query.get(event.player_entity) {
            Ok((pawn_c, _pawn_c2)) => {
                player_pawn_component=pawn_c;
            },
            Err(_rr) => {
                warn!("Couldn't find Pawn component belonging to player.");
                continue;
            },
        }

        let mut tab_data = vec![];

        for (_action_id, tab_action) in player_pawn_component.tab_actions.iter() {

            let entity = Entity::from_bits(event.examine_entity_bits);
            
            let s = Some(event.examine_entity_bits);

            match examinable_query.get(entity) {
                Ok((examinable_component, sensable_component)) => {
                    if sensable_component.sensed_by.contains(&event.player_entity) {
                        if (tab_action.prerequisite_check)(s, None) {
                            tab_data.push(tab_action.into_net(examinable_component.name.get_name(), s,None));
                        }
                    }
                },
                Err(_rr) => {},
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

        match pawn_query.get(event.player_entity) {
            Ok((pawn_c, pawn_c2)) => {
                player_pawn_component=pawn_c;
                player_senser_component=pawn_c2;
            },
            Err(_rr) => {
                warn!("Couldn't find Pawn component belonging to player (2).");
                continue;
            },
        }

        let mut tab_data = vec![];

        for (_action_id, tab_action) in player_pawn_component.tab_actions.iter() {
            let s = Some((event.gridmap_type.clone(), event.gridmap_cell_id.x, event.gridmap_cell_id.y,event.gridmap_cell_id.z));

            let doryen_coords = to_doryen_coordinates(event.gridmap_cell_id.x, event.gridmap_cell_id.z);

            match event.gridmap_type {
                crate::space_core::resources::network_messages::GridMapType::Main => {
                    match gridmap_main.data.get(&event.gridmap_cell_id) {
                        Some(cell_data) => {
                            
                            match gridmap_data.main_text_names.get(&cell_data.item) {
                                Some(cell_name) => {
                                    if player_senser_component.fov.is_in_fov(doryen_coords.0, doryen_coords.1) {
                                        if (tab_action.prerequisite_check)(None, s.clone()) {
                                            tab_data.push(tab_action.into_net(cell_name.get_name(), None, s));
                                        }
                                    }
                                },
                                None => {},
                            }
                        },
                        None => {},
                    }
                },
                crate::space_core::resources::network_messages::GridMapType::Details1 => {
                    match gridmap_details1.data.get(&event.gridmap_cell_id) {
                        Some(cell_data) => {
                            match gridmap_data.details1_text_names.get(&cell_data.item) {
                                Some(cell_name) => {
                                    if player_senser_component.fov.is_in_fov(doryen_coords.0, doryen_coords.1) {
                                        if (tab_action.prerequisite_check)(None, s.clone()) {
                                            tab_data.push(tab_action.into_net(cell_name.get_name(), None, s));
                                        }
                                    }
                                },
                                None => {},
                            }
                        },
                        None => {},
                    }
                },
            }

            

        }

        net.send(NetTabData {
            handle: event.handle,
            message: ReliableServerMessage::TabData(tab_data),
        });

    }

}
