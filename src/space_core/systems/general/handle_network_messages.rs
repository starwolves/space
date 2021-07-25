use bevy::{ecs::system::{ResMut}, prelude::{EventWriter, Res, warn}};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{events::general::{build_graphics::BuildGraphics, console_command::ConsoleCommand, drop_current_item::DropCurrentItem, examine_entity::ExamineEntity, examine_map::ExamineMap, input_chat_message::InputChatMessage, input_sprinting::InputSprinting, movement_input::MovementInput, scene_ready::SceneReady, switch_hands::SwitchHands, take_off_item::TakeOffItem, ui_input::UIInput, ui_input_transmit_text::UIInputTransmitText, use_world_item::UseWorldItem, wear_item::WearItem}, resources::{handle_to_entity::HandleToEntity, precalculated_fov_data::Vec3Int}, structs::network_messages::{ReliableClientMessage, ReliableServerMessage, UnreliableServerMessage}};

pub fn handle_network_messages(

    tuple0 : (
        ResMut<NetworkResource>,
        EventWriter<UIInput>,
        EventWriter<SceneReady>,
        EventWriter<UIInputTransmitText>,
        EventWriter<MovementInput>,
        EventWriter<BuildGraphics>,
        EventWriter<InputChatMessage>,
        EventWriter<InputSprinting>,
        EventWriter<ExamineEntity>,
        EventWriter<ExamineMap>,
        EventWriter<UseWorldItem>,
        EventWriter<DropCurrentItem>,
        EventWriter<SwitchHands>,
        EventWriter<WearItem>,
        EventWriter<TakeOffItem>,
    ),

    tuple1 : (
        EventWriter<ConsoleCommand>,
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

    let 
        mut console_command
    
    = tuple1.0;


    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        

        while let Some(client_message) = channels.recv::<ReliableClientMessage>() {
            //info!("ReliableClientMessage received on [{}]: {:?}",handle, client_message);

            match client_message {
                ReliableClientMessage::Awoo => {},
                ReliableClientMessage::UIInput(
                    node_class,
                    action,
                    node_name,
                    ui_type
                ) => {
                    ui_input_event.send(UIInput{
                        handle : *handle,
                        node_class: node_class,
                        action: action,
                        node_name : node_name,
                        ui_type : ui_type
                    });
                }
                ReliableClientMessage::SceneReady(scene_type) => {
                    scene_ready_event.send(SceneReady{
                        handle: *handle,
                        scene_type: scene_type
                    });
                }
                ReliableClientMessage::UIInputTransmitData(ui_type, node_path, input_text) => {
                    ui_input_transmit_text.send(UIInputTransmitText{
                        handle: *handle,
                        ui_type:ui_type,
                        node_path:node_path,
                        input_text:input_text
                    });
                }
                ReliableClientMessage::MovementInput(movement_input) => {
                    movement_input_event.send(MovementInput{
                        handle: *handle,
                        vector: movement_input
                    });
                }
                ReliableClientMessage::BuildGraphics => {
                    build_graphics_event.send(BuildGraphics{
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

                    examine_entity.send(ExamineEntity{
                        handle: *handle,
                        examine_entity_bits: entity_id,
                    });

                },
                ReliableClientMessage::ExamineMap(grid_map_type, cell_id_x,cell_id_y,cell_id_z) => {

                    examine_map.send(ExamineMap{
                        handle: *handle,
                        gridmap_type: grid_map_type,
                        gridmap_cell_id: Vec3Int {
                            x: cell_id_x,
                            y: cell_id_y,
                            z: cell_id_z,
                        },
                    });

                },
                ReliableClientMessage::UseWorldItem(entity_id) => {


                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            use_world_item.send(UseWorldItem {
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
                ReliableClientMessage::DropCurrentItem => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            drop_current_item.send(DropCurrentItem {
                                handle: *handle,
                                pickuper_entity : *player_entity
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
                            switch_hands.send(SwitchHands {
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
                            wear_items.send(WearItem {
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
                            take_off_item.send(TakeOffItem {
                                handle: *handle,
                                entity: *player_entity,
                                slot_name: slot_name,
                            });
                        },
                        None => {
                            warn!("Couldn't find player_entity belonging to take_off_item sender handle.");
                        },
                    }
                    

                },
                ReliableClientMessage::HeartBeat => {},
                ReliableClientMessage::ConsoleCommand(command_name, variant_arguments) => {

                    match handle_to_entity.map.get(handle) {
                        Some(player_entity) => {
                            console_command.send(ConsoleCommand {
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
