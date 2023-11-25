use bevy::log::warn;
use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::With;
use networking::server::ConnectedPlayer;
use networking::server::IncomingEarlyReliableClientMessage;
use networking::server::IncomingEarlyUnreliableClientMessage;
use networking::server::OutgoingReliableServerMessage;
use networking::server::OutgoingUnreliableServerMessage;
use pawn::net::UnreliableControllerClientMessage;
use physics::entity::RigidBodyLink;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::input::InputMovementInput;

use networking::server::HandleToEntity;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum UIInputNodeClass {
    Button,
}

use crate::net::ControllerClientMessage;
use bevy::prelude::EventReader;
use networking::server::IncomingReliableClientMessage;

/// Replicates client input to peers this is a server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub struct PeerReliableControllerMessage {
    pub message: ControllerClientMessage,
    pub peer_handle: u16,
    pub client_stamp: u8,
}
/// Replicates client input to peers this is a server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub struct PeerUnreliableControllerMessage {
    pub message: UnreliableControllerClientMessage,
    pub peer_handle: u16,
    pub client_stamp: u8,
}

/// Replicate client input to peers instantly.
pub(crate) fn peer_replication(
    mut server: EventReader<IncomingEarlyReliableClientMessage<ControllerClientMessage>>,
    mut u_server: EventReader<
        IncomingEarlyUnreliableClientMessage<UnreliableControllerClientMessage>,
    >,
    mut peer: EventWriter<OutgoingReliableServerMessage<PeerReliableControllerMessage>>,
    mut peer_unreliable: EventWriter<
        OutgoingUnreliableServerMessage<PeerUnreliableControllerMessage>,
    >,
    players: Query<&ConnectedPlayer, With<RigidBodyLink>>,
) {
    for message in server.read() {
        for connected in players.iter() {
            if !connected.connected {
                continue;
            }
            if message.handle == connected.handle {
                continue;
            }
            //info!("Sending {:?} tick {}", message.message, message.stamp);
            peer.send(OutgoingReliableServerMessage {
                handle: connected.handle,
                message: PeerReliableControllerMessage {
                    message: message.message.clone(),
                    peer_handle: message.handle.raw() as u16,
                    client_stamp: message.stamp,
                },
            });
        }
    }
    for message in u_server.read() {
        for connected in players.iter() {
            if !connected.connected {
                continue;
            }
            if message.handle == connected.handle {
                continue;
            }

            peer_unreliable.send(OutgoingUnreliableServerMessage {
                handle: connected.handle,
                message: PeerUnreliableControllerMessage {
                    message: message.message.clone(),
                    peer_handle: message.handle.raw() as u16,
                    client_stamp: message.stamp,
                },
            });
        }
    }
}

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ControllerClientMessage>>,
    mut movement_input_event: EventWriter<InputMovementInput>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for message in server.read() {
        let client_message = message.message.clone();

        match client_message {
            ControllerClientMessage::MovementInput(movement_input) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        movement_input_event.send(InputMovementInput {
                            player_entity: *player_entity,
                            pressed: movement_input.pressed,
                            up: movement_input.up,
                            left: movement_input.left,
                            right: movement_input.right,
                            down: movement_input.down,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                }
            }
        }
    }
}
