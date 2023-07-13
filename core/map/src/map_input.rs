use crate::map::Map;
use bevy::{
    math::Vec2,
    prelude::{Entity, Event, EventReader, Query, Resource},
};
use entity::senser::WORLD_WIDTH_CELLS;
use resources::math::Vec2Int;

use std::collections::HashMap;
/// Read map input events and apply them to the Map component.

pub(crate) fn map_input(
    mut input_view_range_change_events: EventReader<InputMap>,
    mut map_holders: Query<&mut Map>,
) {
    for event in input_view_range_change_events.iter() {
        match map_holders.get_mut(event.entity) {
            Ok(mut map_component) => match event.input {
                MapInput::Range(range_x) => {
                    map_component.view_range =
                        range_x.clamp(0., (WORLD_WIDTH_CELLS / 2) as f32) as usize;
                }
                MapInput::Position(position) => {
                    let width = WORLD_WIDTH_CELLS as f32 * 2. - 1.;
                    map_component.camera_position =
                        position.clamp(Vec2::new(-width, -width), Vec2::new(width, width));
                }
                MapInput::MouseCell(idx, idy) => {
                    map_component.passed_mouse_cell = Some((idx, idy));
                }
            },
            Err(_) => {
                continue;
            }
        }
    }
}

use bevy::prelude::EventWriter;
use networking::server::OutgoingReliableServerMessage;

use crate::net::MapServerMessage;
/// Request available map overlays.

pub(crate) fn request_map_overlay(
    mut events: EventReader<InputMapRequestOverlay>,
    map_holders: Query<&Map>,
    mut server: EventWriter<OutgoingReliableServerMessage<MapServerMessage>>,
) {
    for event in events.iter() {
        let map_component;

        match map_holders.get(event.entity) {
            Ok(m) => {
                map_component = m;
            }
            Err(_) => {
                continue;
            }
        }
        server.send(OutgoingReliableServerMessage {
            handle: event.handle,

            message: MapServerMessage::MapSendDisplayModes(
                map_component.available_display_modes.clone(),
            ),
        });
    }
}

/// Mini-map data resource.
#[derive(Default, Resource)]

pub struct MapData {
    pub data: HashMap<Vec2Int, i16>,
}

impl MapData {
    pub fn to_net(&self) -> Vec<(i16, i16, i16)> {
        let mut net_data = vec![];

        for (id, item) in self.data.iter() {
            net_data.push((id.x, id.y, *item));
        }

        net_data
    }
}

/// Client input change display mode mini-map event.
#[derive(Event)]
pub struct InputMapChangeDisplayMode {
    pub handle: u64,
    pub entity: Entity,
    pub display_mode: String,
}

/// Client map input.

pub enum MapInput {
    Range(f32),
    Position(Vec2),
    MouseCell(i16, i16),
}

/// Client map input event.
#[derive(Event)]
pub struct InputMap {
    pub handle: u64,
    pub entity: Entity,
    pub input: MapInput,
}

/// Client map request display modes event.
#[derive(Event)]
pub struct InputMapRequestOverlay {
    pub handle: u64,
    pub entity: Entity,
}
use crate::net::{MapReliableClientMessage, MapUnreliableClientMessage};

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<MapReliableClientMessage>>,
    mut u_server: EventReader<IncomingUnreliableClientMessage<MapUnreliableClientMessage>>,
    mut input_map_change_display_mode: EventWriter<InputMapChangeDisplayMode>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_map_request_display_modes: EventWriter<InputMapRequestOverlay>,
    mut input_map_view_range: EventWriter<InputMap>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            MapReliableClientMessage::MapChangeDisplayMode(display_mode) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_map_change_display_mode.send(InputMapChangeDisplayMode {
                            handle: message.handle,
                            entity: *player_entity,
                            display_mode,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to MapChangeDisplayMode sender handle.");
                    }
                }
            }

            MapReliableClientMessage::MapRequestDisplayModes => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_map_request_display_modes.send(InputMapRequestOverlay {
                            handle: message.handle,
                            entity: *player_entity,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to input_map_request_display_modes sender handle.");
                    }
                }
            }

            MapReliableClientMessage::MapCameraPosition(position) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_map_view_range.send(InputMap {
                            handle: message.handle,
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

        for message in u_server.iter() {
            let client_message = message.message.clone();
            match client_message {
                MapUnreliableClientMessage::MapViewRange(range_x) => {
                    match handle_to_entity.map.get(&message.handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: message.handle,
                                entity: *player_entity,
                                input: MapInput::Range(range_x),
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to MapViewRange sender handle.");
                        }
                    }
                }
                MapUnreliableClientMessage::MapOverlayMouseHoverCell(idx, idy) => {
                    match handle_to_entity.map.get(&message.handle) {
                        Some(player_entity) => {
                            input_map_view_range.send(InputMap {
                                handle: message.handle,
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

use bevy::prelude::Res;
use networking::server::HandleToEntity;

use networking::server::{IncomingReliableClientMessage, IncomingUnreliableClientMessage};

use bevy::prelude::warn;
