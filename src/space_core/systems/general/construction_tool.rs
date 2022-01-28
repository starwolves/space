use std::collections::HashMap;

use bevy::prelude::{EventReader, Res, EventWriter, Query, Entity, warn, Commands, ResMut};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;
use rand::Rng;

use crate::space_core::{events::{general::{input_construct::InputConstruct, input_deconstruct::InputDeconstruct, input_construction_options::InputConstructionOptions, input_construction_options_selection::InputConstructionOptionsSelection}, net::net_construction_tool::NetConstructionTool}, resources::{gridmap_data::GridmapData, entity_data_resource::EntityDataResource, network_messages::{ReliableServerMessage, TextTreeBit}, handle_to_entity::HandleToEntity, sfx_auto_destroy_timers::SfxAutoDestroyTimers}, components::{construction_tool::ConstructionTool, sensable::Sensable, pawn::Pawn, inventory_item::InventoryItem, sfx::sfx_auto_destroy}, functions::{entity::new_chat_message::{FURTHER_ITALIC_FONT}, converters::isometry_to_transform::isometry_to_transform}, bundles::{ui_interaction1_sfx::UIInteraction1SfxBundle, ui_interaction2_sfx::UIInteraction2SfxBundle, ui_interaction3_sfx::UIInteraction3SfxBundle}};

pub fn construction_tool(
    _input_construct_event : EventReader<InputConstruct>,
    _input_deconstruct_event : EventReader<InputDeconstruct>,
    mut input_construction_options_event : EventReader<InputConstructionOptions>,
    mut input_construction_options_selection_event : EventReader<InputConstructionOptionsSelection>,

    entity_data : Res<EntityDataResource>,
    gridmap_data : Res<GridmapData>,
    handle_to_entity : Res<HandleToEntity>,

    mut net_construction_tool : EventWriter<NetConstructionTool>,

    mut construction_tools : Query<(Entity, &mut ConstructionTool, &Sensable, &InventoryItem, &RigidBodyPositionComponent)>,

    pawns : Query<&Pawn>,

    mut commands : Commands,

    mut sfx_auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,


) {

    // Retreive all construction and complex constructions as a text list and make generic client GUI text list call.
    for event in input_construction_options_event.iter() {

        let mut text_options = vec![];

        let entity = Entity::from_bits(event.belonging_entity);

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

        let mut pawn_name = "";

        let inventory_item_component = construction_tools.get_component::<InventoryItem>(entity).unwrap();

        match inventory_item_component.in_inventory_of_entity {
            Some(owner_entity) => {
                match pawns.get(owner_entity) {
                    Ok(pawn_component) => {
                        pawn_name = &pawn_component.name;
                    },
                    Err(_) => {
                        warn!("This construction tool's owner isnt a pawn!");
                    },
                }
            },
            None => {
                warn!("This construction tool has no owner!");
            },
        }

        

        let public_notification =     "[font=".to_owned() + FURTHER_ITALIC_FONT + "]" + pawn_name + " navigates the interface of the construction tool.[/font]";

        let sensable_component = construction_tools.get_component::<Sensable>(entity).unwrap();

        for sensed_by_entity in sensable_component.sensed_by.iter() {

            match handle_to_entity.inv_map.get(sensed_by_entity) {
                Some(senser_handle) => {
                    if senser_handle != &event.handle {
                        net_construction_tool.send(NetConstructionTool {
                            handle: *senser_handle,
                            message: ReliableServerMessage::ChatMessage(public_notification.clone()),
                        });
                    }
                },
                None => {
                    warn!("Couldn't find handle for entity!");
                },
            }

        }

        

        // Make a generic GUI netcode call now.
        net_construction_tool.send(NetConstructionTool {
            handle: event.handle,
            message: ReliableServerMessage::TextTreeSelection(Some(event.belonging_entity), "constructionoptions".to_string(),"constructiontoolco".to_string(), "Construction Options".to_string(), text_tree_selection_map),
        });

    }

    for event in input_construction_options_selection_event.iter() {

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

        let mut construction_tool_component = construction_tools.get_component_mut::<ConstructionTool>(event.entity).unwrap();

        if text_options.contains(&event.menu_selection) {
            construction_tool_component.construction_option = Some(event.menu_selection.clone());
        }

        let personal_update_text =     "[font=".to_owned() + FURTHER_ITALIC_FONT + "]" + "Cycled construction option " + &event.menu_selection + ".[/font]";

        net_construction_tool.send(NetConstructionTool {
            handle: event.handle,
            message: ReliableServerMessage::ChatMessage(personal_update_text),
        });

        let mut pawn_name = "";

        let (
            _s,
            _y,
            sensable_component,
            inventory_item_component,
            rgpc,
        ) = construction_tools.get(event.entity).unwrap();

        match inventory_item_component.in_inventory_of_entity {
            Some(owner_entity) => {
                match pawns.get(owner_entity) {
                    Ok(pawn_component) => {
                        pawn_name = &pawn_component.name;
                    },
                    Err(_) => {
                        warn!("This construction tool's owner isnt a pawn!");
                    },
                }
            },
            None => {
                warn!("This construction tool has no owner!");
            },
        }

        let public_notification =     "[font=".to_owned() + FURTHER_ITALIC_FONT + "]" + pawn_name + " cycles the options of the construction tool.[/font]";

        for sensed_by_entity in sensable_component.sensed_by.iter() {

            match handle_to_entity.inv_map.get(sensed_by_entity) {
                Some(senser_handle) => {
                    if senser_handle != &event.handle {
                        net_construction_tool.send(NetConstructionTool {
                            handle: *senser_handle,
                            message: ReliableServerMessage::ChatMessage(public_notification.clone()),
                        });
                    }
                    
                },
                None => {
                    warn!("Couldn't find handle for entity! (1)");
                },
            }

        }

        let mut rng = rand::thread_rng();
        let random_pick : i32 = rng.gen_range(0..3);

        let sfx_bundle;

        if random_pick == 0 {
            sfx_bundle = UIInteraction1SfxBundle::new(isometry_to_transform(rgpc.position));
        } else if random_pick == 1 {
            sfx_bundle = UIInteraction2SfxBundle::new(isometry_to_transform(rgpc.position));
        } else {
            sfx_bundle = UIInteraction3SfxBundle::new(isometry_to_transform(rgpc.position));
        }

        

        let sfx_entity = commands.spawn().insert_bundle(sfx_bundle).id();
        sfx_auto_destroy(sfx_entity,&mut sfx_auto_destroy_timers);


    }

    // Write to a SpawnShipCell event.
    
    // Write to a DespawnShipCell event.

}
