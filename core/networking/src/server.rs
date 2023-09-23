use bevy::{
    math::{Vec2, Vec3},
    prelude::{info, Component, Entity, Event, Local, Quat, Resource, SystemSet},
};
use resources::core::TickRate;
use serde::{Deserialize, Serialize};
use typename::TypeName;

use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, DefaultChannel, RenetServer,
};

/// The network port the server will listen use for connections.

pub const SERVER_PORT: u16 = 57713;

/// Network protocol ID.

pub(crate) const PROTOCOL_ID: u64 = 7;

pub(crate) const PRIV_KEY: [u8; 32] = *b"(=^.^=)(=^.^=)(=^.^=)(=^.^=)(=^.";

/// Start server and open and listen to port.

pub(crate) fn startup_server_listen_connections() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = (local_ipaddress::get().unwrap_or_default() + ":" + &SERVER_PORT.to_string())
        .parse()
        .unwrap();
    let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();
    let channels_config = DefaultChannel::config();

    let renet_server = RenetServer::new(ConnectionConfig {
        server_channels_config: channels_config.clone(),
        client_channels_config: channels_config,

        ..Default::default()
    });

    let server_config = ServerConfig {
        max_clients: 128,
        protocol_id: PROTOCOL_ID,
        public_addr,
        authentication: ServerAuthentication::Secure {
            private_key: PRIV_KEY,
        },
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    info!("Listening to connections on [{}].", public_addr);

    (renet_server, transport)
}

use bevy::prelude::EventReader;

use crate::{
    client::NetworkingClientMessage,
    messaging::{
        ReliableClientMessageBatch, ReliableMessage, ReliableServerMessageBatch, UnreliableMessage,
        UnreliableServerMessageBatch,
    },
    plugin::RENET_RELIABLE_ORDERED_ID,
    stamp::TickRateStamp,
};

/// Obtain player souls, mwahahhaa. (=^.^=)

pub(crate) fn souls(
    mut server: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();
        match client_message {
            //                                          |
            // Where the souls of the players are       |
            //   while they're connected.               V
            NetworkingClientMessage::HeartBeat => { /* <3 */ }
            _ => (),
        }
    }
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingServerMessage {
    Awoo(StartSync),
    AdjustSync(AdjustSync),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartSync {
    pub tick_rate: TickRate,
    pub stamp: TickRateStamp,
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UnreliableServerMessage {
    TransformUpdate(u64, Vec3, Quat, Option<Vec3>, u64, u8),
    PositionUpdate(u64, Vec3, u64),
}

/// Variant types for input console commands with values.
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum ConsoleArgVariant {
    Int,
    String,
    Float,
    Bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum UIInputAction {
    Pressed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum TextTreeBit {
    Final(Vec<String>),
    Bit(HashMap<String, TextTreeBit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum EntityUpdateData {
    Int(i64),
    UInt8(u8),
    String(String),
    StringVec(Vec<String>),
    Float(f32),
    Transform(Vec3, Quat, Vec3),
    Color(f32, f32, f32, f32),
    Bool(bool),
    Vec3(Vec3),
    Vec2(Vec2),
    AttachedItem(u64, Vec3, Quat, Vec3),
    WornItem(String, u64, String, Vec3, Quat, Vec3),
    WornItemNotAttached(String, u64, String),
}

/// A resource that links entities to their appropiate connection handles for connected players.
#[derive(Default, Resource)]

pub struct HandleToEntity {
    pub map: HashMap<u64, Entity>,
    pub inv_map: HashMap<Entity, u64>,
}

/// The component for an entity controlled by a connected player.
#[derive(Component, Clone)]

pub struct ConnectedPlayer {
    pub handle: u64,
    pub authid: u16,
    pub rcon: bool,
    pub connected: bool,
}

impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: 0,
            authid: 0,
            rcon: false,
            connected: true,
        }
    }
}

/// Gets serialized and sent over the net, this is the server message.
/// This should be inside core/chat/ but this causes cyclic dependency for the time being.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingChatServerMessage {
    ChatMessage(String),
}

use bevy::prelude::warn;

/// Serializes and sends the outgoing unreliable server messages.
pub(crate) fn send_outgoing_unreliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    let mut messages: HashMap<u64, Vec<UnreliableMessage>> = HashMap::default();
    for message in events.iter() {
        let net;
        match typenames
            .unreliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!("Couldnt find unreliable type");
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize unreliable message");
                continue;
            }
        }

        let z = UnreliableMessage {
            serialized: bin,
            typename_net: *net,
        };

        match messages.get_mut(&message.handle) {
            Some(m) => {
                m.push(z);
            }
            None => {
                messages.insert(message.handle, vec![z]);
            }
        }
    }

    for (handle, msgs) in messages {
        match bincode::serialize(&UnreliableServerMessageBatch {
            messages: msgs,
            stamp: stamp.stamp,
        }) {
            Ok(bits) => {
                server.send_message(handle, RENET_UNRELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                return;
            }
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ServerMessageSet {
    Send,
}

use bevy::prelude::{Res, ResMut};

use crate::messaging::Typenames;
/// Serializes and sends the outgoing reliable server messages.
pub fn send_outgoing_reliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    let mut messages: HashMap<u64, Vec<ReliableMessage>> = HashMap::default();

    for message in events.iter() {
        let net;
        match typenames
            .reliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!(
                    "Couldnt find server reliable type {}",
                    message.message.type_name_of()
                );
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize reliable message");
                continue;
            }
        }

        let z = ReliableMessage {
            serialized: bin,
            typename_net: *net,
        };

        match messages.get_mut(&message.handle) {
            Some(m) => {
                m.push(z);
            }
            None => {
                messages.insert(message.handle, vec![z]);
            }
        }
    }
    for (handle, msgs) in messages {
        match bincode::serialize(&ReliableServerMessageBatch {
            messages: msgs,
            stamp: stamp.stamp,
        }) {
            Ok(bits) => {
                server.send_message(handle, RENET_RELIABLE_ORDERED_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                return;
            }
        }
    }
}
use crate::client::get_unreliable_message;
use bevy::prelude::EventWriter;

pub(crate) fn deserialize_incoming_unreliable_client_message<
    T: TypeName + Send + Sync + Serialize + Clone + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawUnreliableClientMessage>,
    mut outgoing: EventWriter<IncomingUnreliableClientMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
    mut queue: Local<HashMap<u8, Vec<IncomingUnreliableClientMessage<T>>>>,
) {
    for event in incoming_raw.iter() {
        for message in event.message.messages.iter() {
            match get_unreliable_message::<T>(&typenames, message.typename_net, &message.serialized)
            {
                Some(data) => {
                    let r = IncomingUnreliableClientMessage {
                        message: data,
                        handle: event.handle,
                        stamp: event.message.stamp,
                    };

                    if stamp.stamp > event.message.stamp
                        || (event.message.stamp - stamp.stamp) > 182
                    {
                        continue;
                    }

                    match queue.get_mut(&event.message.stamp) {
                        Some(v) => v.push(r),
                        None => {
                            queue.insert(event.message.stamp, vec![r]);
                        }
                    }
                }
                None => {}
            }
        }
    }

    match queue.get_mut(&stamp.stamp) {
        Some(v) => {
            for msg in v.clone() {
                outgoing.send(msg);
            }
            v.clear();
        }
        None => {}
    }
}
use crate::messaging::get_reliable_message;

pub(crate) fn deserialize_incoming_reliable_client_message<
    T: TypeName + Send + Sync + Serialize + Clone + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawReliableClientMessage>,
    mut outgoing: EventWriter<IncomingReliableClientMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
    mut queue: Local<HashMap<u8, Vec<IncomingReliableClientMessage<T>>>>,
) {
    for event in incoming_raw.iter() {
        for message in event.message.messages.iter() {
            match get_reliable_message::<T>(&typenames, message.typename_net, &message.serialized) {
                Some(data) => {
                    let r = IncomingReliableClientMessage {
                        message: data,
                        handle: event.handle,
                        stamp: event.message.stamp,
                    };

                    if stamp.stamp > event.message.stamp
                        || (event.message.stamp - stamp.stamp) > 182
                    {
                        outgoing.send(r);
                        continue;
                    }

                    match queue.get_mut(&event.message.stamp) {
                        Some(v) => v.push(r),
                        None => {
                            queue.insert(event.message.stamp, vec![r]);
                        }
                    }
                }
                None => {}
            }
        }
    }

    match queue.get_mut(&stamp.stamp) {
        Some(v) => {
            for msg in v.clone() {
                outgoing.send(msg);
            }
            v.clear();
        }
        None => {}
    }
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event, Clone)]
pub struct IncomingReliableClientMessage<T: TypeName + Send + Sync + Serialize> {
    pub handle: u64,
    pub message: T,
    pub stamp: u8,
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event, Clone)]
pub struct IncomingUnreliableClientMessage<T: TypeName + Send + Sync + Serialize + Clone> {
    pub handle: u64,
    pub message: T,
    pub stamp: u8,
}

#[derive(Resource, Default)]
pub struct SyncConfirmations {
    pub incremental: HashMap<u64, u64>,
    pub server_sync: HashMap<u64, u64>,
}
#[derive(Debug)]
pub struct LatencyReport {
    pub client_sync_iteration: u64,
    pub tick_difference: i8,
}

/// Vectors containing adjustment iteration and tick difference linked by connection handle.
#[derive(Resource, Default)]
pub struct Latency {
    pub tickrate_differences: HashMap<u64, Vec<LatencyReport>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdjustSync {
    pub advance: i8,
}

pub(crate) fn adjust_clients(
    mut latency: ResMut<Latency>,
    tickrate: Res<TickRate>,
    mut net: EventWriter<OutgoingReliableServerMessage<NetworkingServerMessage>>,
    mut confirmations: ResMut<SyncConfirmations>,
) {
    for (handle, tickrate_differences) in latency.tickrate_differences.iter_mut() {
        let server_side_sync_iteration;

        match confirmations.server_sync.get(handle) {
            Some(x) => {
                server_side_sync_iteration = *x;
            }
            None => {
                server_side_sync_iteration = 0;
            }
        }

        let mut accumulative = 0;
        let mut length = 0;
        for difference in tickrate_differences.iter() {
            if difference.client_sync_iteration >= server_side_sync_iteration {
                accumulative += difference.tick_difference as i16;
                length += 1;
            }
        }
        let average_latency = accumulative as f32 / length as f32;

        let max_latency = 3. * (tickrate.bevy_rate as f32 / 32.);

        if length >= 16 {
            if average_latency < 1. {
                // Tell client to fast-forward x ticks.
                let advance;
                if average_latency > 0. {
                    advance = -1;
                } else {
                    advance = average_latency.floor() as i8 - 1;
                }

                net.send(OutgoingReliableServerMessage {
                    handle: *handle,
                    message: NetworkingServerMessage::AdjustSync(AdjustSync { advance }),
                });

                tickrate_differences.clear();
                length = 0;

                match confirmations.server_sync.get_mut(handle) {
                    Some(x) => {
                        *x += 1;
                    }
                    None => {
                        confirmations.server_sync.insert(*handle, 1);
                    }
                }
            } else if average_latency > max_latency {
                // Tell client freeze x ticks.

                net.send(OutgoingReliableServerMessage {
                    handle: *handle,
                    message: NetworkingServerMessage::AdjustSync(AdjustSync {
                        advance: average_latency.ceil() as i8 - max_latency.floor() as i8,
                    }),
                });
                tickrate_differences.clear();
                length = 0;

                match confirmations.server_sync.get_mut(handle) {
                    Some(x) => {
                        *x += 1;
                    }
                    None => {
                        confirmations.server_sync.insert(*handle, 1);
                    }
                }
            }
        }

        if length > 16 {
            tickrate_differences.remove(0);
        }
    }
}

/// Deserializes header of incoming client messages and writes to event.

pub(crate) fn receive_incoming_unreliable_client_messages(
    mut events: EventWriter<IncomingRawUnreliableClientMessage>,
    mut server: ResMut<RenetServer>,
    mut sync: ResMut<Latency>,
    stamp: Res<TickRateStamp>,
    confirmations: Res<SyncConfirmations>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_UNRELIABLE_CHANNEL_ID) {
            match bincode::deserialize::<UnreliableServerMessageBatch>(&message) {
                Ok(msg) => {
                    events.send(IncomingRawUnreliableClientMessage {
                        message: msg.clone(),
                        handle,
                    });
                    let c: u64;
                    match confirmations.incremental.get(&handle) {
                        Some(x) => {
                            c = *x;
                        }
                        None => {
                            c = 0;
                        }
                    }
                    match sync.tickrate_differences.get_mut(&handle) {
                        Some(v) => {
                            v.push(LatencyReport {
                                client_sync_iteration: c,
                                tick_difference: stamp.get_difference(msg.stamp),
                            });
                        }
                        None => {
                            sync.tickrate_differences.insert(
                                handle,
                                vec![LatencyReport {
                                    client_sync_iteration: c,
                                    tick_difference: stamp.get_difference(msg.stamp),
                                }],
                            );
                        }
                    }
                }
                Err(_) => {
                    warn!("Received an invalid message 0.");
                }
            }
        }
    }
}
use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;

/// Deserializes header of incoming client messages and writes to event.

pub(crate) fn receive_incoming_reliable_client_messages(
    mut events: EventWriter<IncomingRawReliableClientMessage>,
    mut server: ResMut<RenetServer>,
    mut sync: ResMut<Latency>,
    stamp: Res<TickRateStamp>,
    typenames: Res<Typenames>,
    mut confirmations: ResMut<SyncConfirmations>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_ORDERED_ID) {
            match bincode::deserialize::<ReliableClientMessageBatch>(&message) {
                Ok(msg) => {
                    events.send(IncomingRawReliableClientMessage {
                        message: msg.clone(),
                        handle,
                    });

                    for m in msg.messages.iter() {
                        if m.typename_net
                            == *typenames
                                .reliable_net_types
                                .get(&NetworkingClientMessage::type_name())
                                .unwrap()
                        {
                            match confirmations.incremental.get_mut(&handle) {
                                Some(a) => {
                                    *a += 1;
                                }
                                None => {
                                    confirmations.incremental.insert(handle, 1);
                                }
                            }
                        }
                    }

                    let c: u64;
                    match confirmations.incremental.get(&handle) {
                        Some(x) => {
                            c = *x;
                        }
                        None => {
                            c = 0;
                        }
                    }
                    match sync.tickrate_differences.get_mut(&handle) {
                        Some(v) => {
                            v.push(LatencyReport {
                                client_sync_iteration: c,
                                tick_difference: stamp.get_difference(msg.stamp),
                            });
                        }
                        None => {
                            sync.tickrate_differences.insert(
                                handle,
                                vec![LatencyReport {
                                    client_sync_iteration: c,
                                    tick_difference: stamp.get_difference(msg.stamp),
                                }],
                            );
                        }
                    }
                }
                Err(_) => {
                    warn!("Received an invalid message.");
                }
            }
        }
    }
}

/// Event to send reliable messages from server to client. Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event)]
pub struct OutgoingReliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}

/// Event to send unreliable messages from server to client. Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub struct OutgoingUnreliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event)]
pub(crate) struct IncomingRawReliableClientMessage {
    pub handle: u64,
    pub message: ReliableClientMessageBatch,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub(crate) struct IncomingRawUnreliableClientMessage {
    pub handle: u64,
    pub message: UnreliableServerMessageBatch,
}
