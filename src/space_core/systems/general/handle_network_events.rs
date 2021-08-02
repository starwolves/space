use bevy::{ecs::{system::{Commands, Res, ResMut}}, prelude::{EventReader, EventWriter, Query, info}};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::net::net_on_new_player_connection::NetOnNewPlayerConnection, functions::entity::{on_new_player_connection::on_new_player_connection, on_player_disconnect::on_player_disconnect}, resources::{
        all_ordered_cells::AllOrderedCells,
        authid_i::AuthidI,
        blackcells_data::BlackcellsData,
        server_id::ServerId,
        tick_rate::TickRate,
        handle_to_entity::HandleToEntity
    }};

pub fn handle_network_events(
    mut net: ResMut<NetworkResource>,
    tick_rate : Res<TickRate>,
    blackcells_data: Res<BlackcellsData>,
    mut auth_id_i : ResMut<AuthidI>,
    server_id : Res<ServerId>,
    all_ordered_cells: Res<AllOrderedCells>,
    mut handle_to_entity : ResMut<HandleToEntity>,
    mut commands: Commands,
    mut reader: EventReader<NetworkEvent>,
    mut net_on_new_player_connection : EventWriter<NetOnNewPlayerConnection>,
    mut connected_players : Query<&mut ConnectedPlayer>,
) {

    for event in reader.iter() {
        
        match event {
            NetworkEvent::Packet(_handle, _packet) => {
                info!("New Packet!");
            },
            NetworkEvent::Connected(handle) => {
                
                // https://github.com/smokku/bevy_networking_turbulence/blob/master/examples/channels.rs

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

                on_new_player_connection(
                    &mut net_on_new_player_connection,
                    handle,
                    &tick_rate,
                    &blackcells_data, 
                    &mut auth_id_i, 
                    &server_id,
                    &all_ordered_cells,
                    &mut handle_to_entity,
                    &mut commands
                );


            }
            
            NetworkEvent::Disconnected(handle) => {
                info!("[{}] disconnected!", handle);
                on_player_disconnect(
                    *handle,
                    &mut handle_to_entity,
                    &mut connected_players,
                );
            }
            NetworkEvent::Error(_handle, _err) => {
                //warn!("NetworkEvent error [{}] : {:?}", _handle, _err);
            }
        }
    }
    
}
