use bevy_app::EventWriter;
use bevy_ecs::system::{Commands, Res, ResMut};

use crate::space::core::{
    chat::functions::get_talk_spaces_setupui,
    configuration::resources::{ServerId, TickRate},
    connected_player::{
        components::{ConnectedPlayer, SoftPlayer},
        events::NetOnNewPlayerConnection,
        resources::HandleToEntity,
    },
    console_commands::systems::get_console_commands,
    gridmap::resources::GridmapData,
    map::resources::MapData,
    networking::resources::{ReliableServerMessage, ServerConfigMessage},
    pawn::{
        components::{ControllerInput, PersistentPlayerData},
        resources::{AuthidI, UsedNames},
    },
};

pub fn on_new_player_connection(
    net_on_new_player_connection: &mut EventWriter<NetOnNewPlayerConnection>,
    handle: &u32,
    tick_rate: &Res<TickRate>,
    auth_id_i: &mut ResMut<AuthidI>,
    server_id: &Res<ServerId>,
    handle_to_entity: &mut ResMut<HandleToEntity>,
    commands: &mut Commands,
    used_names: &mut ResMut<UsedNames>,
    gridmap_data: &Res<GridmapData>,
    map_data: &Res<MapData>,
) {
    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::Awoo),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TickRate(
            tick_rate.rate,
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::BlackCellID(
            gridmap_data.blackcell_id,
            gridmap_data.blackcell_blocking_id,
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::OrderedCellsMain(
            gridmap_data.ordered_main_names.clone(),
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::OrderedCellsDetails1(
            gridmap_data.ordered_details1_names.clone(),
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ServerEntityId(
            server_id.id.to_bits(),
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(
            false,
            "setupUI".to_string(),
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
            "concrete_walking_footsteps".to_string(),
            vec![
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step1.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step2.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step3.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step4.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step5.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step6.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step7.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step8.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step9.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step10.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step11.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step12.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step13.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step14.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step15.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step16.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step17.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step18.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step19.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step20.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step21.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step22.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step23.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step24.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step25.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step26.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step27.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step28.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step29.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step30.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step31.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step32.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step33.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step34.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step35.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step36.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step37.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step38.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Walking_step39.sample".to_string(),
            ],
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::RepeatingSFX(
            "concrete_sprinting_footsteps".to_string(),
            vec![
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step4.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step5.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step7.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step9.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step10.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step12.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step13.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step14.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step15.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step16.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step17.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step20.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step21.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step22.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step23.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step24.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step25.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step27.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step28.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step30.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step31.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step32.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step34.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step35.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step36.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step38.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step40.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step41.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step42.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step43.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step44.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step45.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step46.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step47.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step49.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step50.sample".to_string(),
                "/content/audio/footsteps/default/Concrete_Shoes_Running_step51.sample".to_string(),
            ],
        )),
    });

    // Create the actual Bevy entity for the player , with its network handle, authid and softConnected components.

    let connected_player_component = ConnectedPlayer {
        handle: *handle,
        authid: auth_id_i.i,
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

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(
            player_entity_id.to_bits(),
        )),
    });

    let console_commands = get_console_commands();

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ConsoleCommands(
            console_commands,
        )),
    });

    let talk_spaces = get_talk_spaces_setupui();

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TalkSpaces(talk_spaces)),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::FinishedInitialization),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::PlaceableItemsSurfaces(
            gridmap_data.placeable_items_cells_list.clone(),
        )),
    });

    net_on_new_player_connection.send(NetOnNewPlayerConnection {
        handle: *handle,
        message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::NonBlockingCells(
            gridmap_data.non_fov_blocking_cells_list.clone(),
        )),
    });

    for add in map_data.to_net() {
        net_on_new_player_connection.send(NetOnNewPlayerConnection {
            handle: *handle,
            message: ReliableServerMessage::MapDefaultAddition(add.0, add.1, add.2),
        });
    }
}
