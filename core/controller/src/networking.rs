use std::collections::HashMap;

use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Local;
use bevy::ecs::system::ResMut;
use bevy::ecs::system::Resource;
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::With;
use bevy::transform::components::Transform;
use bevy_renet::renet::ClientId;
use cameras::LookTransform;
use entity::senser::Senser;
use itertools::Itertools;
use networking::server::ConnectedPlayer;
use networking::server::IncomingUnreliableClientMessage;
use networking::server::OutgoingReliableServerMessage;
use networking::server::OutgoingUnreliableServerMessage;
use networking::stamp::TickRateStamp;
use pawn::net::UnreliableControllerClientMessage;
use pawn::net::UnreliablePeerControllerClientMessage;
use physics::entity::RigidBodyLink;
use physics::sync::DisableSync;
use physics::sync::DESYNC_FREQUENCY;
use resources::core::TickRate;
use resources::correction::MAX_CACHE_TICKS_AMNT;
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
}
/// Replicates client input to peers this is a server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub struct PeerUnreliableControllerMessage {
    pub message: UnreliablePeerControllerClientMessage,
    pub peer_handle: u16,
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
                //warn!("Couldnt find handle entity.");
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct PeerLatestLookSync(HashMap<ClientId, (u64, u8)>);

pub(crate) fn server_replicate_peer_input_messages(
    mut reliable: EventReader<IncomingReliableClientMessage<ControllerClientMessage>>,
    mut unreliable: EventReader<IncomingUnreliableClientMessage<UnreliableControllerClientMessage>>,
    players: Query<(&ConnectedPlayer, &Senser), With<RigidBodyLink>>,
    query: Query<(&Transform, &LookTransform), With<RigidBodyLink>>,
    stamp: Res<TickRateStamp>,
    handle_to_entity: Res<HandleToEntity>,
    mut latest_look_transform_sync: ResMut<PeerLatestLookSync>,
    mut queue: Local<HashMap<ClientId, HashMap<u64, HashMap<u8, Vec3>>>>,
    mut send_reliable: EventWriter<OutgoingReliableServerMessage<PeerReliableControllerMessage>>,
    mut send_unreliable: EventWriter<
        OutgoingUnreliableServerMessage<PeerUnreliableControllerMessage>,
    >,
) {
    for message in reliable.read() {
        let moving_entity;
        match handle_to_entity.map.get(&message.handle) {
            Some(e) => {
                moving_entity = *e;
            }
            None => {
                warn!("no handle entity found.0");
                continue;
            }
        }
        let tuple;
        match query.get(moving_entity) {
            Ok(t) => {
                tuple = t.clone();
            }
            Err(_) => {
                warn!("replicate couldnt find moving entity.");
                continue;
            }
        }
        for (connected, senser) in players.iter() {
            if !connected.connected {
                continue;
            }
            if message.handle == connected.handle {
                continue;
            }
            if !senser.sensing.contains(&moving_entity) {
                continue;
            }

            let new_message = PeerReliableControllerMessage {
                message: PeerControllerClientMessage::from(
                    message.message.clone(),
                    tuple.0.translation,
                    tuple.1.target,
                ),
                peer_handle: message.handle.raw() as u16,
            };
            send_reliable.send(OutgoingReliableServerMessage {
                handle: connected.handle,
                message: new_message,
            });
        }
    }
    for message in unreliable.read() {
        let moving_entity;
        match handle_to_entity.map.get(&message.handle) {
            Some(e) => {
                moving_entity = *e;
            }
            None => {
                //warn!("no handle entity found.1");
                continue;
            }
        }
        let tuple;
        match query.get(moving_entity) {
            Ok(t) => {
                tuple = t.clone();
            }
            Err(_) => {
                //warn!("Couldnt find moving entity.");
                continue;
            }
        }
        for (connected, senser) in players.iter() {
            if !connected.connected {
                continue;
            }
            if message.handle == connected.handle {
                continue;
            }
            if !senser.sensing.contains(&moving_entity) {
                continue;
            }

            let mut latest = false;
            match message.message {
                UnreliableControllerClientMessage::UpdateLookTransform(target, new_id) => {
                    let large = stamp.large;
                    match queue.get_mut(&connected.handle) {
                        Some(q1) => match q1.get_mut(&large) {
                            Some(q2) => {
                                q2.insert(new_id, target);
                                for i in q2.keys().sorted().rev() {
                                    if new_id > *i {
                                        latest = true;
                                    }
                                    break;
                                }
                            }
                            None => {
                                let mut m = HashMap::new();
                                m.insert(new_id, target);
                                q1.insert(large, m);
                                latest = true;
                            }
                        },
                        None => {
                            let mut n = HashMap::new();
                            n.insert(new_id, target);
                            let mut m = HashMap::new();
                            m.insert(large, n);
                            queue.insert(connected.handle, m);
                            latest = true;
                        }
                    }

                    match latest_look_transform_sync.0.get(&message.handle) {
                        Some((tick, id)) => {
                            if large >= *tick || (large == *tick && new_id > *id) {
                                latest_look_transform_sync
                                    .0
                                    .insert(message.handle, (large, new_id));
                            }
                        }
                        None => {
                            latest_look_transform_sync
                                .0
                                .insert(message.handle, (large, new_id));
                        }
                    }
                }
            }
            if !latest {
                continue;
            }

            let new_message = PeerUnreliableControllerMessage {
                message: UnreliablePeerControllerClientMessage::from(
                    message.message.clone(),
                    tuple.0.translation,
                ),
                peer_handle: message.handle.raw() as u16,
            };
            send_unreliable.send(OutgoingUnreliableServerMessage {
                handle: connected.handle,
                message: new_message,
            });
        }
    }

    // Clean cache.
    for (_, cache) in queue.iter_mut() {
        if cache.len() > MAX_CACHE_TICKS_AMNT as usize {
            let mut j = 0;
            for i in cache.clone().keys().sorted().rev() {
                if j >= MAX_CACHE_TICKS_AMNT {
                    cache.remove(i);
                }
                j += 1;
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
