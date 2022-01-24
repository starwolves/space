use bevy::prelude::{EventReader, EventWriter, Query, Without};

use crate::space_core::{components::{connected_player::ConnectedPlayer, soft_player::SoftPlayer}, events::general::{examine_entity::InputExamineEntity, examine_map::InputExamineMap, input_tab_action::InputTabAction, input_construct::InputConstruct, input_deconstruct::InputDeconstruct, use_world_item::InputUseWorldItem}, resources::doryen_fov::Vec3Int};

pub fn tab_action(

    mut events : EventReader<InputTabAction>,
    mut event_examine_entity : EventWriter<InputExamineEntity>,
    mut event_examine_map : EventWriter<InputExamineMap>,
    mut event_construct : EventWriter<InputConstruct>,
    mut event_deconstruct : EventWriter<InputDeconstruct>,
    mut pickup_world_item_event : EventWriter<InputUseWorldItem>,
    criteria_query : Query<&ConnectedPlayer, Without<SoftPlayer>>,
) {

    for event in events.iter() {

        // Safety check.
        match criteria_query.get(event.player_entity) {
            Ok(_) => {},
            Err(_rr) => {
                continue;
            },
        }
        if event.tab_id == "examine" {
            match event.target_entity_option {
                Some(entity_bits) => {
                    event_examine_entity.send(InputExamineEntity{
                        handle: event.handle,
                        examine_entity_bits: entity_bits,
                        entity: event.player_entity,
                    });

                },
                None => {
                    match &event.target_cell_option {
                        Some((gridmap_type, idx, idy, idz)) => {

                            event_examine_map.send(InputExamineMap{
                                handle: event.handle,
                                entity: event.player_entity,
                                gridmap_type: gridmap_type.clone(),
                                gridmap_cell_id: Vec3Int {
                                    x: *idx,
                                    y: *idy,
                                    z: *idz,
                                },
                            });

                        },
                        None => {},
                    }

                },
            }

        } else if event.tab_id == "construct" {

            event_construct.send(InputConstruct {
                handle: event.handle,
                target_cell: event.target_cell_option.as_ref().unwrap().clone(),
            });

        } else if event.tab_id == "deconstruct" {

            event_deconstruct.send(InputDeconstruct {
                handle: event.handle,
                target_cell: event.target_cell_option.as_ref().unwrap().clone(),
            });

        } else if event.tab_id == "pickup" {

            pickup_world_item_event.send(InputUseWorldItem {
                handle: event.handle,
                pickuper_entity: event.player_entity,
                pickupable_entity_bits: event.target_entity_option.unwrap(),
            });

        }

    }

}
