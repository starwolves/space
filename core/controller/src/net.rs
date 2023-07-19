use bevy::prelude::{EventWriter, Query};
use networking::server::{OutgoingReliableServerMessage, UIInputAction};

use player::net::PlayerServerMessage;

use networking::server::ConnectedPlayer;
use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::networking::UIInputNodeClass;

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ControllerUnreliableClientMessage {
    MouseDirectionUpdate(f32, u64),
}
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
    UIInput(UIInputNodeClass, UIInputAction, String, String),
    UIInputTransmitData(String, String, String),
    MovementInput(MovementInput),
    SprintInput(bool),
    BuildGraphics,
    ToggleCombatModeInput,
    InputMouseAction(bool),
    ToggleAutoMove,
    AttackEntity(u64),
    AltItemAttack,
    AttackCell(i16, i16, i16),
}
/// Send server time to clients for ping update.

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
