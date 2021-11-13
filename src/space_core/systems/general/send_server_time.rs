use std::time::{SystemTime};

use bevy::{prelude::{EventWriter, Query}};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::net::net_send_server_time::NetSendServerTime, resources::network_messages::{ReliableServerMessage, ServerConfigMessage}};

pub fn send_server_time(
    mut event_writer : EventWriter<NetSendServerTime>,
    connected_players : Query<&ConnectedPlayer>,
) {

    let current_time_stamp;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(s) => {
            current_time_stamp = s.as_millis();
        },
        Err(_rr) => {return;},
    };

    for connected_player_component in connected_players.iter() {

        if !connected_player_component.connected {
            continue;
        }

        event_writer.send(NetSendServerTime {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerTime(current_time_stamp)),
        });

    }


}
