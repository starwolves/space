use std::collections::HashMap;

use bevy::prelude::{EventReader, Res, EventWriter};

use crate::space_core::{events::{general::{input_construct::InputConstruct, input_deconstruct::InputDeconstruct, input_construction_options::InputConstructionOptions}, net::net_construction_tool::NetConstructionTool}, resources::{gridmap_data::GridmapData, entity_data_resource::EntityDataResource, network_messages::{ReliableServerMessage, TextTreeBit}}};

pub fn construction_tool(
    _input_construct_event : EventReader<InputConstruct>,
    _input_deconstruct_event : EventReader<InputDeconstruct>,
    mut input_construction_options_event : EventReader<InputConstructionOptions>,

    entity_data : Res<EntityDataResource>,
    gridmap_data : Res<GridmapData>,

    mut net_construction_tool : EventWriter<NetConstructionTool>,


) {

    // Retreive all construction and complex constructions as a text list and make generic client GUI text list call.
    for event in input_construction_options_event.iter() {

        let mut text_options = vec![];

        for entity_data_properties in entity_data.data.iter() {

            if entity_data_properties.constructable {
                text_options.push(entity_data_properties.name.clone());
            }

        }

        for (i, gridmap_data_properties) in gridmap_data.main_cell_properties.iter() {

            if gridmap_data_properties.constructable {
                text_options.push(gridmap_data.main_id_name_map.get(i).unwrap().to_string());
            }
            
        }

        text_options.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        let mut text_tree_selection_map = HashMap::new();

        text_tree_selection_map.insert("main".to_string(),TextTreeBit::Final(text_options));

        // Make a generic GUI netcode call now.
        net_construction_tool.send(NetConstructionTool {
            handle: event.handle,
            message: ReliableServerMessage::TextTreeSelection("Construction Options".to_string(), text_tree_selection_map),
        });


    }

    // Write to a SpawnShipCell event.
    
    // Write to a DespawnShipCell event.

}
