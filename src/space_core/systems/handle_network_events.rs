use bevy::{ecs::{Res, ResMut}, prelude::{Events, info, warn}};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::space_core::{
    resources::{
        network_reader::NetworkReader,
        world_environments::WorldEnvironment,
        tick_rate::TickRate
    }, 
    structs::network_messages::*,
    functions::{
        on_new_connection::on_new_connection
    }
};

pub fn handle_network_events(
    mut net: ResMut<NetworkResource>,
    mut state: ResMut<NetworkReader>,
    network_events: Res<Events<NetworkEvent>>,
    world_environment: Res<WorldEnvironment>,
    tick_rate : Res<TickRate>
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
                                panic!("handle_network_events.rs NetworkEvent::Connected: new connection with a strange remote_address [{}]", handle);
                            }
                        }
                    }
                    None => {
                        panic!("handle_network_events.rs NetworkEvent::Connected: got packet for non-existing connection [{}]", handle);
                    }
                }

                match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::WorldEnvironment(*world_environment))) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("handle_network_events.rs NetworkEvent::Connected: was unable to send WorldEnvironment: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("handle_network_events.rs NetworkEvent::Connected: was unable to send WorldEnvironment (1): {:?}", err);
                    }
                };

                on_new_connection();

                


            }
            
            NetworkEvent::Disconnected(_) => {
                info!("New Disconnected!");
            }
        }
    }
    
}