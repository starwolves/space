use bevy::{ecs::system::{ResMut}, prelude::{EventWriter, Res, warn}};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{events::general::{build_graphics::InputBuildGraphics, console_command::InputConsoleCommand, drop_current_item::InputDropCurrentItem, examine_entity::InputExamineEntity, examine_map::InputExamineMap, input_alt_item_attack::InputAltItemAttack, input_attack_cell::InputAttackCell, input_attack_entity::InputAttackEntity, input_chat_message::InputChatMessage, input_mouse_action::InputMouseAction, input_select_body_part::InputSelectBodyPart, input_sprinting::InputSprinting, input_tab_action::InputTabAction, input_throw_item::InputThrowItem, input_toggle_auto_move::InputToggleAutoMove, input_toggle_combat_mode::InputToggleCombatMode, input_user_name::InputUserName, mouse_direction_update::InputMouseDirectionUpdate, movement_input::InputMovementInput, scene_ready::InputSceneReady, switch_hands::InputSwitchHands, tab_data_entity::InputTabDataEntity, tab_data_map::InputTabDataMap, take_off_item::InputTakeOffItem, ui_input::InputUIInput, ui_input_transmit_text::InputUIInputTransmitText, use_world_item::InputUseWorldItem, wear_item::InputWearItem}, resources::{doryen_fov::Vec3Int, handle_to_entity::HandleToEntity, network_messages::{ReliableClientMessage, ReliableServerMessage, UnreliableClientMessage, UnreliableServerMessage}}};

pub fn handle_network_messages(

    tuple0 : (
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

    tuple1 : (
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
    ),

    handle_to_entity : Res<HandleToEntity>,

) {

    let (
        mut net,
        mut ui_input_event,
        mut scene_ready_event,
        mut ui_input_transmit_text,
        mut movement_input_event,
        mut build_graphics_event ,
        mut input_chat_message_event,
        mut input_sprinting_event,
        mut examine_entity,
        mut examine_map,
        mut use_world_item,
        mut drop_current_item,
        mut switch_hands,
        mut wear_items,
        mut take_off_item,
    )
    = tuple0;

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
    )
    = tuple1;


    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        

        while let Some(client_message) = channels.recv::<ReliableClientMessage>() {

            match client_message {
                ReliableClientMessage::Awoo => {},
                ReliableClientMessage::UIInput(
                    node_class,
                    action,
                    node_name,
                    ui_type
                ) => {
                    ui_input_event.send(InputUIInput{
                        handle : *handle,
                        node_class: node_class,
                        action: action,
                        node_name : node_name,
                        ui_type : ui_type
                    });
                }
                ReliableClientMessage::SceneReady(scene_type) => {
                    scene_ready_event.send(InputSceneReady{
                        handle: *handle,
                        scene_type: scene_type
                    });
                }
                ReliableClientMessage::UIInputTransmitData(ui_type, node_path, input_text) => {
                    ui_input_transmit_text.send(InputUIInputTransmitText{
                        handle: *handle,
                        ui_type:ui_type,
                        node_path:node_path,
                        input_text:input_text
                    });
                }
                ReliableClientMessage::MovementInput(movement_input) => {
                    movement_input_event.send(InputMovementInput{
                        handle: *handle,
                        vector: movement_input
                    });
                }
                ReliableClientMessage::BuildGraphics => {
                    build_graphics_event.send(InputBuildGraphics{
                        handle: *handle
                    });
                }
                ReliableClientMessage::InputChatMessage(message) => {
                    input_chat_message_event.send(InputChatMessage{
                        handle: *handle,
                        message: message
                    });
                },
                
                ReliableClientMessage::SprintInput(is_sprinting) => {

                    input_sprinting_event.send(InputSprinting {
                        handle: *handle,
                        is_sprinting: is_sprinting,
                    });

                },
                ReliableClientMessage::ExamineEntity(entity_id) => {

                    

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            examine_entity.send(InputExamineEntity{
                                handle: *handle,
                                examine_entity_bits: entity_id,
                                entity : *player_entity,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineEntity sender handle.");
                        },
                    }

                },
                ReliableClientMessage::ExamineMap(grid_map_type, cell_id_x,cell_id_y,cell_id_z) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            examine_map.send(InputExamineMap{
                                handle: *handle,
                                entity: *player_entity,
                                gridmap_type: grid_map_type,
                                gridmap_cell_id: Vec3Int {
                                    x: cell_id_x,
                                    y: cell_id_y,
                                    z: cell_id_z,
                                },
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                        },
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
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to UseWorldItem sender handle.");
                        },
                    }

                    

                },
                ReliableClientMessage::DropCurrentItem(position_option) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            drop_current_item.send(InputDropCurrentItem {
                                handle: *handle,
                                pickuper_entity : *player_entity,
                                input_position_option: position_option,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to DropCurrentItem sender handle.");
                        },
                    }

                    
                        
                },
                ReliableClientMessage::SwitchHands => {


                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            switch_hands.send(InputSwitchHands {
                                handle: *handle,
                                entity : *player_entity
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to SwitchHands sender handle.");
                        },
                    }

                },
                ReliableClientMessage::WearItem(item_id, wear_slot) => {


                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            wear_items.send(InputWearItem {
                                handle: *handle,
                                wearer_entity: *player_entity,
                                wearable_id_bits: item_id,
                                wear_slot: wear_slot,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to WearItem sender handle.");
                        },
                    }

                },
                ReliableClientMessage::TakeOffItem(slot_name) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            take_off_item.send(InputTakeOffItem {
                                handle: *handle,
                                entity: *player_entity,
                                slot_name: slot_name,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to take_off_item sender handle.");
                        },
                    }
                    
                   //                                    |
                },// Where the souls of the players are  |
                //   while they're connected.            V
                ReliableClientMessage::HeartBeat => {/* <3 */},
                ReliableClientMessage::ConsoleCommand(command_name, variant_arguments) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            console_command.send(InputConsoleCommand {
                                handle: *handle,
                                entity: *player_entity,
                                command_name: command_name,
                                command_arguments: variant_arguments,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to console_command sender handle.");
                        },
                    }


                },
                ReliableClientMessage::ToggleCombatModeInput => {

                    

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_toggle_combat_mode.send(InputToggleCombatMode {
                                handle: *handle,
                                entity: *player_entity,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to input_toggle_combat_mode sender handle.");
                        },
                    }


                },
                ReliableClientMessage::InputMouseAction(pressed) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_mouse_action.send(InputMouseAction {
                                handle: *handle,
                                entity: *player_entity,
                                pressed
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to input_mouse_action sender handle.");
                        },
                    }

                },
                ReliableClientMessage::SelectBodyPart(body_part) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_select_body_part.send(InputSelectBodyPart {
                                handle: *handle,
                                entity: *player_entity,
                                body_part,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to SelectBodyPart sender handle.");
                        },
                    }

                },
                ReliableClientMessage::ToggleAutoMove => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_toggle_auto_move.send(InputToggleAutoMove {
                                handle: *handle,
                                entity: *player_entity,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to InputToggleAutoMove sender handle.");
                        },
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
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to InputUserName sender handle.");
                        },
                    }

                },
                ReliableClientMessage::AttackEntity(entity_id) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_attack_entity.send(InputAttackEntity {
                                handle: *handle,
                                entity: *player_entity,
                                target_entity_bits : entity_id,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to InputAttackEntity sender handle.");
                        },
                    }

                },
                ReliableClientMessage::AltItemAttack => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_alt_item_attack.send(InputAltItemAttack {
                                handle: *handle,
                                entity: *player_entity,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to AltItemAttack sender handle.");
                        },
                    }

                },
                ReliableClientMessage::ThrowItem(position, angle) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_throw_item.send(InputThrowItem {
                                handle: *handle,
                                entity: *player_entity,
                                position,
                                angle
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to InputThrowItem sender handle.");
                        },
                    }

                },
                ReliableClientMessage::AttackCell(cell_x, cell_y, cell_z) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_attack_cell.send(InputAttackCell {
                                handle: *handle,
                                entity: *player_entity,
                                id: Vec3Int{x:cell_x,y:cell_y,z:cell_z}
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to InputAttackCell sender handle.");
                        },
                    }

                },
                ReliableClientMessage::TabDataEntity(entity_id_bits) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            tab_data_entity.send(InputTabDataEntity{
                                handle: *handle,
                                player_entity: *player_entity,
                                examine_entity_bits: entity_id_bits,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to TabDataEntity sender handle.");
                        },
                    }

                },
                ReliableClientMessage::TabDataMap(gridmap_type, idx, idy, idz) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            tab_data_map.send(InputTabDataMap{
                                handle: *handle,
                                player_entity: *player_entity,
                                gridmap_type: gridmap_type,
                                gridmap_cell_id: Vec3Int {
                                    x: idx,
                                    y: idy,
                                    z: idz,
                                },
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                        },
                    }

                },
                ReliableClientMessage::TabPressed(tab_id, entity_option, cell_option, belonging_entity) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            input_tab_action.send(InputTabAction {
                                handle: *handle,
                                tab_id,
                                player_entity: *player_entity,
                                target_entity_option: entity_option,
                                target_cell_option: cell_option,
                                belonging_entity 
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to InputTabAction sender handle.");
                        },
                    }

                
                },
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
                                time_stamp
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to mouse_direction_update sender handle.");
                        },
                    }

                },
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
