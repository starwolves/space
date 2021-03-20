use bevy::{ecs::{Res, ResMut}, prelude::{Events, info, warn}};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::space_core::{resources::{network_reader::NetworkReader, world_environments::WorldEnvironment}, structs::network_messages::*};

pub fn handle_network_events(
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