use bevy_app::EventWriter;
use bevy_ecs::system::Query;

use crate::space::core::{
    connected_player::{components::ConnectedPlayer, events::NetSendServerTime},
    networking::resources::{ReliableServerMessage, ServerConfigMessage},
};

pub fn send_server_time(
    mut event_writer: EventWriter<NetSendServerTime>,
    connected_players: Query<&ConnectedPlayer>,
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
