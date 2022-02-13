use std::{time::Duration, net::SocketAddr};

use bevy::prelude::{ResMut, info};
use bevy_networking_turbulence::{MessageChannelSettings, MessageChannelMode, ReliableChannelSettings, NetworkResource, ConnectionChannelsBuilder};

use crate::space_core::ecs::networking::resources::{ReliableServerMessage, ReliableClientMessage, UnreliableServerMessage, UnreliableClientMessage};


const SERVER_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 163840,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 32765,
    },
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

const CLIENT_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 163840,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 64,
    packet_buffer_size: 64,
};

const SERVER_MESSAGE_UNRELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 2,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 256,
    packet_buffer_size: 256,
};

const CLIENT_MESSAGE_UNRELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 3,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 64,
    packet_buffer_size: 64,
};

const SERVER_PORT: u16 = 57713;

pub fn startup_launch_server(
    mut net: ResMut<NetworkResource>, 
) {

    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ReliableServerMessage>(SERVER_MESSAGE_RELIABLE)
            .unwrap();
        builder
            .register::<ReliableClientMessage>(CLIENT_MESSAGE_RELIABLE)
            .unwrap();
        builder
            .register::<UnreliableServerMessage>(SERVER_MESSAGE_UNRELIABLE)
            .unwrap();
        builder
            .register::<UnreliableClientMessage>(CLIENT_MESSAGE_UNRELIABLE)
            .unwrap();
    });

    let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("main.rs launch_server() Error cannot find IP address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

    net.listen(socket_address, None, None);

    info!("Server is live.");
}
