use bevy::prelude::ResMut;

use bevy::prelude::warn;
use bevy::prelude::Vec2;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

use bevy::prelude::EventWriter;
use serde::Deserialize;
use serde::Serialize;

use crate::map_input::InputMap;
use crate::map_input::InputMapChangeDisplayMode;
use crate::map_input::MapInput;
use bevy::prelude::Res;
use networking::server::HandleToEntity;

use crate::map_input::InputMapRequestOverlay;
use networking::plugin::RENET_UNRELIABLE_CHANNEL_ID;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum MapMessage {
    MapChangeDisplayMode(String),
    MapRequestDisplayModes,
    MapCameraPosition(Vec2),
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum MapUnreliableMessage {
    MapViewRange(f32),
    MapOverlayMouseHoverCell(i16, i16),
}

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    mut input_map_change_display_mode: EventWriter<InputMapChangeDisplayMode>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_map_request_display_modes: EventWriter<InputMapRequestOverlay>,
    mut input_map_view_range: EventWriter<InputMap>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<MapMessage, _> = bincode::deserialize(&message);
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
                MapMessage::MapChangeDisplayMode(display_mode) => {
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

                MapMessage::MapRequestDisplayModes => match handle_to_entity.map.get(&handle) {
                    Some(player_entity) => {
                        input_map_request_display_modes.send(InputMapRequestOverlay {
                            handle: handle,
                            entity: *player_entity,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to input_map_request_display_modes sender handle.");
                    }
                },

                MapMessage::MapCameraPosition(position) => {
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

        while let Some(message) = server.receive_message(handle, RENET_UNRELIABLE_CHANNEL_ID) {
            let client_message: MapUnreliableMessage = bincode::deserialize(&message).unwrap();

            match client_message {
                MapUnreliableMessage::MapViewRange(range_x) => {
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
                MapUnreliableMessage::MapOverlayMouseHoverCell(idx, idy) => {
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

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum MapServerMessage {
    MapSendDisplayModes(Vec<(String, String)>),
    MapOverlayUpdate(Vec<(i16, i16, i16)>),
    MapOverlayHoverData(String),
    MapDefaultAddition(i16, i16, i16),
}
