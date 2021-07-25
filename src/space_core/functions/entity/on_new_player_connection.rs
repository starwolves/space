use bevy::{ecs::{system::{Commands, Res, ResMut}}, prelude::EventWriter};
use crate::space_core::{components::{
        connected_player::ConnectedPlayer,
        persistent_player_data::PersistentPlayerData,
        soft_player::SoftPlayer,
    }, events::net::net_on_new_player_connection::NetOnNewPlayerConnection, functions::console_commands::get_console_commands::get_console_commands, resources::{
        all_ordered_cells::AllOrderedCells,
        authid_i::AuthidI,
        blackcells_data::BlackcellsData,
        server_id::ServerId, 
        tick_rate::TickRate,
        handle_to_entity::HandleToEntity
    }, structs::network_messages::*};


pub fn on_new_player_connection(
    net_on_new_player_connection : &mut EventWriter<NetOnNewPlayerConnection>,
    handle : &u32, 
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
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerEntityId(server_id.id.to_bits()))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle:*handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(false, "setupUI".to_string()))
    });

    

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
            "concrete_walking_footsteps".to_string(),
            vec![
                "Concrete_Shoes_Walking_step1".to_string(),
                "Concrete_Shoes_Walking_step2".to_string(),
                "Concrete_Shoes_Walking_step3".to_string(),
                "Concrete_Shoes_Walking_step4".to_string(),
                "Concrete_Shoes_Walking_step5".to_string(),
                "Concrete_Shoes_Walking_step6".to_string(),
                "Concrete_Shoes_Walking_step7".to_string(),
                "Concrete_Shoes_Walking_step8".to_string(),
                "Concrete_Shoes_Walking_step9".to_string(),
                "Concrete_Shoes_Walking_step10".to_string(),
                "Concrete_Shoes_Walking_step11".to_string(),
                "Concrete_Shoes_Walking_step12".to_string(),
                "Concrete_Shoes_Walking_step13".to_string(),
                "Concrete_Shoes_Walking_step14".to_string(),
                "Concrete_Shoes_Walking_step15".to_string(),
                "Concrete_Shoes_Walking_step16".to_string(),
                "Concrete_Shoes_Walking_step17".to_string(),
                "Concrete_Shoes_Walking_step18".to_string(),
                "Concrete_Shoes_Walking_step19".to_string(),
                "Concrete_Shoes_Walking_step20".to_string(),
                "Concrete_Shoes_Walking_step21".to_string(),
                "Concrete_Shoes_Walking_step22".to_string(),
                "Concrete_Shoes_Walking_step23".to_string(),
                "Concrete_Shoes_Walking_step24".to_string(),
                "Concrete_Shoes_Walking_step25".to_string(),
                "Concrete_Shoes_Walking_step26".to_string(),
                "Concrete_Shoes_Walking_step27".to_string(),
                "Concrete_Shoes_Walking_step28".to_string(),
                "Concrete_Shoes_Walking_step29".to_string(),
                "Concrete_Shoes_Walking_step30".to_string(),
                "Concrete_Shoes_Walking_step31".to_string(),
                "Concrete_Shoes_Walking_step32".to_string(),
                "Concrete_Shoes_Walking_step33".to_string(),
                "Concrete_Shoes_Walking_step34".to_string(),
                "Concrete_Shoes_Walking_step35".to_string(),
                "Concrete_Shoes_Walking_step36".to_string(),
                "Concrete_Shoes_Walking_step37".to_string(),
                "Concrete_Shoes_Walking_step38".to_string(),
                "Concrete_Shoes_Walking_step39".to_string(),
            ]
        ))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
            "concrete_sprinting_footsteps".to_string(),
            vec![
                "Concrete_Shoes_Running_step4".to_string(),
                "Concrete_Shoes_Running_step5".to_string(),
                "Concrete_Shoes_Running_step7".to_string(),
                "Concrete_Shoes_Running_step9".to_string(),
                "Concrete_Shoes_Running_step10".to_string(),
                "Concrete_Shoes_Running_step12".to_string(),
                "Concrete_Shoes_Running_step13".to_string(),
                "Concrete_Shoes_Running_step14".to_string(),
                "Concrete_Shoes_Running_step15".to_string(),
                "Concrete_Shoes_Running_step16".to_string(),
                "Concrete_Shoes_Running_step17".to_string(),
                "Concrete_Shoes_Running_step20".to_string(),
                "Concrete_Shoes_Running_step21".to_string(),
                "Concrete_Shoes_Running_step22".to_string(),
                "Concrete_Shoes_Running_step23".to_string(),
                "Concrete_Shoes_Running_step24".to_string(),
                "Concrete_Shoes_Running_step25".to_string(),
                "Concrete_Shoes_Running_step27".to_string(),
                "Concrete_Shoes_Running_step28".to_string(),
                "Concrete_Shoes_Running_step30".to_string(),
                "Concrete_Shoes_Running_step31".to_string(),
                "Concrete_Shoes_Running_step32".to_string(),
                "Concrete_Shoes_Running_step34".to_string(),
                "Concrete_Shoes_Running_step35".to_string(),
                "Concrete_Shoes_Running_step36".to_string(),
                "Concrete_Shoes_Running_step38".to_string(),
                "Concrete_Shoes_Running_step40".to_string(),
                "Concrete_Shoes_Running_step41".to_string(),
                "Concrete_Shoes_Running_step42".to_string(),
                "Concrete_Shoes_Running_step43".to_string(),
                "Concrete_Shoes_Running_step44".to_string(),
                "Concrete_Shoes_Running_step45".to_string(),
                "Concrete_Shoes_Running_step46".to_string(),
                "Concrete_Shoes_Running_step47".to_string(),
                "Concrete_Shoes_Running_step49".to_string(),
                "Concrete_Shoes_Running_step50".to_string(),
                "Concrete_Shoes_Running_step51".to_string()
            ]
        ))
    });

    // Create the actual Bevy entity for the player , with its network handle, authid and softConnected components.
    
    let connected_player_component = ConnectedPlayer {
        handle: *handle,
        authid: auth_id_i.i,
        rcon : false,
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
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(player_entity_id.to_bits()))
    });

    let console_commands = get_console_commands();

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ConsoleCommands(console_commands))
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection{
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::FinishedInitialization)
    });
    
}
