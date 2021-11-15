use bevy::{prelude::{EventWriter, Query}};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::net::net_send_server_time::NetSendServerTime, resources::network_messages::{ReliableServerMessage, ServerConfigMessage}};

pub fn send_server_time(
    mut event_writer : EventWriter<NetSendServerTime>,
    connected_players : Query<&ConnectedPlayer>,
) {

    for connected_player_component in connected_players.iter() {

        if !connected_player_component.connected {
            continue;
        }

        event_writer.send(NetSendServerTime {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerTime),
        });

    }


}

