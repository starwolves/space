use std::collections::HashMap;

use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::ecs::system::ResMut;
use bevy::ecs::system::Resource;
use bevy::log::warn;
use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::With;
use bevy::transform::components::Transform;
use bevy_renet::renet::ClientId;
use bevy_renet::renet::RenetServer;
use cameras::LookTransform;
use networking::messaging::ReliableMessage;
use networking::messaging::ReliableServerMessageBatch;
use networking::messaging::Typenames;
use networking::messaging::UnreliableMessage;
use networking::messaging::UnreliableServerMessageBatch;
use networking::plugin::RENET_RELIABLE_ORDERED_ID;
use networking::plugin::RENET_UNRELIABLE_CHANNEL_ID;
use networking::server::ConnectedPlayer;
use networking::server::EarlyIncomingRawReliableClientMessage;
use networking::server::EarlyIncomingRawUnreliableClientMessage;
use networking::stamp::TickRateStamp;
use pawn::net::UnreliableControllerClientMessage;
use pawn::net::UnreliablePeerControllerClientMessage;
use physics::entity::RigidBodyLink;
use physics::sync::DisableSync;
use physics::sync::DESYNC_FREQUENCY;
use resources::core::TickRate;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::controller::ControllerInput;
use crate::input::InputMovementInput;
use crate::net::PeerControllerClientMessage;

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
    pub message: PeerControllerClientMessage,
    pub peer_handle: u16,
    pub client_stamp: u8,
}
/// Replicates client input to peers this is a server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub struct PeerUnreliableControllerMessage {
    pub message: UnreliablePeerControllerClientMessage,
    pub peer_handle: u16,
    pub client_stamp: u8,
}

pub(crate) fn syncable_entity(
    latest: Res<PeerLatestLookSync>,
    mut commands: Commands,
    handle_to_entity: Res<HandleToEntity>,
    query: Query<(Entity, Option<&DisableSync>), With<ControllerInput>>,
    stamp: Res<TickRateStamp>,
    tickrate: Res<TickRate>,
) {
    for (entity, disabled) in query.iter() {
        match handle_to_entity.inv_map.get(&entity) {
            Some(handle) => match latest.0.get(handle) {
                Some((last_input_stamp, _)) => {
                    if stamp.large
                        > *last_input_stamp
                            + (tickrate.fixed_rate as f32 / (DESYNC_FREQUENCY * 2.)) as u64
                    {
                        if disabled.is_some() {
                            commands.entity(entity).remove::<DisableSync>();
                        }
                    } else {
                        if disabled.is_none() {
                            commands.entity(entity).insert(DisableSync);
                        }
                    }
                }
                None => {}
            },
            None => {
                warn!("Couldnt find handle entity.");
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct PeerLatestLookSync(HashMap<ClientId, (u64, u8)>);

/// Replicate client input to peers from Update schedule.
/// Will make use of generic systems one day.
pub(crate) fn peer_replicate_input_messages(
    mut reliable: EventReader<EarlyIncomingRawReliableClientMessage>,
    mut unreliable: EventReader<EarlyIncomingRawUnreliableClientMessage>,
    mut server: ResMut<RenetServer>,
    players: Query<&ConnectedPlayer, With<RigidBodyLink>>,
    typenames: Res<Typenames>,
    query: Query<(&Transform, &LookTransform), With<RigidBodyLink>>,
    stamp: Res<TickRateStamp>,
    handle_to_entity: Res<HandleToEntity>,
    mut latest_look_transform_sync: ResMut<PeerLatestLookSync>,
) {
    let mut reliable_peer_messages: HashMap<ClientId, Vec<ReliableMessage>> = HashMap::new();
    let mut unreliable_peer_messages: HashMap<ClientId, Vec<UnreliableMessage>> = HashMap::new();
    for batch in reliable.read() {
        for message in batch.0.message.messages.iter() {
            match typenames
                .reliable_net_types
                .get(&ControllerClientMessage::type_name())
            {
                Some(id) => {
                    if id == &message.typename_net {
                        match bincode::deserialize::<ControllerClientMessage>(&message.serialized) {
                            Ok(client_message) => {
                                for connected in players.iter() {
                                    if !connected.connected {
                                        continue;
                                    }
                                    if batch.0.handle == connected.handle {
                                        continue;
                                    }
                                    let moving_entity;
                                    match handle_to_entity.map.get(&batch.0.handle) {
                                        Some(e) => {
                                            moving_entity = *e;
                                        }
                                        None => {
                                            warn!("no handle entity found.");
                                            continue;
                                        }
                                    }
                                    let tuple;
                                    match query.get(moving_entity) {
                                        Ok(t) => {
                                            tuple = t.clone();
                                        }
                                        Err(_) => {
                                            warn!("Couldnt find moving entity.");
                                            continue;
                                        }
                                    }
                                    let new_message = PeerReliableControllerMessage {
                                        message: PeerControllerClientMessage::from(
                                            client_message.clone(),
                                            tuple.0.translation,
                                            tuple.1.target,
                                        ),
                                        peer_handle: batch.0.handle.raw() as u16,
                                        client_stamp: batch.0.message.stamp,
                                    };

                                    let sub_id = typenames
                                        .reliable_net_types
                                        .get(&PeerReliableControllerMessage::type_name())
                                        .unwrap();

                                    let reliable_message = ReliableMessage {
                                        serialized: bincode::serialize(&new_message).unwrap(),
                                        typename_net: *sub_id,
                                    };

                                    match reliable_peer_messages.get_mut(&connected.handle) {
                                        Some(messages) => messages.push(reliable_message),
                                        None => {
                                            reliable_peer_messages
                                                .insert(connected.handle, vec![reliable_message]);
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                warn!("Couldnt deserialize client message.");
                            }
                        }
                    }
                }
                None => {
                    warn!("Unknown type name.");
                }
            }
        }
    }
    for batch in unreliable.read() {
        for message in batch.0.message.messages.iter() {
            match typenames
                .unreliable_net_types
                .get(&UnreliableControllerClientMessage::type_name())
            {
                Some(id) => {
                    match bincode::deserialize::<UnreliableControllerClientMessage>(
                        &message.serialized,
                    ) {
                        Ok(client_message) => {
                            if id == &message.typename_net {
                                for connected in players.iter() {
                                    if !connected.connected {
                                        continue;
                                    }
                                    if batch.0.handle == connected.handle {
                                        continue;
                                    }

                                    let mut latest = false;
                                    match client_message {
                                        UnreliableControllerClientMessage::UpdateLookTransform(
                                            _,
                                            new_id,
                                        ) => {
                                            let large =
                                                stamp.calculate_large(batch.0.message.stamp);
                                            match latest_look_transform_sync.0.get(&batch.0.handle)
                                            {
                                                Some((tick, id)) => {
                                                    if large >= *tick
                                                        || (large == *tick && new_id > *id)
                                                    {
                                                        latest = true;
                                                        latest_look_transform_sync.0.insert(
                                                            batch.0.handle,
                                                            (large, new_id),
                                                        );
                                                    }
                                                }
                                                None => {
                                                    latest = true;
                                                    latest_look_transform_sync
                                                        .0
                                                        .insert(batch.0.handle, (large, new_id));
                                                }
                                            }
                                        }
                                    }
                                    if !latest {
                                        continue;
                                    }

                                    let moving_entity;
                                    match handle_to_entity.map.get(&batch.0.handle) {
                                        Some(e) => {
                                            moving_entity = *e;
                                        }
                                        None => {
                                            warn!("no handle entity found.");
                                            continue;
                                        }
                                    }
                                    let tuple;
                                    match query.get(moving_entity) {
                                        Ok(t) => {
                                            tuple = t.clone();
                                        }
                                        Err(_) => {
                                            warn!("Couldnt find moving entity.");
                                            continue;
                                        }
                                    }

                                    let new_message = PeerUnreliableControllerMessage {
                                        message: UnreliablePeerControllerClientMessage::from(
                                            client_message.clone(),
                                            tuple.0.translation,
                                        ),
                                        peer_handle: batch.0.handle.raw() as u16,
                                        client_stamp: batch.0.message.stamp,
                                    };

                                    let sub_id = typenames
                                        .unreliable_net_types
                                        .get(&PeerUnreliableControllerMessage::type_name())
                                        .unwrap();

                                    let unreliable_message = UnreliableMessage {
                                        serialized: bincode::serialize(&new_message).unwrap(),
                                        typename_net: *sub_id,
                                    };
                                    match unreliable_peer_messages.get_mut(&connected.handle) {
                                        Some(messages) => messages.push(unreliable_message),
                                        None => {
                                            unreliable_peer_messages
                                                .insert(connected.handle, vec![unreliable_message]);
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            warn!("Couldnt deserialize client message 1.");
                        }
                    }
                }
                None => {
                    warn!("Unknown type name.");
                }
            }
        }
    }

    for (id, msgs) in reliable_peer_messages {
        server.send_message(
            id,
            RENET_RELIABLE_ORDERED_ID,
            bincode::serialize(&ReliableServerMessageBatch {
                messages: msgs,
                stamp: stamp.tick,
            })
            .unwrap(),
        );
    }
    for (id, msgs) in unreliable_peer_messages {
        server.send_message(
            id,
            RENET_UNRELIABLE_CHANNEL_ID,
            bincode::serialize(&UnreliableServerMessageBatch {
                messages: msgs,
                stamp: stamp.tick,
            })
            .unwrap(),
        );
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
                            entity: *player_entity,
                            pressed: movement_input.pressed,
                            up: movement_input.up,
                            left: movement_input.left,
                            right: movement_input.right,
                            down: movement_input.down,
                            peer_data: None,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                }
            }
            ControllerClientMessage::SyncControllerInput(_) => (),
        }
    }
}
