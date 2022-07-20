use bevy::{
    math::{Vec2, Vec3},
    prelude::{info, warn, Entity, EventReader, EventWriter, Res, ResMut},
};
use api::{
    console_commands::ConsoleCommandVariantValues,
    data::{HandleToEntity, Vec3Int},
    examinable::InputExamineEntity,
    gridmap::{ExamineMapMessage, GridMapType, GridmapExamineMessages},
    network::{
        InputChatMessage, InputUIInput, InputUIInputTransmitText, PendingMessage,
        PendingNetworkMessage, ReliableClientMessage, ReliableServerMessage,
        UnreliableClientMessage,
    },
};

pub struct NetPlayerConn {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetPlayerConn {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

pub const SERVER_PORT: u16 = 57713;

use std::{net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{RenetConnectionConfig, RenetServer, ServerConfig, NETCODE_KEY_BYTES};

use super::plugin::{RENET_RELIABLE_CHANNEL_ID, RENET_UNRELIABLE_CHANNEL_ID};

const PROTOCOL_ID: u64 = 7;

pub fn startup_listen_connections(encryption_key: [u8; NETCODE_KEY_BYTES]) -> RenetServer {
    let server_addr = (local_ipaddress::get().unwrap_or_default() + ":" + &SERVER_PORT.to_string())
        .parse()
        .unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, encryption_key);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let renet_server =
        RenetServer::new(current_time, server_config, connection_config, socket).unwrap();

    info!("Listening to connections on [{}].", server_addr);

    renet_server
}

pub struct InputAttackCell {
    pub entity: Entity,
    pub id: Vec3Int,
}

#[derive(Debug)]
pub struct InputTabDataMap {
    pub player_entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
}

pub struct InputMapChangeDisplayMode {
    pub handle: u64,
    pub entity: Entity,
    pub display_mode: String,
}

pub enum MapInput {
    Range(f32),
    Position(Vec2),
    MouseCell(i16, i16),
}

pub struct InputMap {
    pub handle: u64,
    pub entity: Entity,
    pub input: MapInput,
}
pub struct InputMapRequestDisplayModes {
    pub handle: u64,
    pub entity: Entity,
}

#[derive(Default)]
pub struct ExamineEntityMessages {
    pub messages: Vec<InputExamineEntity>,
}

pub fn incoming_messages(
    tuple0: (
        ResMut<RenetServer>,
        EventWriter<InputUIInput>,
        EventWriter<InputSceneReady>,
        EventWriter<InputUIInputTransmitText>,
        EventWriter<InputMovementInput>,
        EventWriter<InputBuildGraphics>,
        EventWriter<InputChatMessage>,
        EventWriter<InputSprinting>,
        ResMut<ExamineEntityMessages>,
        ResMut<GridmapExamineMessages>,
        EventWriter<InputUseWorldItem>,
        EventWriter<InputDropCurrentItem>,
        EventWriter<InputSwitchHands>,
        EventWriter<InputWearItem>,
        EventWriter<InputTakeOffItem>,
    ),

    tuple1: (
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

    mut console_commands_queue: EventWriter<InputConsoleCommand>,

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

    for handle in net.clients_id().into_iter() {
        while let Some(message) = net.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<ReliableClientMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    warn!("Received invalid client message.");
                    continue;
                }
            }

            match client_message {
                ReliableClientMessage::Awoo => {}
                ReliableClientMessage::UIInput(node_class, action, node_name, ui_type) => {
                    ui_input_event.send(InputUIInput {
                        handle: handle,
                        node_class: node_class,
                        action: action,
                        node_name: node_name,
                        ui_type: ui_type,
                    });
                }
                ReliableClientMessage::SceneReady(scene_type) => {
                    scene_ready_event.send(InputSceneReady {
                        handle: handle,
                        scene_type: scene_type,
                    });
                }
                ReliableClientMessage::UIInputTransmitData(ui_type, node_path, input_text) => {
                    ui_input_transmit_text.send(InputUIInputTransmitText {
                        handle: handle,
                        ui_type: ui_type,
                        node_path: node_path,
                        input_text: input_text,
                    });
                }
                ReliableClientMessage::MovementInput(movement_input) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            movement_input_event.send(InputMovementInput {
                                vector: movement_input,
                                player_entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                        }
                    }
                }
                ReliableClientMessage::BuildGraphics => {
                    build_graphics_event.send(InputBuildGraphics { handle: handle });
                }
                ReliableClientMessage::InputChatMessage(message) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_chat_message_event.send(InputChatMessage {
                                entity: *player_entity,
                                message: message,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to SelectBodyPart sender handle.");
                        }
                    }
                }

                ReliableClientMessage::SprintInput(is_sprinting) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_sprinting_event.send(InputSprinting {
                                is_sprinting: is_sprinting,
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to SelectBodyPart sender handle.");
                        }
                    }
                }
                ReliableClientMessage::ExamineEntity(entity_id) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            examine_entity.messages.push(InputExamineEntity {
                                handle: handle,
                                examine_entity_bits: entity_id,
                                entity: *player_entity,
                                ..Default::default()
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
                ) => match handle_to_entity.map.get(&handle) {
                    Some(player_entity) => {
                        examine_map.messages.push(ExamineMapMessage {
                            handle: handle,
                            entity: *player_entity,
                            gridmap_type: grid_map_type,
                            gridmap_cell_id: Vec3Int {
                                x: cell_id_x,
                                y: cell_id_y,
                                z: cell_id_z,
                            },
                            ..Default::default()
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                },
                ReliableClientMessage::UseWorldItem(entity_id) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            use_world_item.send(InputUseWorldItem {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            drop_current_item.send(InputDropCurrentItem {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            switch_hands.send(InputSwitchHands {
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to SwitchHands sender handle.");
                        }
                    }
                }
                ReliableClientMessage::WearItem(item_id, wear_slot) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            wear_items.send(InputWearItem {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            take_off_item.send(InputTakeOffItem {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            console_commands_queue.send(InputConsoleCommand {
                                handle_option: Some(handle),
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_toggle_combat_mode.send(InputToggleCombatMode {
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to input_toggle_combat_mode sender handle.");
                        }
                    }
                }
                ReliableClientMessage::InputMouseAction(pressed) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_mouse_action.send(InputMouseAction {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_select_body_part.send(InputSelectBodyPart {
                                entity: *player_entity,
                                body_part,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to SelectBodyPart sender handle.");
                        }
                    }
                }
                ReliableClientMessage::ToggleAutoMove => match handle_to_entity.map.get(&handle) {
                    Some(player_entity) => {
                        input_toggle_auto_move.send(InputToggleAutoMove {
                            entity: *player_entity,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to InputToggleAutoMove sender handle.");
                    }
                },
                ReliableClientMessage::UserName(input_name) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_global_name.send(InputUserName {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_attack_entity.send(InputAttackEntity {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_alt_item_attack.send(InputAltItemAttack {
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to AltItemAttack sender handle.");
                        }
                    }
                }
                ReliableClientMessage::ThrowItem(position, angle) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_throw_item.send(InputThrowItem {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_attack_cell.send(InputAttackCell {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            tab_data_entity.send(InputTabDataEntity {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            tab_data_map.send(InputTabDataMap {
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
                ) => match handle_to_entity.map.get(&handle) {
                    Some(player_entity) => {
                        input_tab_action.send(InputTabAction {
                            tab_id,
                            action_performing_entity: *player_entity,
                            target_entity_option: entity_option,
                            target_cell_option: cell_option,
                            belonging_entity_option: belonging_entity,
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
                        handle: handle,
                        menu_id,
                        menu_selection: input_selection,
                        belonging_entity,
                        tab_action_id,
                    });
                }
                ReliableClientMessage::MapChangeDisplayMode(display_mode) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_map_change_display_mode.send(InputMapChangeDisplayMode {
                                handle: handle,
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_map_request_display_modes.send(InputMapRequestDisplayModes {
                                handle: handle,
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to input_map_request_display_modes sender handle.");
                        }
                    }
                }
                ReliableClientMessage::MapCameraPosition(position) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: handle,
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

        while let Some(message) = net.receive_message(handle, RENET_UNRELIABLE_CHANNEL_ID) {
            let client_message: UnreliableClientMessage = bincode::deserialize(&message).unwrap();

            match client_message {
                UnreliableClientMessage::MouseDirectionUpdate(mouse_direction, time_stamp) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            mouse_direction_update.send(InputMouseDirectionUpdate {
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: handle,
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
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: handle,
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
    }
}

pub fn net_system<T: std::marker::Send + std::marker::Sync + PendingMessage + 'static>(
    mut net1: EventReader<T>,
    mut pending_net: EventWriter<PendingNetworkMessage>,
) {
    for new_event in net1.iter() {
        let message = new_event.get_message();

        pending_net.send(PendingNetworkMessage {
            handle: message.handle,
            message: message.message,
        });
    }
}
pub struct InputConsoleCommand {
    pub handle_option: Option<u64>,
    pub entity: Entity,
    pub command_name: String,
    pub command_arguments: Vec<ConsoleCommandVariantValues>,
}

pub struct InputToggleCombatMode {
    pub entity: Entity,
}

pub struct InputDropCurrentItem {
    pub pickuper_entity: Entity,
    pub input_position_option: Option<Vec3>,
}

pub struct InputThrowItem {
    pub entity: Entity,
    pub position: Vec3,
    pub angle: f32,
}

pub struct InputSwitchHands {
    pub entity: Entity,
}

pub struct InputTakeOffItem {
    pub entity: Entity,
    pub slot_name: String,
}

pub struct InputUseWorldItem {
    pub pickuper_entity: Entity,
    pub pickupable_entity_bits: u64,
}

pub struct InputWearItem {
    pub wearer_entity: Entity,
    pub wearable_id_bits: u64,
    pub wear_slot: String,
}
pub struct InputUserName {
    pub entity: Entity,
    pub input_name: String,
}
pub struct InputTabDataEntity {
    pub player_entity: Entity,
    pub examine_entity_bits: u64,
}

pub struct InputTabAction {
    pub tab_id: String,
    pub action_performing_entity: Entity,
    pub target_entity_option: Option<u64>,
    pub belonging_entity_option: Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
}

pub struct InputToggleAutoMove {
    pub entity: Entity,
}

pub struct InputAttackEntity {
    pub entity: Entity,
    pub target_entity_bits: u64,
}

pub struct InputAltItemAttack {
    pub entity: Entity,
}

pub struct InputMouseAction {
    pub entity: Entity,
    pub pressed: bool,
}
pub struct InputSelectBodyPart {
    pub entity: Entity,
    pub body_part: String,
}
pub struct InputMovementInput {
    pub player_entity: Entity,
    pub vector: Vec2,
}

pub struct TextTreeInputSelection {
    pub handle: u64,
    pub menu_id: String,
    pub menu_selection: String,
    pub tab_action_id: String,
    pub belonging_entity: Option<u64>,
}

pub struct InputSprinting {
    pub entity: Entity,
    pub is_sprinting: bool,
}
pub struct InputSceneReady {
    pub handle: u64,
    pub scene_type: String,
}
pub struct InputBuildGraphics {
    pub handle: u64,
}

pub struct InputMouseDirectionUpdate {
    pub entity: Entity,
    pub direction: f32,
    pub time_stamp: u64,
}
pub struct InputConstructionOptionsSelection {
    pub handle_option: Option<u64>,
    pub menu_selection: String,
    // Entity has been validated.
    pub entity: Entity,
}
