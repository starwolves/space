use bevy::{
    math::Vec2,
    prelude::{Entity, EventReader, EventWriter, Query},
};

use crate::core::{gridmap::gridmap::FOV_MAP_WIDTH, networking::networking::ReliableServerMessage};

use super::map_overlay::Map;

pub fn map_input(
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

pub struct InputMapChangeDisplayMode {
    pub handle: u64,
    pub entity: Entity,
    pub display_mode: String,
}

pub struct InputMapRequestDisplayModes {
    pub handle: u64,
    pub entity: Entity,
}

pub struct NetRequestDisplayModes {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct InputMap {
    pub handle: u64,
    pub entity: Entity,
    pub input: MapInput,
}

pub enum MapInput {
    Range(f32),
    Position(Vec2),
    MouseCell(i16, i16),
}

pub fn request_display_modes(
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
