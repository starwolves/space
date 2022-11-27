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
use bevy::prelude::ResMut;
use std::net::IpAddr;

use crate::plugin::RENET_RELIABLE_CHANNEL_ID;
use crate::server::NetworkingClientServerMessage;

#[cfg(feature = "client")]
pub(crate) fn connect_to_server(
    mut event: EventReader<ConnectToServer>,
    mut commands: Commands,
    preferences: Res<ConnectionPreferences>,
    mut connection: ResMut<Connection>,
) {
    for _ in event.iter() {
        match connection.status {
            ConnectionStatus::None => (),
            ConnectionStatus::Connecting => {
                continue;
            }
            ConnectionStatus::Connected => {
                continue;
            }
        }

        match connection.status {
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

                let mut client = RenetClient::new(
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
                let message = bincode::serialize(&NetworkingClientServerMessage::Awoo).unwrap();
                client.send_message(RENET_RELIABLE_CHANNEL_ID, message);

                commands.insert_resource(client);

                connection.status = ConnectionStatus::Connecting;
            }
            ConnectionStatus::Connecting => {}
            ConnectionStatus::Connected => {}
        }
    }
}
#[cfg(feature = "client")]
#[derive(Default)]
pub enum ConnectionStatus {
    #[default]
    None,
    Connecting,
    Connected,
}

#[cfg(feature = "client")]
#[derive(Default, Resource)]
pub struct Connection {
    pub status: ConnectionStatus,
}
