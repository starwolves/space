use bevy::prelude::EventReader;

use crate::space_core::events::general::{input_construct::InputConstruct, input_deconstruct::InputDeconstruct, input_construction_options::InputConstructionOptions};

pub fn construction_tool(
    _input_construct_event : EventReader<InputConstruct>,
    _input_deconstruct_event : EventReader<InputDeconstruct>,
    _input_construction_options_event : EventReader<InputConstructionOptions>,
) {

    // Add generic function main tile generator with right mouse click on this entity in inventory.

    // Write to a SpawnShipCell event.
    
    // Write to a DespawnShipCell event.

}
