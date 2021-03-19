use std::{fs};

use bevy::{
    prelude::*
};

use bevy_rapier3d::{
    physics::{
        RapierPhysicsPlugin
    }
};

use serde::{Serialize, Deserialize};


use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, MessageChannelMode , MessageChannelSettings, ConnectionChannelsBuilder, ReliableChannelSettings};
use std::{ time::Duration};

mod space_core;

use space_core::resources::world_environments::{WorldEnvironment,WorldEnvironmentRaw};
use space_core::systems::{launch_server::launch_server};

#[derive(Default)]
struct NetworkReader {
    network_events: EventReader<NetworkEvent>,
}

fn main() {


    let current_map_environment_raw_json : String = fs::read_to_string(&DEFAULT_MAP_ENVIRONMENT_LOCATION).expect("main.rs launch_server() Error reading map environment.json file from drive.");
    let current_map_raw_environment : WorldEnvironmentRaw = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs launch_server() Error parsing map environment.json String.");
    let current_map_environment : WorldEnvironment = WorldEnvironment::new(current_map_raw_environment);
    

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(network_setup.system())
        .add_startup_system(launch_server.system())
        .add_resource(NetworkReader::default())
        .add_resource(current_map_environment)
        .add_system(handle_packets.system())
        .add_system_to_stage(stage::PRE_UPDATE, handle_messages_server.system())
        .run();
}

const DEFAULT_MAP_ENVIRONMENT_LOCATION : &str = "content\\maps\\bullseye\\environment.json";

fn handle_messages_server(mut net: ResMut<NetworkResource>) {
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ClientMessage>() {
            info!("ClientMessage received on [{}]: {:?}",handle, client_message);
        }
    }
}


// Start In sync with client
#[derive(Serialize, Deserialize, Debug, Clone)]
enum ClientMessage {
    ConfigMessage(ConfigMessage)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ConfigMessage {
    Awoo,
    WorldEnvironment(WorldEnvironment)
}

const CLIENT_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
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
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

fn network_setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_MESSAGE_RELIABLE)
            .unwrap();
    });
}
// End in sync with client

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    mut state: ResMut<NetworkReader>,
    network_events: Res<Events<NetworkEvent>>,
    world_environment: Res<WorldEnvironment>
) {

    for event in state.network_events.iter(&network_events) {

        info!("New network_events");

        match event {
            NetworkEvent::Packet(_handle, _packet) => {
                info!("New Packet!");
            },
            NetworkEvent::Connected(handle) => {
                
                // https://github.com/smokku/bevy_networking_turbulence/blob/master/examples/channels.rs
                
                info!("New Connection!");

                match net.connections.get_mut(handle) {
                    Some(connection) => {
                        match connection.remote_address() {
                            Some(remote_address) => {
                                info!(
                                    "Incoming connection on [{}] from [{}]",
                                    handle,
                                    remote_address
                                );
                            }
                            None => {
                                panic!("main.rs NetworkEvent::Connected: new connection with a strange remote_address [{}]", handle);
                            }
                        }
                    }
                    None => {
                        panic!("main.rs NetworkEvent::Connected: got packet for non-existing connection [{}]", handle);
                    }
                }

                match net.send_message(*handle, ClientMessage::ConfigMessage(ConfigMessage::WorldEnvironment(*world_environment))) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("Networkhound was unable to send Awoo: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("Networkhound was unable to send Awoo (1): {:?}", err);
                    }
                };

            }
            
            NetworkEvent::Disconnected(_) => {
                info!("New Disconnected!");
            }
        }
    }
    
}
