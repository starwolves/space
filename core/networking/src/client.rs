use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::info;
use bevy_renet::renet::{
    ChannelConfig, ClientAuthentication, ConnectToken, ReliableChannelConfig, RenetClient,
    RenetConnectionConfig,
};

use crate::{plugin::PRIVATE_KEY, server::PROTOCOL_ID};

#[cfg(feature = "client")]
pub const CLIENT_PORT: u16 = 56613;

/// Resource containing needed for the server.
#[cfg(feature = "client")]
#[derive(Default)]
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

                let encryption_key = *PRIVATE_KEY;
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
                    ChannelConfig::Block(Default::default()),
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

                let token = ConnectToken::generate(
                    current_time,
                    PROTOCOL_ID,
                    300,
                    client_id,
                    7,
                    vec![socket_address],
                    None,
                    &encryption_key,
                )
                .unwrap();
                info!("Establishing connection with [{}]", socket_address);

                commands.insert_resource(
                    RenetClient::new(
                        current_time,
                        socket,
                        client_id,
                        connection_config,
                        ClientAuthentication::Secure {
                            connect_token: token,
                        },
                    )
                    .unwrap(),
                );

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
#[derive(Default)]
pub struct Connection {
    pub status: ConnectionStatus,
}
