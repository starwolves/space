use bevy::{
    math::Vec3,
    prelude::{EventWriter, Query},
};
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

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum PeerControllerClientMessage {
    MovementInput(MovementInput, Vec3, Vec3),
    SyncControllerInput(ControllerInput),
}

impl PeerControllerClientMessage {
    pub fn from(message: ControllerClientMessage, position: Vec3, target: Vec3) -> Self {
        match message {
            ControllerClientMessage::MovementInput(i) => {
                PeerControllerClientMessage::MovementInput(i, position, target)
            }
            ControllerClientMessage::SyncControllerInput(i) => {
                PeerControllerClientMessage::SyncControllerInput(i)
            }
        }
    }
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
