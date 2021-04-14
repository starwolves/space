use bevy::{ecs::{system::{Commands, Res, ResMut}}, prelude::{EventReader, info}};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::space_core::{functions::{
        on_new_connection::on_new_connection
    }, resources::{all_ordered_cells::AllOrderedCells, authid_i::AuthidI, blackcells_data::BlackcellsData, server_id::ServerId, tick_rate::TickRate, world_environments::WorldEnvironment}};

pub fn handle_network_events(
    mut net: ResMut<NetworkResource>,
    mut reader: EventReader<NetworkEvent>,
    world_environment: Res<WorldEnvironment>,
    tick_rate : Res<TickRate>,
    blackcells_data: Res<BlackcellsData>,
    mut auth_id_i : ResMut<AuthidI>,
    server_id : Res<ServerId>,
    all_ordered_cells: Res<AllOrderedCells>,
    mut commands: Commands
) {

    for event in reader.iter() {

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

                on_new_connection(
                    &mut net,
                     handle, &world_environment,
                      &tick_rate, &blackcells_data, 
                      &mut auth_id_i, 
                      &server_id,
                    &all_ordered_cells,
                    &mut commands
                );


            }
            
            NetworkEvent::Disconnected(_) => {
                info!("New Disconnected!");
            }
        }
    }
    
}