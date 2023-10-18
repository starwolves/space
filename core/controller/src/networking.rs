use bevy::prelude::warn;
use bevy::prelude::Event;
use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use networking::server::ConnectedPlayer;
use networking::server::IncomingEarlyReliableClientMessage;
use networking::server::IncomingEarlyUnreliableClientMessage;
use networking::server::OutgoingReliableServerMessage;
use networking::server::OutgoingUnreliableServerMessage;
use networking::server::UIInputAction;
use pawn::net::MouseMessage;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::input::InputAltItemAttack;
use crate::input::InputAttackCell;
use crate::input::InputAttackEntity;
use crate::input::InputMouseAction;
use crate::input::InputMovementInput;
use crate::input::InputSprinting;
use crate::input::InputToggleAutoMove;
use crate::input::InputToggleCombatMode;
use crate::input::{InputBuildGraphics, InputMouseDirectionUpdate};
use resources::math::Vec3Int;

use networking::server::HandleToEntity;
use player::boarding::InputUIInputTransmitText;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum UIInputNodeClass {
    Button,
}

/// Event as client input , interaction with UI.
#[derive(Event)]
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

use crate::net::{ControllerClientMessage, ControllerUnreliableClientMessage};
use bevy::prelude::EventReader;
use networking::server::{IncomingReliableClientMessage, IncomingUnreliableClientMessage};

/// Replicates client input to peers this is a server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub struct PeerReliableControllerMessage {
    pub message: ControllerClientMessage,
    pub peer_handle: u64,
    pub client_stamp: u8,
}
/// Replicates client input to peers this is a server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub struct PeerMouseMessage {
    pub message: MouseMessage,
    pub peer_handle: u64,
    pub client_stamp: u8,
}

/// Replicate client input to peers instantly.
pub(crate) fn peer_replication(
    mut server: EventReader<IncomingEarlyReliableClientMessage<ControllerClientMessage>>,
    mut u_server: EventReader<IncomingEarlyUnreliableClientMessage<MouseMessage>>,
    mut peer: EventWriter<OutgoingReliableServerMessage<PeerReliableControllerMessage>>,
    mut peer_unreliable: EventWriter<OutgoingUnreliableServerMessage<PeerMouseMessage>>,
    players: Query<&ConnectedPlayer>,
) {
    for message in server.iter() {
        for connected in players.iter() {
            if !connected.connected {
                continue;
            }
            if message.handle == connected.handle {
                continue;
            }

            peer.send(OutgoingReliableServerMessage {
                handle: connected.handle,
                message: PeerReliableControllerMessage {
                    message: message.message.clone(),
                    peer_handle: message.handle,
                    client_stamp: message.stamp,
                },
            });
        }
    }
    for message in u_server.iter() {
        for connected in players.iter() {
            if !connected.connected {
                continue;
            }
            if message.handle == connected.handle {
                continue;
            }

            peer_unreliable.send(OutgoingUnreliableServerMessage {
                handle: connected.handle,
                message: PeerMouseMessage {
                    message: message.message.clone(),
                    peer_handle: message.handle,
                    client_stamp: message.stamp,
                },
            });
        }
    }
}

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ControllerClientMessage>>,
    mut u_server: EventReader<IncomingUnreliableClientMessage<ControllerUnreliableClientMessage>>,
    mut input_ui_input: EventWriter<InputUIInput>,
    mut ui_input_transmit_text: EventWriter<InputUIInputTransmitText>,
    mut movement_input_event: EventWriter<InputMovementInput>,
    handle_to_entity: Res<HandleToEntity>,
    mut build_graphics_event: EventWriter<InputBuildGraphics>,
    mut input_sprinting_event: EventWriter<InputSprinting>,
    mut input_toggle_combat_mode: EventWriter<InputToggleCombatMode>,
    mut input_mouse_action: EventWriter<InputMouseAction>,
    mut mouse_direction_update: EventWriter<InputMouseDirectionUpdate>,
    input_tuple: (
        EventWriter<InputToggleAutoMove>,
        EventWriter<InputAttackEntity>,
        EventWriter<InputAltItemAttack>,
        EventWriter<InputAttackCell>,
    ),
) {
    let (
        mut input_toggle_auto_move,
        mut input_attack_entity,
        mut input_alt_item_attack,
        mut input_attack_cell,
    ) = input_tuple;

    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            ControllerClientMessage::UIInput(node_class, action, node_name, ui_type) => {
                input_ui_input.send(InputUIInput {
                    handle: message.handle,
                    node_class: node_class,
                    action: action,
                    node_name: node_name,
                    ui_type: ui_type,
                });
            }
            ControllerClientMessage::UIInputTransmitData(ui_type, node_path, input_text) => {
                ui_input_transmit_text.send(InputUIInputTransmitText {
                    handle: message.handle,
                    ui_type: ui_type,
                    node_path: node_path,
                    input_text: input_text,
                });
            }

            ControllerClientMessage::MovementInput(movement_input) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        movement_input_event.send(InputMovementInput {
                            player_entity: *player_entity,
                            pressed: movement_input.pressed,
                            up: movement_input.up,
                            left: movement_input.left,
                            right: movement_input.right,
                            down: movement_input.down,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                }
            }

            ControllerClientMessage::BuildGraphics => {
                build_graphics_event.send(InputBuildGraphics {
                    handle: message.handle,
                });
            }

            ControllerClientMessage::SprintInput(is_sprinting) => {
                match handle_to_entity.map.get(&message.handle) {
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
                match handle_to_entity.map.get(&message.handle) {
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
                match handle_to_entity.map.get(&message.handle) {
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

            ControllerClientMessage::ToggleAutoMove => {
                match handle_to_entity.map.get(&message.handle) {
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
                match handle_to_entity.map.get(&message.handle) {
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
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_alt_item_attack.send(InputAltItemAttack {
                            entity: *player_entity,
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to AltItemAttack sender handle."
                        );
                    }
                }
            }

            ControllerClientMessage::AttackCell(cell_x, cell_y, cell_z) => {
                match handle_to_entity.map.get(&message.handle) {
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

    for message in u_server.iter() {
        let client_message = message.message.clone();

        match client_message {
            ControllerUnreliableClientMessage::MouseDirectionUpdate(
                mouse_direction,
                time_stamp,
            ) => match handle_to_entity.map.get(&message.handle) {
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
            },
        }
    }
}
