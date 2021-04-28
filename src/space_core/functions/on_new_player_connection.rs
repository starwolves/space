use bevy::{ecs::{system::{Commands, Res, ResMut}}, prelude::EventWriter};
use crate::space_core::{components::{
        connected_player::ConnectedPlayer,
        persistent_player_data::PersistentPlayerData,
        soft_player::SoftPlayer,
    }, events::net::net_on_new_player_connection::NetOnNewPlayerConnection, resources::{
        all_ordered_cells::AllOrderedCells,
        authid_i::AuthidI,
        blackcells_data::BlackcellsData,
        server_id::ServerId, 
        tick_rate::TickRate,
        world_environments::WorldEnvironment,
        handle_to_entity::HandleToEntity
    }, structs::network_messages::*};


pub fn on_new_player_connection(
    net_on_new_player_connection : &mut EventWriter<NetOnNewPlayerConnection>,
    handle : &u32, world_environment: &Res<WorldEnvironment>, 
    tick_rate: &Res<TickRate>,
    blackcells_data: &Res<BlackcellsData>,
    auth_id_i : &mut ResMut<AuthidI>,
    server_id : &Res<ServerId>,
    all_ordered_cells : &Res<AllOrderedCells>,
    handle_to_entity : &mut ResMut<HandleToEntity>,
    commands: &mut Commands
) {
    
    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle : *handle,
        message : ReliableServerMessage::ConfigMessage(ServerConfigMessage::Awoo)
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle : *handle,
        message : ReliableServerMessage::ConfigMessage(ServerConfigMessage::WorldEnvironment(**world_environment))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TickRate(tick_rate.rate))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::BlackCellID(blackcells_data.blackcell_id, blackcells_data.blackcell_blocking_id))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::OrderedCellsMain(all_ordered_cells.main.clone()))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::OrderedCellsDetails1(all_ordered_cells.details1.clone()))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle:*handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(false, "setupUI".to_string()))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerEntityId(server_id.id.id()))
    });

    // Create the actual Bevy entity for the player , with its network handle, authid and softConnected components.
    
    let connected_player_component = ConnectedPlayer {
        handle: *handle,
        authid: auth_id_i.i
    };

    let soft_connected_component = SoftPlayer;

    let persistent_player_data = PersistentPlayerData {
        character_name: "".to_string()
    };

    auth_id_i.i+=1;

    let player_entity_id = commands.spawn().insert_bundle((
        connected_player_component,
        soft_connected_component,
        persistent_player_data
    )).id();
    
    handle_to_entity.map.insert(*handle, player_entity_id);
    handle_to_entity.inv_map.insert(player_entity_id.id(), *handle);

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(player_entity_id.id()))
    });
    
}
