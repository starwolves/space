use bevy::prelude::warn;
use bevy::prelude::Entity;
use bevy::prelude::EventWriter;
use bevy::prelude::Quat;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use bevy::prelude::Vec3;
use bevy::prelude::With;
use bevy_xpbd_3d::prelude::LinearVelocity;
use networking::server::ConnectedPlayer;
use networking::server::IncomingEarlyReliableClientMessage;
use networking::server::IncomingEarlyUnreliableClientMessage;
use networking::server::OutgoingReliableServerMessage;
use networking::server::OutgoingUnreliableServerMessage;
use networking::stamp::TickRateStamp;
use pawn::net::UnreliableControllerClientMessage;
use physics::entity::RigidBodies;
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
    pub data: EarlyTickData,
}
/// Replicates client input to peers this is a server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub struct PeerUnreliableControllerMessage {
    pub message: UnreliableControllerClientMessage,
    pub peer_handle: u16,
    pub client_stamp: u8,
    pub data: EarlyTickData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EarlyTickData {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub stamp: u8,
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
    players: Query<(Entity, &ConnectedPlayer), With<RigidBodyLink>>,
    rigidbody_query: Query<(&Transform, &LinearVelocity)>,
    rigidbodies: Res<RigidBodies>,
    stamp: Res<TickRateStamp>,
) {
    for message in server.iter() {
        for (entity, connected) in players.iter() {
            if !connected.connected {
                continue;
            }
            /*if message.handle == connected.handle {
                continue;
            }*/

            let rb_entity;
            match rigidbodies.get_entity_rigidbody(&entity) {
                Some(e) => {
                    rb_entity = *e;
                }
                None => {
                    warn!("Couldnt find rb.");
                    continue;
                }
            }

            match rigidbody_query.get(rb_entity) {
                Ok((transform, velocity)) => {
                    peer.send(OutgoingReliableServerMessage {
                        handle: connected.handle,
                        message: PeerReliableControllerMessage {
                            message: message.message.clone(),
                            peer_handle: message.handle as u16,
                            client_stamp: message.stamp,
                            data: EarlyTickData {
                                position: transform.translation,
                                rotation: transform.rotation,
                                velocity: velocity.0,
                                stamp: stamp.tick,
                            },
                        },
                    });
                }
                Err(_) => {
                    warn!("Couldnt find rb in query.");
                    continue;
                }
            }
        }
    }
    for message in u_server.iter() {
        for (entity, connected) in players.iter() {
            if !connected.connected {
                continue;
            }
            /*if message.handle == connected.handle {
                continue;
            }*/

            let rb_entity;
            match rigidbodies.get_entity_rigidbody(&entity) {
                Some(e) => {
                    rb_entity = *e;
                }
                None => {
                    warn!("Couldnt find rb.");
                    continue;
                }
            }

            match rigidbody_query.get(rb_entity) {
                Ok((transform, velocity)) => {
                    peer_unreliable.send(OutgoingUnreliableServerMessage {
                        handle: connected.handle,
                        message: PeerUnreliableControllerMessage {
                            message: message.message.clone(),
                            peer_handle: message.handle as u16,
                            client_stamp: message.stamp,
                            data: EarlyTickData {
                                position: transform.translation,
                                rotation: transform.rotation,
                                velocity: velocity.0,
                                stamp: stamp.tick,
                            },
                        },
                    });
                }
                Err(_) => {
                    warn!("Couldnt find rb in query.");
                    continue;
                }
            }
        }
    }
}

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ControllerClientMessage>>,
    mut movement_input_event: EventWriter<InputMovementInput>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for message in server.iter() {
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
