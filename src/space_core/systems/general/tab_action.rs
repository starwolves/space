use bevy::{prelude::{EventReader, EventWriter, Query, Without, warn, Entity, Res}, math::Vec3};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space_core::{components::{connected_player::ConnectedPlayer, soft_player::SoftPlayer, pawn::Pawn, inventory::Inventory}, events::general::{examine_entity::InputExamineEntity, examine_map::InputExamineMap, input_tab_action::InputTabAction, input_construct::InputConstruct, input_deconstruct::InputDeconstruct, use_world_item::InputUseWorldItem, input_construction_options::InputConstructionOptions}, resources::{doryen_fov::Vec3Int, gridmap_main::GridmapMain}, functions::gridmap::gridmap_functions::cell_id_to_world};

pub fn tab_action(

    mut events : EventReader<InputTabAction>,
    mut event_examine_entity : EventWriter<InputExamineEntity>,
    mut event_examine_map : EventWriter<InputExamineMap>,
    mut event_construct : EventWriter<InputConstruct>,
    mut event_deconstruct : EventWriter<InputDeconstruct>,
    mut pickup_world_item_event : EventWriter<InputUseWorldItem>,
    mut event_construction_options : EventWriter<InputConstructionOptions>,
    criteria_query : Query<&ConnectedPlayer, Without<SoftPlayer>>,

    pawns : Query<(&Pawn, &RigidBodyPositionComponent, &Inventory)>,
    inventory_items : Query<&RigidBodyPositionComponent>,

    gridmap_main_data : Res<GridmapMain>,

) {

    for event in events.iter() {

        // Safety check.
        match criteria_query.get(event.player_entity) {
            Ok(_) => {},
            Err(_rr) => {
                continue;
            },
        }


        let pawn_component;
        let pawn_inventory_component;
        let pawn_rigid_body_position_component;
        
        match pawns.get(event.player_entity) {
            Ok((c, c1, c2)) => {
                pawn_component=c;
                pawn_rigid_body_position_component=c1;
                pawn_inventory_component=c2;
            },
            Err(_rr) => {
                warn!("Couldn't find pawn_component.");
                continue;
            },
        }

        let distance;
        let start_pos : Vec3;
        let end_pos : Vec3 = pawn_rigid_body_position_component.0.position.translation.into();

        match event.target_entity_option {
            Some(target_entity) => {

                let rigid_body_position_component = inventory_items.get(Entity::from_bits(target_entity)).unwrap();
                start_pos = rigid_body_position_component.0.position.translation.into();

            },
            None => {
                let cell_data = event.target_cell_option.as_ref().unwrap();
                start_pos = cell_id_to_world(Vec3Int{ x: cell_data.1, y: cell_data.2, z: cell_data.3 });
                
            },
        }

        distance = start_pos.distance(end_pos);

        let mut index_option = None;

        for (_entity_option, action_id_index_map) in pawn_component.tab_actions_data.layout.iter() {

            for (action_id , index) in action_id_index_map {

                if action_id == &event.tab_id {
                    index_option=Some(index);
                    break;

                }

            }

        }

        match index_option {
            Some(index) => {

                let action = pawn_component.tab_actions.get(index).unwrap();

                let self_belonging_entity;

                match event.belonging_entity {
                    Some(e) => {
                        self_belonging_entity = Some(Entity::from_bits(e));
                    },
                    None => {self_belonging_entity=None;},
                }

                

                let mut cell_option = None;

                match &event.target_cell_option {
                    Some(gridmap_cell_data) => {
                        let cell_item;
                        match gridmap_main_data.data.get(&Vec3Int{
                            x:gridmap_cell_data.1,
                            y:gridmap_cell_data.2,
                            z:gridmap_cell_data.3
                        }) {
                            Some(x) => {
                                cell_item = Some(x);
                            },
                            None => {
                                cell_item = None;
                            },
                        }
                        cell_option = Some((gridmap_cell_data.0.clone(), gridmap_cell_data.1, gridmap_cell_data.2, gridmap_cell_data.3, cell_item))
                    },
                    None => {},
                }

                

                // Safety check 2.
                match (action.prerequisite_check)(self_belonging_entity, event.target_entity_option, cell_option, distance, pawn_inventory_component) {
                    true => {},
                    false => {
                        continue;
                    }
                }

            },
            None => {
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
                belonging_entity: event.belonging_entity.unwrap()
            });

        } else if event.tab_id == "deconstruct" {

            event_deconstruct.send(InputDeconstruct {
                handle: event.handle,
                target_cell: event.target_cell_option.as_ref().unwrap().clone(),
                belonging_entity: event.belonging_entity.unwrap()
            });

        } else if event.tab_id == "constructionoptions" {

            event_construction_options.send(InputConstructionOptions {
                handle: event.handle,
                belonging_entity: event.belonging_entity.unwrap()
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
