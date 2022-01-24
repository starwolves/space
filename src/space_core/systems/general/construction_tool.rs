use bevy::prelude::EventReader;

use crate::space_core::events::general::{input_construct::InputConstruct, input_deconstruct::InputDeconstruct};

pub fn construction_tool(
    _input_construct_event : EventReader<InputConstruct>,
    _input_deconstruct_event : EventReader<InputDeconstruct>,
) {

    

}
