use bevy::prelude::{EventWriter, Query};
use networking::server::OutgoingReliableServerMessage;

use player::net::PlayerServerMessage;

use networking::server::ConnectedPlayer;
use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::controller::ControllerInput;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MovementInput {
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub pressed: bool,
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ControllerClientMessage {
    MovementInput(MovementInput),
    SyncControllerInput(ControllerInput),
}
/// Update player count info for clients.

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
