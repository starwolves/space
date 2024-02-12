use bevy::log::info;
use bevy::{
    math::{Vec2, Vec3},
    prelude::{Component, Entity, Event, Local, Quat, Resource, SystemSet},
};

use bevy_renet::renet::ServerEvent;
use resources::core::TickRate;
use serde::{Deserialize, Serialize};
use typename::TypeName;

use std::collections::BTreeMap;
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ClientId, ConnectionConfig, DefaultChannel, RenetServer,
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
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_config = ServerConfig {
        max_clients: 128,
        protocol_id: PROTOCOL_ID,
        authentication: ServerAuthentication::Secure {
            private_key: PRIV_KEY,
        },
        public_addresses: vec![public_addr],
        current_time,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

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
    mut server: EventReader<IncomingUnreliableClientMessage<NetworkingUnreliableClientMessage>>,
) {
    for message in server.read() {
        let client_message = message.message.clone();
        match client_message {
            //                                          |
            // Where the souls of the players are       |
            //   while they're connected.               V
            NetworkingUnreliableClientMessage::HeartBeat => { /* <3 */ }
        }
    }
}
#[derive(Resource, Default)]
pub(crate) struct ClientsReadyForSync(HashMap<ClientId, bool>);

pub(crate) fn client_loaded_game_world(
    mut messages: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
    mut start: EventWriter<OutgoingReliableServerMessage<NetworkingServerMessage>>,
    stamp: Res<TickRateStamp>,
    tickrate: Res<TickRate>,
) {
    for message in messages.read() {
        match message.message {
            NetworkingClientMessage::LoadedGameWorld => {
                start.send(OutgoingReliableServerMessage {
                    handle: message.handle,
                    message: NetworkingServerMessage::StartSync(StartSync {
                        tick_rate: tickrate.clone(),
                        stamp: stamp.clone(),
                    }),
                });
            }
            _ => (),
        }
    }
}

pub(crate) fn start_sync_confirmation(
    mut net: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
    mut cache: ResMut<ClientsReadyForSync>,
) {
    for msg in net.read() {
        match &msg.message {
            NetworkingClientMessage::StartSyncConfirmation => {
                cache.0.insert(msg.handle, true);
            }
            _ => (),
        }
    }
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingServerMessage {
    Awoo,
    StartSync(StartSync),
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
    pub map: HashMap<ClientId, Entity>,
    pub inv_map: HashMap<Entity, ClientId>,
}

/// The component for an entity controlled by a connected player.
#[derive(Component, Clone)]

pub struct ConnectedPlayer {
    pub handle: ClientId,
    pub authid: u16,
    pub rcon: bool,
    pub connected: bool,
}

impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: ClientId::from_raw(0),
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

use bevy::log::warn;

/// Serializes and sends the outgoing unreliable server messages.
pub(crate) fn send_outgoing_unreliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    let mut messages: HashMap<ClientId, Vec<UnreliableMessage>> = HashMap::default();
    for message in events.read() {
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
            stamp: stamp.tick,
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

use crate::messaging::{Typenames, UnreliableClientMessageBatch};
/// Serializes and sends the outgoing reliable server messages.
pub fn send_outgoing_reliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    let mut messages_ordered: HashMap<ClientId, Vec<ReliableMessage>> = HashMap::default();
    let mut messages_unordered: HashMap<ClientId, Vec<ReliableMessage>> = HashMap::default();

    for message in events.read() {
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

        let mut unordered = false;
        for x in &typenames.reliable_unordered_types {
            if x == &message.message.type_name_of() {
                unordered = true;
                break;
            }
        }

        if unordered {
            match messages_unordered.get_mut(&message.handle) {
                Some(m) => {
                    m.push(z);
                }
                None => {
                    messages_ordered.insert(message.handle, vec![z]);
                }
            }
        } else {
            match messages_ordered.get_mut(&message.handle) {
                Some(m) => {
                    m.push(z);
                }
                None => {
                    messages_ordered.insert(message.handle, vec![z]);
                }
            }
        }
    }
    for (handle, msgs) in messages_ordered {
        match bincode::serialize(&ReliableServerMessageBatch {
            messages: msgs,
            stamp: stamp.tick,
        }) {
            Ok(bits) => {
                server.send_message(handle, RENET_RELIABLE_ORDERED_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize reliable message.");
                return;
            }
        }
    }
    for (handle, msgs) in messages_unordered {
        match bincode::serialize(&ReliableServerMessageBatch {
            messages: msgs,
            stamp: stamp.tick,
        }) {
            Ok(bits) => {
                server.send_message(handle, RENET_RELIABLE_UNORDERED_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize reliable message.");
                return;
            }
        }
    }
}
use crate::client::{get_unreliable_message, NetworkingUnreliableClientMessage};
use bevy::prelude::EventWriter;

pub(crate) fn deserialize_incoming_unreliable_client_message<
    T: TypeName + Send + Sync + Serialize + Clone + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawUnreliableClientMessage>,
    mut outgoing: EventWriter<IncomingUnreliableClientMessage<T>>,
    typenames: Res<Typenames>,

    stamp: Res<TickRateStamp>,
) {
    for event in incoming_raw.read() {
        for message in event.message.messages.iter() {
            match get_unreliable_message::<T>(&typenames, message.typename_net, &message.serialized)
            {
                Some(data) => {
                    let b = stamp.calculate_large(event.message.stamp);

                    let r = IncomingUnreliableClientMessage {
                        message: data,
                        handle: event.handle,
                        stamp: b,
                    };
                    outgoing.send(r);
                }
                None => {}
            }
        }
    }
}
use crate::messaging::get_reliable_message;
use std::fmt::Debug;
pub(crate) fn deserialize_incoming_reliable_client_message<
    T: Debug + TypeName + Send + Sync + Serialize + Clone + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawReliableClientMessage>,
    mut outgoing: EventWriter<IncomingReliableClientMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for event in incoming_raw.read() {
        for message in event.message.messages.iter() {
            match get_reliable_message::<T>(&typenames, message.typename_net, &message.serialized) {
                Some(data) => {
                    let b = stamp.calculate_large(event.message.stamp);

                    let r = IncomingReliableClientMessage {
                        message: data,
                        handle: event.handle,
                        stamp: b,
                        fixed: event.message.fixed,
                    };
                    outgoing.send(r);
                }
                None => {}
            }
        }
    }
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event, Clone)]
pub struct IncomingReliableClientMessage<T: TypeName + Send + Sync + Serialize> {
    pub handle: ClientId,
    pub message: T,
    pub stamp: u64,
    pub fixed: bool,
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event, Clone)]
pub struct IncomingUnreliableClientMessage<T: TypeName + Send + Sync + Serialize + Clone> {
    pub handle: ClientId,
    pub message: T,
    pub stamp: u64,
}

#[derive(Resource, Default)]
pub struct SyncConfirmations {
    pub incremental: HashMap<ClientId, u64>,
    pub server_sync: HashMap<ClientId, u64>,
}
#[derive(Debug)]
pub struct LatencyReport {
    pub client_sync_iteration: u64,
    pub tick_difference: i16,
}

/// Vectors containing adjustment iteration and tick difference linked by connection handle.
#[derive(Resource, Default)]
pub struct Latency {
    pub reports: HashMap<ClientId, Vec<LatencyReport>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdjustSync {
    pub tick: u64,
    pub forward: bool,
}
pub(crate) fn clean_latency_reports(
    mut latency: ResMut<Latency>,
    confirmations: Res<SyncConfirmations>,
) {
    for (handle, tickrate_differences) in latency.reports.iter_mut() {
        let server_sync;

        match confirmations.server_sync.get(handle) {
            Some(x) => {
                server_sync = *x;
            }
            None => {
                server_sync = 0;
            }
        }
        let mut length = 0;
        for difference in tickrate_differences.iter() {
            if difference.client_sync_iteration == server_sync {
                length += 1;
            }
        }
        if length > DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT {
            for _ in 0..length - DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT {
                tickrate_differences.remove(0);
            }
        }
    }
}
pub(crate) fn adjust_clients(
    latency: Res<Latency>,
    tickrate: Res<TickRate>,
    mut net: EventWriter<OutgoingReliableServerMessage<NetworkingServerMessage>>,
    mut confirmations: ResMut<SyncConfirmations>,
    synced_clients: Res<ClientsReadyForSync>,
    stamp: Res<TickRateStamp>,
    limits: Res<LatencyLimits>,
) {
    for (handle, tickrate_differences) in latency.reports.iter() {
        let mut ready = false;
        match synced_clients.0.get(handle) {
            Some(b) => {
                ready = *b;
            }
            None => {}
        }
        if !ready {
            continue;
        }
        let server_sync;

        match confirmations.server_sync.get(handle) {
            Some(x) => {
                server_sync = *x;
            }
            None => {
                server_sync = 0;
            }
        }
        let min_limit;
        let max_limit;
        let desired_min_point;
        let desired_max_point;

        match limits.limits.get(handle) {
            Some(l) => {
                min_limit = l.min;
                max_limit = l.max;
                desired_min_point = max_limit as f32 - ((l.max as f32 - l.min as f32) / 2.).ceil();
                desired_max_point = min_limit as f32 + ((l.max as f32 - l.min as f32) / 2.).round();
            }
            None => {
                //warn!("Couldnt find handle in limits.");
                //client disconnected
                continue;
            }
        }

        let mut accumulative = 0;
        let mut length = 0;
        for difference in tickrate_differences.iter() {
            if difference.client_sync_iteration == server_sync {
                accumulative += difference.tick_difference as i16;
                length += 1;
            }
        }
        let average_latency = accumulative as f32 / length as f32;

        let min_latency = min_limit as f32 * (tickrate.fixed_rate as f32 / 60.);
        let max_latency = max_limit as f32 * (tickrate.fixed_rate as f32 / 60.);

        if length >= DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT {
            if average_latency < min_latency {
                // Tell client to fast-forward x ticks.
                let advance;
                if average_latency > 0. {
                    advance = desired_min_point - average_latency;
                } else {
                    advance = average_latency.abs() + desired_min_point;
                }
                let num = advance.round() as i16;
                if num == 0 {
                    continue;
                }
                let sync = AdjustSync {
                    tick: (stamp.large as i128 + num as i128) as u64,
                    forward: true,
                };

                net.send(OutgoingReliableServerMessage {
                    handle: *handle,
                    message: NetworkingServerMessage::AdjustSync(sync),
                });

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

                let num = average_latency.round() as i16 - desired_max_point as i16;
                if num == 0 {
                    continue;
                }
                let sync = AdjustSync {
                    tick: (stamp.large as i128 - num as i128) as u64,
                    forward: false,
                };
                net.send(OutgoingReliableServerMessage {
                    handle: *handle,
                    message: NetworkingServerMessage::AdjustSync(sync),
                });

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
    }
}
pub(crate) const DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT: i32 = 15 * 2;

// In ticks.
pub const DEFAULT_MIN_LATENCY: u8 = 2;
// In ticks.
pub const DEFAULT_MAX_LATENCY: u8 = 3;

#[derive(Resource, Default, Clone)]
pub(crate) struct LatencyLimits {
    pub limits: HashMap<ClientId, Limit>,
}
#[derive(Clone)]
pub(crate) struct Limit {
    pub min: u8,
    pub max: u8,
}

impl Default for Limit {
    fn default() -> Self {
        Limit {
            min: DEFAULT_MIN_LATENCY,
            max: DEFAULT_MAX_LATENCY,
        }
    }
}

pub(crate) fn server_events(
    mut server_event: EventReader<ServerEvent>,
    mut limits: ResMut<LatencyLimits>,
    mut latency: ResMut<Latency>,
) {
    for event in server_event.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                limits.limits.insert(*client_id, Limit::default());
            }
            ServerEvent::ClientDisconnected {
                client_id,
                reason: _,
            } => {
                limits.limits.remove(client_id);
                latency.reports.remove(client_id);
            }
        }
    }
}

/// For connections whose network is less consistent than one tick in latency.
/// Could work better with higher DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT values.
pub(crate) fn _adjust_latency_limits(
    mut limits: ResMut<LatencyLimits>,
    report: Res<Latency>,
    synced_clients: Res<ClientsReadyForSync>,
    confirmations: Res<SyncConfirmations>,
) {
    for (handle, reports) in report.reports.iter() {
        let mut ready = false;
        match synced_clients.0.get(handle) {
            Some(b) => {
                ready = *b;
            }
            None => {}
        }
        if !ready {
            continue;
        }

        let server_sync;

        match confirmations.server_sync.get(handle) {
            Some(x) => {
                server_sync = *x;
            }
            None => {
                server_sync = 0;
            }
        }
        let mut lowest = 0;
        let mut highest = 0;
        let mut length = 0;
        let mut first = true;

        for difference in reports.iter() {
            if difference.client_sync_iteration == server_sync {
                if difference.tick_difference < lowest {
                    lowest = difference.tick_difference;
                }
                if difference.tick_difference > highest {
                    highest = difference.tick_difference;
                }
                length += 1;
                if first {
                    first = false;
                    lowest = difference.tick_difference;
                    highest = difference.tick_difference;
                }
            }
        }
        if length >= DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT as usize {
            let l;
            match limits.limits.get_mut(handle) {
                Some(limit) => {
                    l = limit;
                }
                None => {
                    continue;
                }
            }

            let difference = highest - lowest;

            if difference >= l.min as i16 {
                l.min += 1;
                l.max += 2;
                info!(
                    "Increasing latency min/max to {}/{} [{}].",
                    l.min, l.max, handle
                );
            } else if difference < l.min as i16 - 1 {
                if l.min == DEFAULT_MIN_LATENCY {
                    continue;
                }
                l.min -= 1;
                l.max -= 2;
                info!(
                    "Decreasing latency min/max to {}/{} [{}].",
                    l.min, l.max, handle
                );
            }
        }
    }
}
#[derive(Event, Clone)]
pub struct IncomingReliableClientMessageToReport {
    pub handle: ClientId,
    pub message: ReliableClientMessageBatch,
}
#[derive(Event, Clone)]
pub struct IncomingUnreliableClientMessageToReport {
    pub handle: ClientId,
    pub message: UnreliableClientMessageBatch,
}
pub(crate) fn latency_report_incoming_messages(
    mut events: EventReader<IncomingReliableClientMessageToReport>,
    mut eventsu: EventReader<IncomingUnreliableClientMessageToReport>,

    confirmations: Res<SyncConfirmations>,
    mut latency: ResMut<Latency>,
    stampres: Res<TickRateStamp>,
    synced_clients: Res<ClientsReadyForSync>,
) {
    for message in events.read() {
        if message.message.fixed {
            continue;
        }
        let c: u64;
        match confirmations.incremental.get(&message.handle) {
            Some(x) => {
                c = *x;
            }
            None => {
                c = 0;
            }
        }
        let report = LatencyReport {
            client_sync_iteration: c,
            tick_difference: stampres.get_difference(message.message.stamp),
        };

        let mut ready = false;
        match synced_clients.0.get(&message.handle) {
            Some(b) => {
                ready = *b;
            }
            None => {}
        }
        if !ready {
            continue;
        }

        match latency.reports.get_mut(&message.handle) {
            Some(v) => {
                v.push(report);
            }
            None => {
                latency.reports.insert(message.handle, vec![report]);
            }
        }
    }

    for message in eventsu.read() {
        if message.message.fixed {
            continue;
        }

        let c: u64;
        match confirmations.incremental.get(&message.handle) {
            Some(x) => {
                c = *x;
            }
            None => {
                c = 0;
            }
        }
        let report = LatencyReport {
            client_sync_iteration: c,
            tick_difference: stampres.get_difference(message.message.stamp),
        };
        match latency.reports.get_mut(&message.handle) {
            Some(v) => {
                v.push(report);
            }
            None => {
                latency.reports.insert(message.handle, vec![report]);
            }
        }
    }
}

pub(crate) fn process_sync_confirmation(
    mut messages: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
    mut confirmations: ResMut<SyncConfirmations>,
) {
    for message in messages.read() {
        match &message.message {
            NetworkingClientMessage::SyncConfirmation => {
                match confirmations.incremental.get_mut(&message.handle) {
                    Some(a) => {
                        *a += 1;
                    }
                    None => {
                        confirmations.incremental.insert(message.handle, 1);
                    }
                }
            }
            _ => (),
        }
    }
}

/// Deserializes header of incoming client messages and writes to event.

pub(crate) fn receive_incoming_reliable_client_messages(
    mut server: ResMut<RenetServer>,
    mut queue: Local<HashMap<ClientId, BTreeMap<u64, Vec<IncomingRawReliableClientMessage>>>>,
    stamp: Res<TickRateStamp>,
    mut events: EventWriter<IncomingRawReliableClientMessage>,
    mut report: EventWriter<IncomingReliableClientMessageToReport>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(msg) = server.receive_message(handle, RENET_RELIABLE_ORDERED_ID) {
            match bincode::deserialize::<ReliableClientMessageBatch>(&msg) {
                Ok(message) => {
                    let incoming = IncomingRawReliableClientMessage {
                        message: message.clone(),
                        handle,
                    };
                    let b = stamp.calculate_large(message.stamp);

                    match queue.get_mut(&handle) {
                        Some(q1) => match q1.get_mut(&b) {
                            Some(v) => {
                                v.push(incoming);
                            }
                            None => {
                                q1.insert(b, vec![incoming]);
                            }
                        },
                        None => {
                            queue.insert(handle, BTreeMap::from([(b, vec![incoming])]));
                        }
                    }
                    report.send(IncomingReliableClientMessageToReport {
                        message: message.clone(),
                        handle,
                    });
                }
                Err(_) => {
                    warn!("Received an invalid message.");
                }
            }
        }
        while let Some(msg) = server.receive_message(handle, RENET_RELIABLE_UNORDERED_ID) {
            match bincode::deserialize::<ReliableClientMessageBatch>(&msg) {
                Ok(message) => {
                    let incoming = IncomingRawReliableClientMessage {
                        message: message.clone(),
                        handle,
                    };
                    let b = stamp.calculate_large(message.stamp);

                    match queue.get_mut(&handle) {
                        Some(q1) => match q1.get_mut(&b) {
                            Some(v) => {
                                v.push(incoming);
                            }
                            None => {
                                q1.insert(b, vec![incoming]);
                            }
                        },
                        None => {
                            queue.insert(handle, BTreeMap::from([(b, vec![incoming])]));
                        }
                    }
                }
                Err(_) => {
                    warn!("Received an invalid message 1.");
                }
            }
        }
    }
    // Process one fixed message per tick per client.
    // There are also multiple small low latency messages that arrive and get injected in sync with the client's fixed messages.

    for (_handle, client_messages) in queue.iter_mut() {
        let mut processed_stamps = vec![];
        let mut processed_fixed_message = false;
        for (i, tick_messages) in client_messages.iter() {
            if i > &stamp.large {
                break;
            }

            for msg in tick_messages {
                if msg.message.fixed {
                    processed_fixed_message = true;
                }
                events.send(msg.clone());

                processed_stamps.push(*i);
                if processed_fixed_message {
                    break;
                }
            }
        }

        for i in processed_stamps {
            client_messages.remove(&i);
        }
    }
}
use crate::plugin::{RENET_RELIABLE_UNORDERED_ID, RENET_UNRELIABLE_CHANNEL_ID};

/// Deserializes header of incoming client messages and writes to event.

pub(crate) fn receive_incoming_unreliable_client_messages(
    mut server: ResMut<RenetServer>,
    mut queue: Local<HashMap<ClientId, BTreeMap<u64, Vec<IncomingRawUnreliableClientMessage>>>>,
    stamp: Res<TickRateStamp>,
    mut events: EventWriter<IncomingRawUnreliableClientMessage>,
    mut report: EventWriter<IncomingUnreliableClientMessageToReport>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(msg) = server.receive_message(handle, RENET_UNRELIABLE_CHANNEL_ID) {
            match bincode::deserialize::<UnreliableClientMessageBatch>(&msg) {
                Ok(message) => {
                    let incoming = IncomingRawUnreliableClientMessage {
                        message: message.clone(),
                        handle,
                    };
                    let b = stamp.calculate_large(message.stamp);

                    match queue.get_mut(&handle) {
                        Some(q1) => match q1.get_mut(&b) {
                            Some(v) => {
                                v.push(incoming);
                            }
                            None => {
                                q1.insert(b, vec![incoming]);
                            }
                        },
                        None => {
                            queue.insert(handle, BTreeMap::from([(b, vec![incoming])]));
                        }
                    }
                    report.send(IncomingUnreliableClientMessageToReport {
                        message: message.clone(),
                        handle,
                    });
                }
                Err(_) => {
                    warn!("Received an invalid message.");
                }
            }
        }
    }
    // Process one fixed message per tick per client.
    // There are also multiple small low latency messages that arrive and get injected in sync with the client's fixed messages.

    for (_handle, client_messages) in queue.iter_mut() {
        let mut processed_stamps = vec![];
        for (i, tick_messages) in client_messages.iter() {
            if i > &stamp.large {
                break;
            }
            for msg in tick_messages {
                events.send(msg.clone());
                processed_stamps.push(*i);
            }
        }

        for i in processed_stamps {
            client_messages.remove(&i);
        }
    }
}

/// Event to send reliable messages from server to client. Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event)]
pub struct OutgoingReliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: ClientId,
    pub message: T,
}

/// Event to send unreliable messages from server to client. Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub struct OutgoingUnreliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: ClientId,
    pub message: T,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event, Clone)]
pub struct IncomingRawReliableClientMessage {
    pub handle: ClientId,
    pub message: ReliableClientMessageBatch,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event, Clone)]
pub struct IncomingRawUnreliableClientMessage {
    pub handle: ClientId,
    pub message: UnreliableClientMessageBatch,
}

/// Entity updates are serializable server messages (created with register_reliable_message) that are usually sent as live traffic from a spawned entity.
/// Clients obtain the full state of an entity when these updates are sent through the LoadEntity message.
/// Entity updates allow for replication and construction of the perfect entity state.
/// Entity updates are received when the clients recieve the spawn/load entity message.
/// These entity updates clear each tick and are built for each tick per entity that is getting this data accessed.
#[derive(Resource, Default)]
pub struct EntityUpdates<T: TypeName + Send + Sync + 'static> {
    pub map: HashMap<Entity, Vec<T>>,
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EntityUpdatesSet {
    Write,
    Prepare,
    BuildUpdates,
    Serialize,
    Ready,
}
/// Construct entity updates for a specific entity for this frame. Ie when loading it in for a client.
#[derive(Resource, Default)]
pub struct ConstructEntityUpdates {
    pub entities: HashMap<Entity, bool>,
}
pub(crate) fn clear_construct_entity_updates(mut r: ResMut<ConstructEntityUpdates>) {
    r.entities.clear();
}
pub(crate) fn clear_entity_updates<T: TypeName + Send + Sync + 'static>(
    mut res: ResMut<EntityUpdates<T>>,
) {
    res.map.clear();
}
#[derive(Resource, Default)]
pub struct EntityUpdatesSerialized {
    pub reliable: HashMap<Entity, Vec<Vec<u8>>>,
    pub unreliable: HashMap<Entity, Vec<Vec<u8>>>,
}

pub(crate) fn clear_serialized_entity_updates(mut r: ResMut<EntityUpdatesSerialized>) {
    r.reliable.clear();
    r.unreliable.clear();
}

pub(crate) fn serialize_reliable_entity_updates<T: Serialize + TypeName + Send + Sync + 'static>(
    updates: Res<EntityUpdates<T>>,
    mut serialized: ResMut<EntityUpdatesSerialized>,
    typenames: Res<Typenames>,
) {
    for (entity, updates) in updates.map.iter() {
        for update in updates {
            let net;
            match typenames.reliable_net_types.get(&update.type_name_of()) {
                Some(n) => {
                    net = n;
                }
                None => {
                    warn!(
                        "Couldnt find server reliable type {}",
                        update.type_name_of()
                    );
                    continue;
                }
            }

            match bincode::serialize(update) {
                Ok(c) => {
                    match bincode::serialize(&ReliableMessage {
                        serialized: c,
                        typename_net: *net,
                    }) {
                        Ok(r) => match serialized.reliable.get_mut(entity) {
                            Some(m) => m.push(r),
                            None => {
                                serialized.reliable.insert(*entity, vec![r]);
                            }
                        },
                        Err(_) => {
                            warn!("Couldnt serialize reliable msg.");
                        }
                    }
                }
                Err(_) => {
                    warn!("Couldnt serialize entity update.");
                }
            }
        }
    }
}

pub(crate) fn serialize_unreliable_entity_updates<
    T: Serialize + TypeName + Send + Sync + 'static,
>(
    updates: Res<EntityUpdates<T>>,
    mut serialized: ResMut<EntityUpdatesSerialized>,
    typenames: Res<Typenames>,
) {
    for (entity, updates) in updates.map.iter() {
        for update in updates {
            let net;
            match typenames.unreliable_net_types.get(&update.type_name_of()) {
                Some(n) => {
                    net = n;
                }
                None => {
                    warn!(
                        "Couldnt find server reliable type {}",
                        update.type_name_of()
                    );
                    continue;
                }
            }

            match bincode::serialize(update) {
                Ok(c) => {
                    match bincode::serialize(&UnreliableMessage {
                        serialized: c,
                        typename_net: *net,
                    }) {
                        Ok(r) => match serialized.unreliable.get_mut(entity) {
                            Some(m) => m.push(r),
                            None => {
                                serialized.unreliable.insert(*entity, vec![r]);
                            }
                        },
                        Err(_) => {
                            warn!("Couldnt serialize unreliable msg.");
                        }
                    }
                }
                Err(_) => {
                    warn!("Couldnt serialize u-entity update.");
                }
            }
        }
    }
}
