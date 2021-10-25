use bevy::prelude::{ EventReader, EventWriter, Query, warn};

use crate::space_core::{components::pawn::Pawn, events::{general::{tab_data_entity::InputTabDataEntity, tab_data_map::InputTabDataMap}, net::net_tab_data_entity::NetTabData}, resources::network_messages::ReliableServerMessage};

pub fn tab_data(

    mut entity_events : EventReader<InputTabDataEntity>,
    mut map_events : EventReader<InputTabDataMap>,
    pawn_query : Query<&Pawn>,

    mut net : EventWriter<NetTabData>,

) {


    for event in entity_events.iter() {

        let player_pawn_component;

        match pawn_query.get(event.player_entity) {
            Ok(pawn_c) => {
                player_pawn_component=pawn_c;
            },
            Err(_rr) => {
                warn!("Couldn't find Pawn component belonging to player.");
                continue;
            },
        }

        let mut tab_data = vec![];

        for tab_action in player_pawn_component.tab_actions.iter() {

            let s = Some(event.examine_entity_bits);

            if (tab_action.prerequisite_check)(s, None) {
                tab_data.push(tab_action.into_net(s,None));
            }
            
        }

        net.send(NetTabData {
            handle: event.handle,
            message: ReliableServerMessage::TabData(tab_data),
        });


    }

    for event in map_events.iter() {

        let player_pawn_component;

        match pawn_query.get(event.player_entity) {
            Ok(pawn_c) => {
                player_pawn_component=pawn_c;
            },
            Err(_rr) => {
                warn!("Couldn't find Pawn component belonging to player (2).");
                continue;
            },
        }

        let mut tab_data = vec![];

        for tab_action in player_pawn_component.tab_actions.iter() {

            let s = Some((event.gridmap_type.clone(), event.gridmap_cell_id.x, event.gridmap_cell_id.y,event.gridmap_cell_id.z));

            if (tab_action.prerequisite_check)(None, s.clone()) {
                tab_data.push(tab_action.into_net(None, s));
            }
            
        }

        net.send(NetTabData {
            handle: event.handle,
            message: ReliableServerMessage::TabData(tab_data),
        });

    }

}
