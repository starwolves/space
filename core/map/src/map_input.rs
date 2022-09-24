use crate::map::Map;
use api::{
    gridmap::FOV_MAP_WIDTH,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
};
use bevy::{
    math::Vec2,
    prelude::{EventReader, EventWriter, Query},
};
use networking::messages::{InputMap, InputMapRequestDisplayModes, MapInput};

use api::data::Vec2Int;
use std::collections::HashMap;
/// Manage map input.
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

/// Request available map display modes.
pub(crate) fn request_display_modes(
    mut events: EventReader<InputMapRequestDisplayModes>,
    map_holders: Query<&Map>,
    mut net: EventWriter<NetRequestDisplayModes>,
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

        net.send(NetRequestDisplayModes {
            handle: event.handle,
            message: ReliableServerMessage::MapSendDisplayModes(
                map_component.available_display_modes.clone(),
            ),
        });
    }
}

pub(crate) struct NetRequestDisplayModes {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetRequestDisplayModes {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Mini-map data resource.
#[derive(Default)]
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
