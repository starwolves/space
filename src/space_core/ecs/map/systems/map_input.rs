use bevy::{prelude::{EventReader, Query}, math::Vec2};

use crate::space_core::ecs::map::{events::InputMap, components::Map};

pub fn map_input (
    mut input_view_range_change_events : EventReader<InputMap>,
    mut map_holders : Query<&mut Map>,
) {

    for event in input_view_range_change_events.iter() {

        match map_holders.get_mut(event.entity) {
            Ok(mut map_component) => {

                match event.input {
                    crate::space_core::ecs::map::events::MapInput::Range(range) => {
                        map_component.view_range = range.clamp(0.,60.);
                    },
                    crate::space_core::ecs::map::events::MapInput::Position(position) => {
                        map_component.camera_position = position.clamp(Vec2::new(-1000.,-1000.),Vec2::new(1000.,1000.));
                    }, 
                }

                
            },
            Err(_) => {
                continue;
            },
        }

    }

}
