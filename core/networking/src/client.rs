use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::info;
use bevy_renet::renet::{
    ChannelConfig, ClientAuthentication, ConnectToken, ReliableChannelConfig, RenetClient,
    RenetConnectionConfig, NETCODE_KEY_BYTES,
};

use crate::{plugin::PRIVATE_KEY, server::PROTOCOL_ID};

#[cfg(feature = "client")]
pub const CLIENT_PORT: u16 = 56613;

#[cfg(feature = "client")]
pub struct ServerAddress {
    pub address: SocketAddr,
}
#[cfg(feature = "client")]
pub fn startup_client_listen_connections(
    server_address: ServerAddress,
    mut encryption_key: [u8; NETCODE_KEY_BYTES],
) -> RenetClient {
    if encryption_key.len() == 0 {
        encryption_key = *PRIVATE_KEY;
    }

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

    // This connect token should come from another system, NOT generated from the client.
    // Usually from a matchmaking system
    // The client should not have access to the PRIVATE_KEY from the server.
    let token = ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        300,
        client_id,
        7,
        vec![server_address.address],
        None,
        &encryption_key,
    )
    .unwrap();
    info!("Networkhound is connecting to {}.", server_address.address);

    RenetClient::new(
        current_time,
        socket,
        client_id,
        connection_config,
        ClientAuthentication::Secure {
            connect_token: token,
        },
    )
    .unwrap()
}
