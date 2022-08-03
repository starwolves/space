pub fn on_new_player_connection(
    net_on_new_player_connection: &mut EventWriter<NetPlayerConn>,
    handle: &u64,
    tick_rate: &Res<TickRate>,
    auth_id_i: &mut ResMut<AuthidI>,
    server_id: &Res<ServerId>,
    handle_to_entity: &mut ResMut<HandleToEntity>,
    commands: &mut Commands,
    used_names: &mut ResMut<UsedNames>,
    gridmap_data: &Res<GridmapData>,
    map_data: &Res<MapData>,
    console_commands: &Res<AllConsoleCommands>,
    give_all_rcon: &Res<GiveAllRCON>,
) {
    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::Awoo),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TickRate(
            tick_rate.physics_rate,
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::BlackCellID(
            gridmap_data.blackcell_id,
            gridmap_data.blackcell_blocking_id,
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::OrderedCellsMain(
            gridmap_data.ordered_main_names.clone(),
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::OrderedCellsDetails1(
            gridmap_data.ordered_details1_names.clone(),
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerEntityId(
            server_id.id.to_bits(),
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(
            false,
            "setupUI".to_string(),
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
            "concrete_walking_footsteps".to_string(),
            (1..=39)
                .map(|i| {
                    format!(
                        "/content/audio/footsteps/default/Concrete_Shoes_Walking_step{i}.sample"
                    )
                })
                .collect(),
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
            "concrete_sprinting_footsteps".to_string(),
            [
                4, 5, 7, 9, 10, 12, 13, 14, 15, 16, 17, 20, 21, 22, 23, 24, 25, 27, 28, 30, 31, 32,
                34, 35, 36, 38, 40, 41, 42, 43, 44, 45, 46, 47, 49, 50, 51,
            ]
            .iter()
            .map(|i| {
                format!("/content/audio/footsteps/default/Concrete_Shoes_Running_step{i}.sample")
            })
            .collect(),
        )),
    });

    // Create the actual Bevy entity for the player , with its network handle, authid and softConnected components.

    let connected_player_component = ConnectedPlayer {
        handle: *handle,
        authid: auth_id_i.i,
        rcon: give_all_rcon.give,
        ..Default::default()
    };

    let soft_connected_component = SoftPlayer;

    let mut default_name = "Wolf".to_string() + &used_names.player_i.to_string();

    used_names.player_i += 1;

    while used_names.user_names.contains_key(&default_name) {
        used_names.player_i += 1;
        default_name = "Wolf".to_string() + &used_names.player_i.to_string();
    }

    let persistent_player_data = PersistentPlayerData {
        character_name: "".to_string(),
        user_name: default_name.clone(),
        ..Default::default()
    };

    let player_input = ControllerInput::default();

    auth_id_i.i += 1;

    let player_entity_id = commands
        .spawn()
        .insert_bundle((
            connected_player_component,
            soft_connected_component,
            persistent_player_data,
            player_input,
        ))
        .id();

    used_names.user_names.insert(default_name, player_entity_id);

    handle_to_entity.map.insert(*handle, player_entity_id);
    handle_to_entity.inv_map.insert(player_entity_id, *handle);

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(
            player_entity_id.to_bits(),
        )),
    });

    let console_commands = console_commands.list.clone();

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ConsoleCommands(
            console_commands,
        )),
    });

    let talk_spaces = get_talk_spaces_setupui();

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TalkSpaces(talk_spaces)),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::FinishedInitialization),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::PlaceableItemsSurfaces(
            gridmap_data.placeable_items_cells_list.clone(),
        )),
    });

    net_on_new_player_connection.send(NetPlayerConn {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::NonBlockingCells(
            gridmap_data.non_fov_blocking_cells_list.clone(),
        )),
    });

    for add in map_data.to_net() {
        net_on_new_player_connection.send(NetPlayerConn {
            handle: *handle,
            message: ReliableServerMessage::MapDefaultAddition(add.0, add.1, add.2),
        });
    }
}

use api::{
    connected_player::SoftPlayer,
    data::{ConnectedPlayer, HandleToEntity, ServerId, TickRate},
    gridmap::GridmapData,
    network::{ReliableServerMessage, ServerConfigMessage},
};
use bevy::prelude::{Commands, EventWriter, Res, ResMut};
use console_commands::{commands::AllConsoleCommands, rcon::GiveAllRCON};
use map::map_input::MapData;
use networking::messages::NetPlayerConn;
use pawn::pawn::{ControllerInput, PersistentPlayerData, UsedNames};

use crate::{chat::get_talk_spaces_setupui, connection::AuthidI};
