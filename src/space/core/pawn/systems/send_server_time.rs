use bevy_app::EventWriter;
use bevy_ecs::system::Query;

use crate::space::core::{
    networking::resources::{ReliableServerMessage, ServerConfigMessage},
    pawn::{components::ConnectedPlayer, events::NetSendServerTime},
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
