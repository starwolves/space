use bevy::prelude::{EventWriter, Query, Vec2};
use networking::server::{OutgoingReliableServerMessage, UIInputAction};

use player::net::PlayerServerMessage;

use networking::server::ConnectedPlayer;
use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::networking::UIInputNodeClass;

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ControllerUnreliableClientMessage {
    MouseDirectionUpdate(f32, u64),
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ControllerClientMessage {
    UIInput(UIInputNodeClass, UIInputAction, String, String),
    UIInputTransmitData(String, String, String),
    MovementInput(Vec2),
    SprintInput(bool),
    BuildGraphics,
    ToggleCombatModeInput,
    InputMouseAction(bool),
    SelectBodyPart(String),
    ToggleAutoMove,
    AttackEntity(u64),
    AltItemAttack,
    AttackCell(i16, i16, i16),
}
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
