use bevy::prelude::warn;
use bevy::prelude::Res;
use bevy::prelude::{EventWriter, ResMut};
use bevy_renet::renet::RenetServer;
use networking::{plugin::RENET_RELIABLE_CHANNEL_ID, server::ReliableClientMessage};
use networking::{plugin::RENET_UNRELIABLE_CHANNEL_ID, server::UnreliableClientMessage};
use resources::core::HandleToEntity;

use crate::input::InputAltItemAttack;
use crate::input::InputAttackCell;
use crate::input::InputAttackEntity;
use crate::input::InputMouseAction;
use crate::input::InputMovementInput;
use crate::input::InputSceneReady;
use crate::input::InputSelectBodyPart;
use crate::input::InputSprinting;
use crate::input::InputToggleAutoMove;
use crate::input::InputToggleCombatMode;
use crate::input::InputUIInput;
use crate::input::InputUIInputTransmitText;
use crate::input::{InputBuildGraphics, InputMouseDirectionUpdate};
use math::grid::Vec3Int;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    mut input_ui_input: EventWriter<InputUIInput>,
    mut scene_ready_event: EventWriter<InputSceneReady>,
    mut ui_input_transmit_text: EventWriter<InputUIInputTransmitText>,
    mut movement_input_event: EventWriter<InputMovementInput>,
    handle_to_entity: Res<HandleToEntity>,
    mut build_graphics_event: EventWriter<InputBuildGraphics>,
    mut input_sprinting_event: EventWriter<InputSprinting>,
    mut input_toggle_combat_mode: EventWriter<InputToggleCombatMode>,
    mut input_mouse_action: EventWriter<InputMouseAction>,
    mut mouse_direction_update: EventWriter<InputMouseDirectionUpdate>,
    mut input_select_body_part: EventWriter<InputSelectBodyPart>,
    mut input_toggle_auto_move: EventWriter<InputToggleAutoMove>,
    mut input_attack_entity: EventWriter<InputAttackEntity>,
    mut input_alt_item_attack: EventWriter<InputAltItemAttack>,
    mut input_attack_cell: EventWriter<InputAttackCell>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
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
                ReliableClientMessage::UIInput(node_class, action, node_name, ui_type) => {
                    input_ui_input.send(InputUIInput {
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
                        scene_id: scene_type,
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
                _ => (),
            }
        }

        while let Some(message) = server.receive_message(handle, RENET_UNRELIABLE_CHANNEL_ID) {
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
                _ => (),
            }
        }
    }
}
