use bevy::prelude::{EventWriter, Query};
use networking::server::OutgoingReliableServerMessage;

use player::connections::PlayerServerMessage;

use networking::server::ConnectedPlayer;

/// Send server time to clients for ping update.
#[cfg(feature = "server")]
pub(crate) fn send_server_time(
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    connected_players: Query<&ConnectedPlayer>,
) {
    for connected_player_component in connected_players.iter() {
        if !connected_player_component.connected {
            continue;
        }

        server.send(OutgoingReliableServerMessage {
            handle: connected_player_component.handle,
            message: PlayerServerMessage::ServerTime,
        });
    }
}

/// Update player count info for clients.
#[cfg(feature = "server")]
pub(crate) fn update_player_count(
    connected_players: Query<&ConnectedPlayer>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
) {
    let mut connected_players_amount: u16 = 0;

    for connected_player_component in connected_players.iter() {
        if connected_player_component.connected {
            connected_players_amount += 1;
        }
    }

    for connected_player_component in connected_players.iter() {
        if !connected_player_component.connected {
            continue;
        }
        server.send(OutgoingReliableServerMessage {
            handle: connected_player_component.handle,
            message: PlayerServerMessage::ConnectedPlayers(connected_players_amount),
        });
    }
}
