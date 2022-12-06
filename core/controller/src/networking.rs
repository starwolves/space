use bevy::prelude::warn;
use bevy::prelude::Res;
use bevy::prelude::Vec2;
use bevy::prelude::{EventWriter, ResMut};
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::plugin::RENET_UNRELIABLE_CHANNEL_ID;
use networking::server::UIInputAction;
use serde::Deserialize;
use serde::Serialize;

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
use crate::input::{InputBuildGraphics, InputMouseDirectionUpdate};
use math::grid::Vec3Int;

use networking::server::HandleToEntity;
use player::boarding::InputUIInputTransmitText;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ControllerClientMessage {
    UIInput(UIInputNodeClass, UIInputAction, String, String),
    SceneReady(String),
    UIInputTransmitData(String, String, String),
    MovementInput(Vec2),
    SprintInput(bool),
    BuildGraphics,
    ToggleCombatModeInput,
    InputMouseAction(bool),
    SelectBodyPart(String),
    ToggleAutoMove,
    AttackEntity(u64),
    AltItemAttack,
    AttackCell(i16, i16, i16),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UIInputNodeClass {
    Button,
}

/// Event as client input , interaction with UI.
#[cfg(feature = "server")]
pub struct InputUIInput {
    /// Handle of the connection that input this.
    pub handle: u64,
    /// The Godot node class of the input element.
    pub node_class: UIInputNodeClass,
    /// The action ID.
    pub action: UIInputAction,
    /// The Godot node name of the input element.
    pub node_name: String,
    /// The UI this input was submitted from.
    pub ui_type: String,
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ControllerUnreliableMessage {
    MouseDirectionUpdate(f32, u64),
}

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
            let client_message_result: Result<ControllerClientMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    continue;
                }
            }

            match client_message {
                ControllerClientMessage::UIInput(node_class, action, node_name, ui_type) => {
                    input_ui_input.send(InputUIInput {
                        handle: handle,
                        node_class: node_class,
                        action: action,
                        node_name: node_name,
                        ui_type: ui_type,
                    });
                }
                ControllerClientMessage::SceneReady(scene_type) => {
                    scene_ready_event.send(InputSceneReady {
                        handle: handle,
                        scene_id: scene_type,
                    });
                }
                ControllerClientMessage::UIInputTransmitData(ui_type, node_path, input_text) => {
                    ui_input_transmit_text.send(InputUIInputTransmitText {
                        handle: handle,
                        ui_type: ui_type,
                        node_path: node_path,
                        input_text: input_text,
                    });
                }

                ControllerClientMessage::MovementInput(movement_input) => {
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

                ControllerClientMessage::BuildGraphics => {
                    build_graphics_event.send(InputBuildGraphics { handle: handle });
                }

                ControllerClientMessage::SprintInput(is_sprinting) => {
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

                ControllerClientMessage::ToggleCombatModeInput => {
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

                ControllerClientMessage::InputMouseAction(pressed) => {
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

                ControllerClientMessage::SelectBodyPart(body_part) => {
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
                ControllerClientMessage::ToggleAutoMove => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_toggle_auto_move.send(InputToggleAutoMove {
                                entity: *player_entity,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to InputToggleAutoMove sender handle.");
                        }
                    }
                }
                ControllerClientMessage::AttackEntity(entity_id) => {
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

                ControllerClientMessage::AltItemAttack => {
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

                ControllerClientMessage::AttackCell(cell_x, cell_y, cell_z) => {
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
            }
        }

        while let Some(message) = server.receive_message(handle, RENET_UNRELIABLE_CHANNEL_ID) {
            let client_message: ControllerUnreliableMessage =
                bincode::deserialize(&message).unwrap();

            match client_message {
                ControllerUnreliableMessage::MouseDirectionUpdate(mouse_direction, time_stamp) => {
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
            }
        }
    }
}
