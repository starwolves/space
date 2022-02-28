
use bevy_internal::{prelude::{EventReader, Query}, math::Vec2};

use crate::space::core::{
    gridmap::resources::FOV_MAP_WIDTH,
    map::{components::Map, events::InputMap},
};

pub fn map_input(
    mut input_view_range_change_events: EventReader<InputMap>,
    mut map_holders: Query<&mut Map>,
) {
    for event in input_view_range_change_events.iter() {
        match map_holders.get_mut(event.entity) {
            Ok(mut map_component) => match event.input {
                crate::space::core::map::events::MapInput::Range(range_x) => {
                    map_component.view_range =
                        range_x.clamp(0., (FOV_MAP_WIDTH / 2) as f32) as usize;
                }
                crate::space::core::map::events::MapInput::Position(position) => {
                    let width = FOV_MAP_WIDTH as f32 * 2. - 1.;
                    map_component.camera_position =
                        position.clamp(Vec2::new(-width, -width), Vec2::new(width, width));
                }
                crate::space::core::map::events::MapInput::MouseCell(idx, idy) => {
                    map_component.passed_mouse_cell = Some((idx, idy));
                }
            },
            Err(_) => {
                continue;
            }
        }
    }
}
