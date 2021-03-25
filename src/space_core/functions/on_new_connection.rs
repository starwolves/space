use bevy::{ecs::{Commands, Res, ResMut}, prelude::warn};
use bevy_networking_turbulence::NetworkResource;
use crate::space_core::{components::{connected_player::ConnectedPlayer, soft_connected::SoftConnected}, resources::{authid_i::AuthidI, blackcells_data::BlackcellsData, server_id::ServerId, tick_rate::TickRate, world_environments::WorldEnvironment}, structs::network_messages::*};


pub fn on_new_connection(
    net : &mut ResMut<NetworkResource>, 
    handle : &u32, world_environment: &Res<WorldEnvironment>, 
    tick_rate: &Res<TickRate>,
    blackcells_data: &Res<BlackcellsData>,
    auth_id_i : &mut ResMut<AuthidI>,
    server_id : &Res<ServerId>,
    commands: &mut Commands
) {

    match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::WorldEnvironment(**world_environment))) {
        Ok(msg) => match msg {
            Some(msg) => {
                warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send WorldEnvironment: {:?}", msg);
            }
            None => {}
        },
        Err(err) => {
            warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send WorldEnvironment (1): {:?}", err);
        }
    };

    match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::TickRate(tick_rate.rate))) {
        Ok(msg) => match msg {
            Some(msg) => {
                warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send TickRate: {:?}", msg);
            }
            None => {}
        },
        Err(err) => {
            warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send TickRate (1): {:?}", err);
        }
    };

    match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::HandleId(*handle))) {
        Ok(msg) => match msg {
            Some(msg) => {
                warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send HandleId: {:?}", msg);
            }
            None => {}
        },
        Err(err) => {
            warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send HandleId (1): {:?}", err);
        }
    };

    match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::BlackCellID(blackcells_data.blackcell_id, blackcells_data.blackcell_blocking_id))) {
        Ok(msg) => match msg {
            Some(msg) => {
                warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send BlackCellID: {:?}", msg);
            }
            None => {}
        },
        Err(err) => {
            warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send BlackCellID (1): {:?}", err);
        }
    };

    match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::ChangeScene(false, "setupUI".to_string()))) {
        Ok(msg) => match msg {
            Some(msg) => {
                warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send ChangeScene: {:?}", msg);
            }
            None => {}
        },
        Err(err) => {
            warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send ChangeScene (1): {:?}", err);
        }
    };

    match net.send_message(*handle, ReliableServerMessage::ConfigMessage(ConfigMessage::ServerEntityId(server_id.id))) {
        Ok(msg) => match msg {
            Some(msg) => {
                warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send ServerEntityId: {:?}", msg);
            }
            None => {}
        },
        Err(err) => {
            warn!("on_new_connection.rs NetworkEvent::Connected: was unable to send ServerEntityId (1): {:?}", err);
        }
    };

    // Create the actual Bevy entity for the player , with its network handle, authid and softConnected components.
    
    let connected_player_component = ConnectedPlayer {
        handle: *handle,
        authid: auth_id_i.i
    };

    let soft_connected_component = SoftConnected {};

    auth_id_i.i+=1;

    commands.spawn((connected_player_component, soft_connected_component));
    









    
    
}
