use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::{info, Resource};
use bevy_renet::renet::{
    ChannelConfig, ClientAuthentication, ReliableChannelConfig, RenetClient, RenetConnectionConfig,
};

use crate::server::PROTOCOL_ID;

#[cfg(feature = "client")]
pub const CLIENT_PORT: u16 = 56613;

/// Resource containing needed for the server.
#[cfg(feature = "client")]
#[derive(Default, Resource)]
pub struct ConnectionPreferences {
    pub account_name: String,
    pub server_address: String,
}

/// Event that triggers a new server connection.
#[cfg(feature = "client")]
pub struct ConnectToServer;

use crate::server::SERVER_PORT;
use bevy::prelude::warn;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::Res;
use std::net::IpAddr;

use crate::server::NetworkingClientMessage;
use bevy::prelude::ResMut;

#[cfg(feature = "client")]
pub(crate) fn connect_to_server(
    mut event: EventReader<ConnectToServer>,
    mut commands: Commands,
    preferences: Res<ConnectionPreferences>,
    mut connection_state: ResMut<Connection>,
    mut client: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
) {
    for _ in event.iter() {
        match connection_state.status {
            ConnectionStatus::None => {
                let address;
                let port;

                match preferences.server_address.split_once(":") {
                    Some((ip, port_str)) => {
                        address = ip;
                        match port_str.parse::<u16>() {
                            Ok(p) => {
                                port = p;
                            }
                            Err(_rr) => {
                                warn!("Couldn't connect: couldn't parse port.");
                                continue;
                            }
                        };
                    }
                    None => {
                        address = &preferences.server_address;
                        port = SERVER_PORT
                    }
                }

                let ip_address;

                match address.parse::<IpAddr>() {
                    Ok(add) => {
                        ip_address = add;
                    }
                    Err(_) => {
                        warn!("Couldn't connect: invalid server address.");
                        continue;
                    }
                }

                let socket_address: SocketAddr = SocketAddr::new(ip_address, port as u16);
                let socket = UdpSocket::bind(
                    local_ipaddress::get().unwrap_or_default() + ":" + &CLIENT_PORT.to_string(),
                )
                .unwrap();

                let channels_config = vec![
                    ChannelConfig::Reliable(ReliableChannelConfig {
                        packet_budget: 6000,
                        max_message_size: 5900,
                        ..Default::default()
                    }),
                    ChannelConfig::Unreliable(Default::default()),
                    ChannelConfig::Chunk(Default::default()),
                ];

                let connection_config = RenetConnectionConfig {
                    send_channels_config: channels_config.clone(),
                    receive_channels_config: channels_config,

                    ..Default::default()
                };
                let current_time = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                let client_id = current_time.as_millis() as u64;

                info!("Establishing connection with [{}]", socket_address);

                let renet_client = RenetClient::new(
                    current_time,
                    socket,
                    connection_config,
                    ClientAuthentication::Unsecure {
                        protocol_id: PROTOCOL_ID,
                        client_id: client_id,
                        server_addr: socket_address,
                        user_data: None,
                    },
                )
                .unwrap();

                client.send(OutgoingReliableClientMessage {
                    message: NetworkingClientMessage::Account(preferences.account_name.clone()),
                });
                commands.insert_resource(renet_client);

                connection_state.status = ConnectionStatus::Connecting;
            }
            ConnectionStatus::Connecting => {
                continue;
            }
            ConnectionStatus::Connected => {
                continue;
            }
        }
    }
}

#[cfg(feature = "client")]
#[derive(Default, Resource)]
pub struct Connection {
    pub status: ConnectionStatus,
}

#[cfg(feature = "client")]
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConnectionStatus {
    #[default]
    None,
    Connecting,
    Connected,
}

use bevy::prelude::EventWriter;

/// System run run_if with iyes_loopless
#[cfg(feature = "client")]
pub fn connected(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connected)
}
/// System run run_if with iyes_loopless. The earliest server messages (for setup_ui, boarding etc.)
/// come in while in the connecting stage.
#[cfg(feature = "client")]
pub fn is_client_connected(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connecting)
        || matches!(connection.status, ConnectionStatus::Connected)
}
use crate::messaging::ReliableMessage;
use crate::messaging::Typenames;
use crate::plugin::RENET_RELIABLE_CHANNEL_ID;

use serde::Serialize;
use typename::TypeName;
/// Serializes and sends the outgoing reliable client messages.
#[cfg(any(feature = "client"))]
pub(crate) fn send_outgoing_reliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableClientMessage<T>>,
    mut client: ResMut<RenetClient>,
    typenames: Res<Typenames>,
) {
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
                warn!("Couldnt find client reliable type");
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

        match bincode::serialize(&ReliableMessage {
            serialized: bin,
            typename_net: *net,
        }) {
            Ok(bits) => {
                client.send_message(RENET_RELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize reliable message.");
                continue;
            }
        }
    }
}
use crate::messaging::UnreliableMessage;

/// Serializes and sends the outgoing unreliable client messages.
#[cfg(any(feature = "client"))]
pub(crate) fn send_outgoing_unreliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableClientMessage<T>>,
    mut client: ResMut<RenetClient>,
    typenames: Res<Typenames>,
) {
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

        match bincode::serialize(&UnreliableMessage {
            serialized: bin,
            typename_net: *net,
        }) {
            Ok(bits) => {
                client.send_message(RENET_UNRELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                continue;
            }
        }
    }
}
use serde::Deserialize;

#[cfg(feature = "client")]
pub(crate) fn deserialize_incoming_unreliable_server_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawUnreliableServerMessage>,
    mut outgoing: EventWriter<IncomingUnreliableServerMessage<T>>,
    typenames: Res<Typenames>,
) {
    for event in incoming_raw.iter() {
        match get_unreliable_message::<T>(
            &typenames,
            event.message.typename_net,
            &event.message.serialized,
        ) {
            Some(data) => {
                outgoing.send(IncomingUnreliableServerMessage { message: data });
            }
            None => {}
        }
    }
}
use crate::messaging::get_reliable_message;

#[cfg(feature = "client")]
pub(crate) fn deserialize_incoming_reliable_server_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawReliableServerMessage>,
    mut outgoing: EventWriter<IncomingReliableServerMessage<T>>,
    typenames: Res<Typenames>,
) {
    for event in incoming_raw.iter() {
        match get_reliable_message::<T>(
            &typenames,
            event.message.typename_net,
            &event.message.serialized,
        ) {
            Some(data) => {
                outgoing.send(IncomingReliableServerMessage { message: data });
            }
            None => {}
        }
    }
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[cfg(feature = "client")]
pub struct IncomingReliableServerMessage<T: TypeName + Send + Sync + Serialize> {
    pub message: T,
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[cfg(feature = "client")]
pub struct IncomingUnreliableServerMessage<T: TypeName + Send + Sync + Serialize> {
    pub message: T,
}
use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;

/// Dezerializes incoming server messages and writes to event.
#[cfg(feature = "client")]
pub(crate) fn receive_incoming_unreliable_server_messages(
    mut events: EventWriter<IncomingRawUnreliableServerMessage>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(RENET_UNRELIABLE_CHANNEL_ID) {
        match bincode::deserialize::<UnreliableMessage>(&message) {
            Ok(msg) => {
                events.send(IncomingRawUnreliableServerMessage { message: msg });
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }
}

/// Deserializes incoming server messages and writes to event.
#[cfg(feature = "client")]
pub(crate) fn receive_incoming_reliable_server_messages(
    mut events: EventWriter<IncomingRawReliableServerMessage>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(RENET_RELIABLE_CHANNEL_ID) {
        match bincode::deserialize::<ReliableMessage>(&message) {
            Ok(msg) => {
                events.send(IncomingRawReliableServerMessage { message: msg });
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }
}

/// Event to send unreliable messages from client to server.
#[cfg(feature = "client")]
pub struct OutgoingUnreliableClientMessage<T: TypeName + Send + Sync + 'static> {
    pub message: T,
}
/// Event to send reliable messages from client to server.
#[cfg(feature = "client")]
pub struct OutgoingReliableClientMessage<T: TypeName + Send + Sync + 'static> {
    pub message: T,
}

/// Event to when received reliable message from server. Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].

#[cfg(feature = "client")]
pub(crate) struct IncomingRawReliableServerMessage {
    pub message: ReliableMessage,
}

/// Event to when received reliable message from server. Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].

#[cfg(feature = "client")]
pub(crate) struct IncomingRawUnreliableServerMessage {
    pub message: UnreliableMessage,
}

/// Returns an option containing the desired unreliable netcode message.
#[cfg(feature = "client")]
pub fn get_unreliable_message<T: TypeName + Serialize + for<'a> Deserialize<'a>>(
    typenames: &Res<Typenames>,
    identifier: u8,
    message: &[u8],
) -> Option<T> {
    match typenames.unreliable_net_types.get(&T::type_name()) {
        Some(i) => {
            if &identifier == i {
                match bincode::deserialize::<T>(message) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        warn!("Couldnt serialize message!");
                        None
                    }
                }
            } else {
                None
            }
        }
        None => {
            warn!("Couldnt find reliable net type.");
            None
        }
    }
}
use crate::server::NetworkingServerMessage;

/// Confirms connection with server.
#[cfg(feature = "client")]
pub(crate) fn confirm_connection(
    mut client1: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut connected_state: ResMut<Connection>,
) {
    for message in client1.iter() {
        let player_message = message.message.clone();
        match player_message {
            NetworkingServerMessage::Awoo => {
                connected_state.status = ConnectionStatus::Connected;
                info!("Connection approved.");
            }
        }
    }
}
