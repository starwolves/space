use crate::map::Map;
use bevy::{
    math::Vec2,
    prelude::{Entity, EventReader, EventWriter, Query},
};
use entity::senser::FOV_MAP_WIDTH;
use math::grid::Vec2Int;
use networking::server::ReliableServerMessage;

use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking_macros::NetMessage;
use std::collections::HashMap;
/// Read map input events and apply them to the Map component.
#[cfg(feature = "server")]
pub(crate) fn map_input(
    mut input_view_range_change_events: EventReader<InputMap>,
    mut map_holders: Query<&mut Map>,
) {
    for event in input_view_range_change_events.iter() {
        match map_holders.get_mut(event.entity) {
            Ok(mut map_component) => match event.input {
                MapInput::Range(range_x) => {
                    map_component.view_range =
                        range_x.clamp(0., (FOV_MAP_WIDTH / 2) as f32) as usize;
                }
                MapInput::Position(position) => {
                    let width = FOV_MAP_WIDTH as f32 * 2. - 1.;
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

/// Request available map overlays.
#[cfg(feature = "server")]
pub(crate) fn request_map_overlay(
    mut events: EventReader<InputMapRequestOverlay>,
    map_holders: Query<&Map>,
    mut net: EventWriter<NetRequestOverlay>,
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

        net.send(NetRequestOverlay {
            handle: event.handle,
            message: ReliableServerMessage::MapSendDisplayModes(
                map_component.available_display_modes.clone(),
            ),
        });
    }
}

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetRequestOverlay {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// Mini-map data resource.
#[derive(Default)]
#[cfg(feature = "server")]
pub struct MapData {
    pub data: HashMap<Vec2Int, i16>,
}
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub struct InputMapChangeDisplayMode {
    pub handle: u64,
    pub entity: Entity,
    pub display_mode: String,
}

/// Client map input.
#[cfg(feature = "server")]
pub enum MapInput {
    Range(f32),
    Position(Vec2),
    MouseCell(i16, i16),
}

/// Client map input event.
#[cfg(feature = "server")]
pub struct InputMap {
    pub handle: u64,
    pub entity: Entity,
    pub input: MapInput,
}

/// Client map request display modes event.
#[cfg(feature = "server")]
pub struct InputMapRequestOverlay {
    pub handle: u64,
    pub entity: Entity,
}
