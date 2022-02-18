pub mod resources;

use std::net::SocketAddr;

use bevy::{
    ecs::system::{Commands, Res, ResMut},
    prelude::{info, warn, EventReader, EventWriter, Query},
};
use bevy_networking_turbulence::{ConnectionChannelsBuilder, NetworkEvent, NetworkResource};

use crate::space::core::{
    configuration::resources::{ServerId, TickRate},
    gridmap::resources::{GridmapData, Vec3Int},
    health::resources::ClientHealthUICache,
    inventory::events::{
        InputDropCurrentItem, InputSwitchHands, InputTakeOffItem, InputThrowItem,
        InputUseWorldItem, InputWearItem,
    },
    map::events::{InputMap, InputMapChangeDisplayMode, InputMapRequestDisplayModes, MapInput},
    networking::resources::{
        ReliableClientMessage, ReliableServerMessage, UnreliableClientMessage,
        UnreliableServerMessage, CLIENT_MESSAGE_RELIABLE, CLIENT_MESSAGE_UNRELIABLE,
        SERVER_MESSAGE_RELIABLE, SERVER_MESSAGE_UNRELIABLE, SERVER_PORT,
    },
    pawn::{
        components::{ConnectedPlayer, PersistentPlayerData, PlayerInput},
        events::{
            InputAltItemAttack, InputAttackCell, InputAttackEntity, InputBuildGraphics,
            InputChatMessage, InputConsoleCommand, InputExamineEntity, InputExamineMap,
            InputMouseAction, InputMouseDirectionUpdate, InputMovementInput, InputSceneReady,
            InputSelectBodyPart, InputSprinting, InputTabAction, InputTabDataEntity,
            InputTabDataMap, InputToggleAutoMove, InputToggleCombatMode, InputUIInput,
            InputUIInputTransmitText, InputUserName, NetOnNewPlayerConnection,
            TextTreeInputSelection,
        },
        functions::{
            on_new_player_connection::on_new_player_connection,
            on_player_disconnect::on_player_disconnect,
        },
        resources::{AuthidI, HandleToEntity, UsedNames},
    },
};
use crate::space::{
    core::{
        entity::events::{NetLoadEntity, NetSendEntityUpdates, NetShowcase, NetUnloadEntity},
        gridmap::events::{NetGridmapUpdates, NetProjectileFOV},
        health::events::NetHealthUpdate,
        inventory::events::{
            NetDropCurrentItem, NetPickupWorldItem, NetSwitchHands, NetTakeOffItem, NetThrowItem,
            NetWearItem,
        },
        map::events::NetRequestDisplayModes,
        pawn::events::{
            NetChatMessage, NetConsoleCommands, NetDoneBoarding, NetExamineEntity, NetOnBoarding,
            NetOnSetupUI, NetOnSpawning, NetSendServerTime, NetSendWorldEnvironment, NetTabData,
            NetUIInputTransmitData, NetUpdatePlayerCount, NetUserName,
        },
    },
    entities::construction_tool_admin::events::NetConstructionTool,
};

use super::{
    atmospherics::events::{
        NetAtmosphericsNotices, NetMapDisplayAtmospherics, NetMapHoverAtmospherics,
    },
    map::resources::MapData,
};

pub fn startup_listen_connections(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ReliableServerMessage>(SERVER_MESSAGE_RELIABLE)
            .unwrap();
        builder
            .register::<ReliableClientMessage>(CLIENT_MESSAGE_RELIABLE)
            .unwrap();
        builder
            .register::<UnreliableServerMessage>(SERVER_MESSAGE_UNRELIABLE)
            .unwrap();
        builder
            .register::<UnreliableClientMessage>(CLIENT_MESSAGE_UNRELIABLE)
            .unwrap();
    });

    let ip_address = bevy_networking_turbulence::find_my_ip_address()
        .expect("main.rs launch_server() Error cannot find IP address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

    net.listen(socket_address, None, None);

    info!("Listening to connections.");
}

pub fn messages_outgoing(
    tuple0: (
        ResMut<NetworkResource>,
        EventWriter<InputUIInput>,
        EventWriter<InputSceneReady>,
        EventWriter<InputUIInputTransmitText>,
        EventWriter<InputMovementInput>,
        EventWriter<InputBuildGraphics>,
        EventWriter<InputChatMessage>,
        EventWriter<InputSprinting>,
        EventWriter<InputExamineEntity>,
        EventWriter<InputExamineMap>,
        EventWriter<InputUseWorldItem>,
        EventWriter<InputDropCurrentItem>,
        EventWriter<InputSwitchHands>,
        EventWriter<InputWearItem>,
        EventWriter<InputTakeOffItem>,
    ),

    tuple1: (
        EventWriter<InputConsoleCommand>,
        EventWriter<InputToggleCombatMode>,
        EventWriter<InputMouseDirectionUpdate>,
        EventWriter<InputMouseAction>,
        EventWriter<InputSelectBodyPart>,
        EventWriter<InputToggleAutoMove>,
        EventWriter<InputUserName>,
        EventWriter<InputAttackEntity>,
        EventWriter<InputAltItemAttack>,
        EventWriter<InputThrowItem>,
        EventWriter<InputAttackCell>,
        EventWriter<InputTabDataEntity>,
        EventWriter<InputTabDataMap>,
        EventWriter<InputTabAction>,
        EventWriter<InputMapChangeDisplayMode>,
    ),

    tuple2: (
        EventWriter<TextTreeInputSelection>,
        EventWriter<InputMapRequestDisplayModes>,
        EventWriter<InputMap>,
    ),

    handle_to_entity: Res<HandleToEntity>,
) {
    let (
        mut net,
        mut ui_input_event,
        mut scene_ready_event,
        mut ui_input_transmit_text,
        mut movement_input_event,
        mut build_graphics_event,
        mut input_chat_message_event,
        mut input_sprinting_event,
        mut examine_entity,
        mut examine_map,
        mut use_world_item,
        mut drop_current_item,
        mut switch_hands,
        mut wear_items,
        mut take_off_item,
    ) = tuple0;

    let (
        mut console_command,
        mut input_toggle_combat_mode,
        mut mouse_direction_update,
        mut input_mouse_action,
        mut input_select_body_part,
        mut input_toggle_auto_move,
        mut input_global_name,
        mut input_attack_entity,
        mut input_alt_item_attack,
        mut input_throw_item,
        mut input_attack_cell,
        mut tab_data_entity,
        mut tab_data_map,
        mut input_tab_action,
        mut input_map_change_display_mode,
    ) = tuple1;

    let (
        mut text_tree_input_selection,
        mut input_map_request_display_modes,
        mut input_map_view_range,
    ) = tuple2;

    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(client_message) = channels.recv::<ReliableClientMessage>() {
            match client_message {
                ReliableClientMessage::Awoo => {}
                ReliableClientMessage::UIInput(node_class, action, node_name, ui_type) => {
                    ui_input_event.send(InputUIInput {
                        handle: *handle,
                        node_class: node_class,
                        action: action,
                        node_name: node_name,
                        ui_type: ui_type,
                    });
                }
                ReliableClientMessage::SceneReady(scene_type) => {
                    scene_ready_event.send(InputSceneReady {
                        handle: *handle,
                        scene_type: scene_type,
                    });
                }
                ReliableClientMessage::UIInputTransmitData(ui_type, node_path, input_text) => {
                    ui_input_transmit_text.send(InputUIInputTransmitText {
                        handle: *handle,
                        ui_type: ui_type,
                        node_path: node_path,
                        input_text: input_text,
                    });
                }
                ReliableClientMessage::MovementInput(movement_input) => {
                    movement_input_event.send(InputMovementInput {
                        handle: *handle,
                        vector: movement_input,
                    });
                }
                ReliableClientMessage::BuildGraphics => {
                    build_graphics_event.send(InputBuildGraphics { handle: *handle });
                }
                ReliableClientMessage::InputChatMessage(message) => {
                    input_chat_message_event.send(InputChatMessage {
                        handle: *handle,
                        message: message,
                    });
                }

                ReliableClientMessage::SprintInput(is_sprinting) => {
                    input_sprinting_event.send(InputSprinting {
                        handle: *handle,
                        is_sprinting: is_sprinting,
                    });
                }
                ReliableClientMessage::ExamineEntity(entity_id) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            examine_entity.send(InputExamineEntity {
                                handle: *handle,
                                examine_entity_bits: entity_id,
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineEntity sender handle.");
                        }
                    }
                }
                ReliableClientMessage::ExamineMap(
                    grid_map_type,
                    cell_id_x,
                    cell_id_y,
                    cell_id_z,
                ) => match handle_to_entity.map.get(handle) {
                    Some(player_entity) => {
                        examine_map.send(InputExamineMap {
                            handle: *handle,
                            entity: *player_entity,
                            gridmap_type: grid_map_type,
                            gridmap_cell_id: Vec3Int {
                                x: cell_id_x,
                                y: cell_id_y,
                                z: cell_id_z,
                            },
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                },
                ReliableClientMessage::UseWorldItem(entity_id) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            use_world_item.send(InputUseWorldItem {
                                handle: *handle,
                                pickuper_entity: *player_entity,
                                pickupable_entity_bits: entity_id,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to UseWorldItem sender handle.");
                        }
                    }
                }
                ReliableClientMessage::DropCurrentItem(position_option) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            drop_current_item.send(InputDropCurrentItem {
                                handle: *handle,
                                pickuper_entity: *player_entity,
                                input_position_option: position_option,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to DropCurrentItem sender handle.");
                        }
                    }
                }
                ReliableClientMessage::SwitchHands => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            switch_hands.send(InputSwitchHands {
                                handle: *handle,
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to SwitchHands sender handle.");
                        }
                    }
                }
                ReliableClientMessage::WearItem(item_id, wear_slot) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            wear_items.send(InputWearItem {
                                handle: *handle,
                                wearer_entity: *player_entity,
                                wearable_id_bits: item_id,
                                wear_slot: wear_slot,
                            });
                        }
                        None => {
                            warn!(
                                "Couldn't find player_entity belonging to WearItem sender handle."
                            );
                        }
                    }
                }
                ReliableClientMessage::TakeOffItem(slot_name) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            take_off_item.send(InputTakeOffItem {
                                handle: *handle,
                                entity: *player_entity,
                                slot_name: slot_name,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to take_off_item sender handle.");
                        }
                    }

                    //                                    |
                } // Where the souls of the players are   |
                //   while they're connected.             V
                ReliableClientMessage::HeartBeat => { /* <3 */ }
                ReliableClientMessage::ConsoleCommand(command_name, variant_arguments) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            console_command.send(InputConsoleCommand {
                                handle: *handle,
                                entity: *player_entity,
                                command_name: command_name,
                                command_arguments: variant_arguments,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to console_command sender handle.");
                        }
                    }
                }
                ReliableClientMessage::ToggleCombatModeInput => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_toggle_combat_mode.send(InputToggleCombatMode {
                                handle: *handle,
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to input_toggle_combat_mode sender handle.");
                        }
                    }
                }
                ReliableClientMessage::InputMouseAction(pressed) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_mouse_action.send(InputMouseAction {
                                handle: *handle,
                                entity: *player_entity,
                                pressed,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to input_mouse_action sender handle.");
                        }
                    }
                }
                ReliableClientMessage::SelectBodyPart(body_part) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_select_body_part.send(InputSelectBodyPart {
                                handle: *handle,
                                entity: *player_entity,
                                body_part,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to SelectBodyPart sender handle.");
                        }
                    }
                }
                ReliableClientMessage::ToggleAutoMove => match handle_to_entity.map.get(handle) {
                    Some(player_entity) => {
                        input_toggle_auto_move.send(InputToggleAutoMove {
                            handle: *handle,
                            entity: *player_entity,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to InputToggleAutoMove sender handle.");
                    }
                },
                ReliableClientMessage::UserName(input_name) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_global_name.send(InputUserName {
                                handle: *handle,
                                entity: *player_entity,
                                input_name,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to InputUserName sender handle.");
                        }
                    }
                }
                ReliableClientMessage::AttackEntity(entity_id) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_attack_entity.send(InputAttackEntity {
                                handle: *handle,
                                entity: *player_entity,
                                target_entity_bits: entity_id,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to InputAttackEntity sender handle.");
                        }
                    }
                }
                ReliableClientMessage::AltItemAttack => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_alt_item_attack.send(InputAltItemAttack {
                                handle: *handle,
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to AltItemAttack sender handle.");
                        }
                    }
                }
                ReliableClientMessage::ThrowItem(position, angle) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_throw_item.send(InputThrowItem {
                                handle: *handle,
                                entity: *player_entity,
                                position,
                                angle,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to InputThrowItem sender handle.");
                        }
                    }
                }
                ReliableClientMessage::AttackCell(cell_x, cell_y, cell_z) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_attack_cell.send(InputAttackCell {
                                handle: *handle,
                                entity: *player_entity,
                                id: Vec3Int {
                                    x: cell_x,
                                    y: cell_y,
                                    z: cell_z,
                                },
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to InputAttackCell sender handle.");
                        }
                    }
                }
                ReliableClientMessage::TabDataEntity(entity_id_bits) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            tab_data_entity.send(InputTabDataEntity {
                                handle: *handle,
                                player_entity: *player_entity,
                                examine_entity_bits: entity_id_bits,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to TabDataEntity sender handle.");
                        }
                    }
                }
                ReliableClientMessage::TabDataMap(gridmap_type, idx, idy, idz) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            tab_data_map.send(InputTabDataMap {
                                handle: *handle,
                                player_entity: *player_entity,
                                gridmap_type: gridmap_type,
                                gridmap_cell_id: Vec3Int {
                                    x: idx,
                                    y: idy,
                                    z: idz,
                                },
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                        }
                    }
                }
                ReliableClientMessage::TabPressed(
                    tab_id,
                    entity_option,
                    cell_option,
                    belonging_entity,
                ) => match handle_to_entity.map.get(handle) {
                    Some(player_entity) => {
                        input_tab_action.send(InputTabAction {
                            handle: *handle,
                            tab_id,
                            player_entity: *player_entity,
                            target_entity_option: entity_option,
                            target_cell_option: cell_option,
                            belonging_entity,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to InputTabAction sender handle.");
                    }
                },
                ReliableClientMessage::TextTreeInput(
                    belonging_entity,
                    tab_action_id,
                    menu_id,
                    input_selection,
                ) => {
                    text_tree_input_selection.send(TextTreeInputSelection {
                        handle: *handle,
                        menu_id,
                        menu_selection: input_selection,
                        belonging_entity,
                        tab_action_id,
                    });
                }
                ReliableClientMessage::MapChangeDisplayMode(display_mode) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_map_change_display_mode.send(InputMapChangeDisplayMode {
                                handle: *handle,
                                entity: *player_entity,
                                display_mode,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to MapChangeDisplayMode sender handle.");
                        }
                    }
                }
                ReliableClientMessage::MapRequestDisplayModes => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_map_request_display_modes.send(InputMapRequestDisplayModes {
                                handle: *handle,
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to input_map_request_display_modes sender handle.");
                        }
                    }
                }
                ReliableClientMessage::MapCameraPosition(position) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: *handle,
                                entity: *player_entity,
                                input: MapInput::Position(position),
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to MapCameraPosition sender handle.");
                        }
                    }
                }
            }
        }

        while let Some(client_message) = channels.recv::<UnreliableClientMessage>() {
            match client_message {
                UnreliableClientMessage::MouseDirectionUpdate(mouse_direction, time_stamp) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            mouse_direction_update.send(InputMouseDirectionUpdate {
                                handle: *handle,
                                entity: *player_entity,
                                direction: mouse_direction,
                                time_stamp,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to mouse_direction_update sender handle.");
                        }
                    }
                }
                UnreliableClientMessage::MapViewRange(range_x) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: *handle,
                                entity: *player_entity,
                                input: MapInput::Range(range_x),
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to MapViewRange sender handle.");
                        }
                    }
                }
                UnreliableClientMessage::MapOverlayMouseHoverCell(idx, idy) => {
                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: *handle,
                                entity: *player_entity,
                                input: MapInput::MouseCell(idx, idy),
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to MapMouseHoverCell sender handle.");
                        }
                    }
                }
            }
        }

        while let Some(_server_message) = channels.recv::<ReliableServerMessage>() {
            // In case we ever get this from faulty or malicious clients, free it up.
        }
        while let Some(_server_message) = channels.recv::<UnreliableServerMessage>() {
            // In case we ever get this from faulty or malicious clients, free it up.
        }
    }
}

pub fn connections(
    mut net: ResMut<NetworkResource>,
    tick_rate: Res<TickRate>,
    mut auth_id_i: ResMut<AuthidI>,
    server_id: Res<ServerId>,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut commands: Commands,
    mut reader: EventReader<NetworkEvent>,
    mut net_on_new_player_connection: EventWriter<NetOnNewPlayerConnection>,
    mut connected_players: Query<(
        &mut PersistentPlayerData,
        &mut ConnectedPlayer,
        &mut PlayerInput,
    )>,
    mut used_names: ResMut<UsedNames>,
    mut client_health_ui_cache: ResMut<ClientHealthUICache>,
    gridmap_data: Res<GridmapData>,
    map_data: Res<MapData>,
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Packet(_handle, _packet) => {}
            NetworkEvent::Connected(handle) => {
                // https://github.com/smokku/bevy_networking_turbulence/blob/master/examples/channels.rs

                match net.connections.get_mut(handle) {
                    Some(connection) => match connection.remote_address() {
                        Some(remote_address) => {
                            info!(
                                "Incoming connection on [{}] from [{}]",
                                handle, remote_address
                            );
                        }
                        None => {
                            warn!("handle_network_events.rs NetworkEvent::Connected: new connection with a strange remote_address [{}]", handle);
                        }
                    },
                    None => {
                        warn!("handle_network_events.rs NetworkEvent::Connected: got packet for non-existing connection [{}]", handle);
                    }
                }

                on_new_player_connection(
                    &mut net_on_new_player_connection,
                    handle,
                    &tick_rate,
                    &mut auth_id_i,
                    &server_id,
                    &mut handle_to_entity,
                    &mut commands,
                    &mut used_names,
                    &gridmap_data,
                    &map_data,
                );
            }

            NetworkEvent::Disconnected(handle) => {
                on_player_disconnect(
                    *handle,
                    &mut handle_to_entity,
                    &mut connected_players,
                    &mut used_names,
                    &mut client_health_ui_cache,
                );
            }
            NetworkEvent::Error(_handle, _err) => {
                //warn!("NetworkEvent error [{}] : {:?}", _handle, _err);
            }
        }
    }
}

pub fn net_send_message_event(
    tuple0: (
        ResMut<NetworkResource>,
        EventReader<NetOnBoarding>,
        EventReader<NetOnNewPlayerConnection>,
        EventReader<NetOnSetupUI>,
        EventReader<NetDoneBoarding>,
        EventReader<NetLoadEntity>,
        EventReader<NetUnloadEntity>,
        EventReader<NetSendEntityUpdates>,
        EventReader<NetSendWorldEnvironment>,
        EventReader<NetChatMessage>,
        EventReader<NetOnSpawning>,
        EventReader<NetPickupWorldItem>,
        EventReader<NetDropCurrentItem>,
        EventReader<NetSwitchHands>,
        EventReader<NetWearItem>,
        EventReader<NetTakeOffItem>,
    ),
    tuple1: (
        EventReader<NetShowcase>,
        EventReader<NetConsoleCommands>,
        EventReader<NetUserName>,
        EventReader<NetUIInputTransmitData>,
        EventReader<NetHealthUpdate>,
        EventReader<NetExamineEntity>,
        EventReader<NetProjectileFOV>,
        EventReader<NetThrowItem>,
        EventReader<NetTabData>,
        EventReader<NetSendServerTime>,
        EventReader<NetUpdatePlayerCount>,
        EventReader<NetConstructionTool>,
        EventReader<NetGridmapUpdates>,
        EventReader<NetRequestDisplayModes>,
        EventReader<NetMapDisplayAtmospherics>,
    ),
    tuple2: (
        EventReader<NetMapHoverAtmospherics>,
        EventReader<NetAtmosphericsNotices>,
    ),
    connected_players: Query<&ConnectedPlayer>,
) {
    let (
        mut net,
        mut net_on_boarding,
        mut net_on_new_player_connection,
        mut net_on_setupui,
        mut net_done_boarding,
        mut net_load_entity,
        mut net_unload_entity,
        mut net_send_entity_updates,
        mut net_send_world_environment,
        mut net_chat_message,
        mut net_on_spawning,
        mut net_pickup_world_item,
        mut net_drop_current_item,
        mut net_switch_hands,
        mut net_wear_item,
        mut net_takeoff_item,
    ) = tuple0;

    let (
        mut net_showcase,
        mut net_console_commands,
        mut net_user_name,
        mut net_ui_input_transmit_data,
        mut net_health_update,
        mut net_examine_entity,
        mut net_projectile_fov,
        mut net_throw_item,
        mut net_tab_data,
        mut net_send_server_time,
        mut net_update_player_count,
        mut net_construction_tool,
        mut net_gridmap_updates,
        mut net_request_display_modes,
        mut net_display_atmospherics,
    ) = tuple1;

    let (mut net_map_hover_atmospherics, mut net_atmospherics_notices) = tuple2;

    let mut not_connected_handles = vec![];

    for connected_player_component in connected_players.iter() {
        if connected_player_component.connected == false {
            not_connected_handles.push(connected_player_component.handle);
        }
    }

    for new_event in net_on_spawning.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unNetRequestDisplayModesable to send net_on_spawning message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_spawning message (1): {:?}", err);
            }
        };
    }

    for new_event in net_on_boarding.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_on_boarding message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_boarding message (1): {:?}", err);
            }
        };
    }

    for new_event in net_on_new_player_connection.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_on_new_player_connection message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_on_new_player_connection message (1): {:?}", err);
            }
        };
    }

    for new_event in net_on_setupui.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => {
                match msg {
                    Some(msg) => {
                        warn!("net_send_message_event.rs was unable to send net_on_setupui message: {:?}", msg);
                    }
                    None => {}
                }
            }
            Err(err) => {
                warn!(
                    "net_send_message_event.rs was unable to send net_on_setupui message (1): {:?}",
                    err
                );
            }
        };
    }

    for new_event in net_done_boarding.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_done_boarding message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_done_boarding message (1): {:?}", err);
            }
        };
    }

    for new_event in net_unload_entity.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_unload_entity message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_unload_entity message (1): {:?}", err);
            }
        };
    }

    for new_event in net_load_entity.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_load_entity message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_load_entity message (1): {:?}", err);
            }
        };
    }

    for new_event in net_send_entity_updates.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_send_entity_updates message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_send_entity_updates message (1): {:?}", err);
            }
        };
    }

    for new_event in net_send_world_environment.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_send_world_environment message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_send_world_environment message (1): {:?}", err);
            }
        };
    }

    for new_event in net_chat_message.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_chat_message message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_chat_message message (1): {:?}", err);
            }
        };
    }

    for new_event in net_pickup_world_item.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_pickup_world_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_pickup_world_item message (1): {:?}", err);
            }
        };
    }

    for new_event in net_drop_current_item.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_drop_current_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_drop_current_item message (1): {:?}", err);
            }
        };
    }

    for new_event in net_switch_hands.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_switch_hands message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_switch_hands message (1): {:?}", err);
            }
        };
    }

    for new_event in net_wear_item.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => {
                match msg {
                    Some(msg) => {
                        warn!("net_send_message_event.rs was unable to send net_wear_item message: {:?}", msg);
                    }
                    None => {}
                }
            }
            Err(err) => {
                warn!(
                    "net_send_message_event.rs was unable to send net_wear_item message (1): {:?}",
                    err
                );
            }
        };
    }

    for new_event in net_takeoff_item.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_takeoff_item message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_takeoff_item message (1): {:?}", err);
            }
        };
    }

    for new_event in net_showcase.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => {
                match msg {
                    Some(msg) => {
                        warn!("net_send_message_event.rs was unable to send net_showcase message: {:?}", msg);
                    }
                    None => {}
                }
            }
            Err(err) => {
                warn!(
                    "net_send_message_event.rs was unable to send net_showcase message (1): {:?}",
                    err
                );
            }
        };
    }

    for new_event in net_console_commands.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_console_commands message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_console_commands message (1): {:?}", err);
            }
        };
    }

    for new_event in net_user_name.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => {
                match msg {
                    Some(msg) => {
                        warn!("net_send_message_event.rs was unable to send net_user_name message: {:?}", msg);
                    }
                    None => {}
                }
            }
            Err(err) => {
                warn!(
                    "net_send_message_event.rs was unable to send net_user_name message (1): {:?}",
                    err
                );
            }
        };
    }

    for new_event in net_ui_input_transmit_data.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_ui_input_transmit_data message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_ui_input_transmit_data message (1): {:?}", err);
            }
        };
    }

    for new_event in net_health_update.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_health_update message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_health_update message (1): {:?}", err);
            }
        };
    }

    for new_event in net_examine_entity.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_examine_entity message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_examine_entity message (1): {:?}", err);
            }
        };
    }

    for new_event in net_projectile_fov.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_projectile_fov message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_projectile_fov message (1): {:?}", err);
            }
        };
    }

    for new_event in net_throw_item.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => {
                match msg {
                    Some(msg) => {
                        warn!("net_send_message_event.rs was unable to send net_throw_item message: {:?}", msg);
                    }
                    None => {}
                }
            }
            Err(err) => {
                warn!(
                    "net_send_message_event.rs was unable to send net_throw_item message (1): {:?}",
                    err
                );
            }
        };
    }

    for new_event in net_tab_data.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => {
                match msg {
                    Some(msg) => {
                        warn!("net_send_message_event.rs was unable to send net_tab_data message: {:?}", msg);
                    }
                    None => {}
                }
            }
            Err(err) => {
                warn!(
                    "net_send_message_event.rs was unable to send net_tab_data message (1): {:?}",
                    err
                );
            }
        };
    }

    for new_event in net_send_server_time.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_send_server_time message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_send_server_time message (1): {:?}", err);
            }
        };
    }

    for new_event in net_update_player_count.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_update_player_count message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_update_player_count message (1): {:?}", err);
            }
        };
    }

    for new_event in net_construction_tool.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_construction_tool message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_construction_tool message (1): {:?}", err);
            }
        };
    }

    for new_event in net_gridmap_updates.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_gridmap_updates message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_gridmap_updates message (1): {:?}", err);
            }
        };
    }

    for new_event in net_request_display_modes.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_request_display_modes message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_request_display_modes message (1): {:?}", err);
            }
        };
    }

    for new_event in net_display_atmospherics.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match &new_event.message {
            crate::space::core::networking::resources::NetMessageType::Reliable(m) => {
                match net.send_message(new_event.handle, m.clone()) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("net_send_message_event.rs was unable to send net_display_atmospherics message: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("net_send_message_event.rs was unable to send net_display_atmospherics message (1): {:?}", err);
                    }
                };
            }
            crate::space::core::networking::resources::NetMessageType::Unreliable(m) => {
                match net.send_message(new_event.handle, m.clone()) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("net_send_message_event.rs was unable to send net_display_atmospherics message: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("net_send_message_event.rs was unable to send net_display_atmospherics message (1): {:?}", err);
                    }
                };
            }
        }
    }

    for new_event in net_map_hover_atmospherics.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match &new_event.message {
            crate::space::core::networking::resources::NetMessageType::Reliable(m) => {
                match net.send_message(new_event.handle, m.clone()) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("net_send_message_event.rs was unable to send net_map_hover_atmospherics message: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("net_send_message_event.rs was unable to send net_map_hover_atmospherics message (1): {:?}", err);
                    }
                };
            }
            crate::space::core::networking::resources::NetMessageType::Unreliable(m) => {
                match net.send_message(new_event.handle, m.clone()) {
                    Ok(msg) => match msg {
                        Some(msg) => {
                            warn!("net_send_message_event.rs was unable to send net_map_hover_atmospherics message: {:?}", msg);
                        }
                        None => {}
                    },
                    Err(err) => {
                        warn!("net_send_message_event.rs was unable to send net_map_hover_atmospherics message (1): {:?}", err);
                    }
                };
            }
        }
    }

    for new_event in net_atmospherics_notices.iter() {
        if not_connected_handles.contains(&new_event.handle) {
            continue;
        }

        match net.send_message(new_event.handle, new_event.message.clone()) {
            Ok(msg) => match msg {
                Some(msg) => {
                    warn!("net_send_message_event.rs was unable to send net_atmospherics_notices message: {:?}", msg);
                }
                None => {}
            },
            Err(err) => {
                warn!("net_send_message_event.rs was unable to send net_atmospherics_notices message (1): {:?}", err);
            }
        };
    }
}
