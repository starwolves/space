use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Commands, EventReader, EventWriter, Query, QuerySet, Res, Transform, warn, QueryState, Entity}};
use bevy_rapier3d::prelude::{QueryPipeline, RigidBodyPositionComponent, RigidBodyForcesComponent, RigidBodyActivationComponent, ColliderFlagsComponent, ColliderPositionComponent, ColliderShapeComponent};

use crate::space_core::{components::{cell::Cell, health::Health, inventory::Inventory, inventory_item::InventoryItem, pawn::Pawn, rigidbody_link_transform::RigidBodyLinkTransform, sensable::Sensable, world_mode::{WorldMode, WorldModes}}, events::{general::drop_current_item::InputDropCurrentItem, net::{net_drop_current_item::NetDropCurrentItem}}, functions::{converters::{isometry_to_transform::isometry_to_transform, transform_to_isometry::transform_to_isometry}, entity::{can_reach_entity::{REACH_DISTANCE, can_reach_entity}, entity_spawn_position_for_player::entity_spawn_position_for_player, toggle_rigidbody::enable_rigidbody}}, resources::{gridmap_data::GridmapData, gridmap_main::GridmapMain, handle_to_entity::HandleToEntity, network_messages::{EntityUpdateData, EntityWorldType, ReliableServerMessage}}};

pub fn drop_current_item<'a>(
    mut drop_current_item_events : EventReader<InputDropCurrentItem>,
    mut rigidbody_positions : Query<&mut RigidBodyPositionComponent>,
    mut inventory_entities : Query<(
        &mut Inventory,
        &Sensable,
        &Pawn,
    )>,
    mut inventory_items_query : Query<&mut InventoryItem>,
    health_query : Query<&Health>,
    cell_query : Query<&Cell>,
    mut q: QuerySet<(
        QueryState<(
            &mut WorldMode,
            &mut RigidBodyActivationComponent,
            &mut ColliderFlagsComponent,
            &mut RigidBodyForcesComponent,
            &mut RigidBodyLinkTransform,
        )>,
        QueryState<(
            Entity,
            &'a ColliderPositionComponent,
            &'a ColliderShapeComponent,
            &'a ColliderFlagsComponent,
        )>,
    )>,
    mut commands : Commands,
    mut net_drop_current_item : EventWriter<NetDropCurrentItem>,
    handle_to_entity : Res<HandleToEntity>,
    gridmap_main : Res<GridmapMain>,
    gridmap_data : Res<GridmapData>,
    query_pipeline: Res<QueryPipeline>,
) {

    for event in drop_current_item_events.iter() {

        let pickuper_components_option = inventory_entities.get_mut(event.pickuper_entity);
        let pickuper_components;

        match pickuper_components_option {
            Ok(components) => {
                pickuper_components = components;
            },
            Err(_rr) => {
                continue;
            },
        }

        let mut pickuper_inventory = pickuper_components.0;
        
        let pickup_slot = &pickuper_inventory.active_slot.clone();

        let drop_slot = pickuper_inventory.get_slot_mut(pickup_slot);

        let pickupable_entity;

        match drop_slot.slot_item {
            Some(item) => {
                pickupable_entity = item;
            },
            None => {
                continue;
            },
        }

        

        let inventory_item_component_prev = inventory_items_query.get_component_mut::<InventoryItem>(pickupable_entity)
        .expect("drop_current_item.rs couldnt find pickupable_components of pickupable_entity from query.");


        let mut new_position;

        let pawn_component = pickuper_components.2;
        
        let pickuper_position : Vec3;

        match event.input_position_option {
            Some(placing_position) => {
                
                match rigidbody_positions.get_component_mut::<RigidBodyPositionComponent>(event.pickuper_entity) {
                    Ok(pickuper_position_rapier) => {
                        
                        pickuper_position = pickuper_position_rapier.position.translation.into();
                        
                        if pickuper_position.distance(placing_position) > REACH_DISTANCE {
                            continue;
                        }

                        new_position = Transform {
                            translation: placing_position,
                            rotation: inventory_item_component_prev.drop_transform.rotation,
                            scale: inventory_item_component_prev.drop_transform.scale,
                        };

                    },
                    Err(_) => {
                        warn!("Couldn't find position of pickuper entity (2)!");
                        continue;
                    },
                }

            },
            None => {
                match rigidbody_positions.get_component_mut::<RigidBodyPositionComponent>(pickupable_entity) {
                    Ok(pickupable_rigidbody_position) => {
        
                        
        
                        let mut new_pickupable_transform = isometry_to_transform(pickupable_rigidbody_position.position);
        
                        let new_results = entity_spawn_position_for_player(
                            new_pickupable_transform,
                            Some(&pawn_component.facing_direction),
                            None,
                            &gridmap_main
                        );
        
                        new_pickupable_transform = new_results.0;
        
                        new_pickupable_transform.rotation = inventory_item_component_prev.drop_transform.rotation;
        
                        new_position = new_pickupable_transform.clone();
                        
                        match rigidbody_positions.get_component_mut::<RigidBodyPositionComponent>(event.pickuper_entity) {
                            Ok(rigidbody_pos) => {
                                pickuper_position = rigidbody_pos.position.translation.into();
                            },
                            Err(_rr) => {
                                warn!("Couldn't find position of pickuper entity (3)!");
                                continue;
                            },
                        }
        
                    },
                    Err(_rr) => {
                        warn!("Couldn't find rigidbodyposition of pickupable_entity!");
                        continue;
                    },
                }
            },
        }


        if event.input_position_option.is_some() {
            if !can_reach_entity(
                &query_pipeline,
                &q.q1(),
                pickuper_position,
                event.input_position_option.unwrap(),
                &pickupable_entity,
                &event.pickuper_entity,
                &health_query,
                &cell_query,
                &gridmap_main,
                &gridmap_data,
                true
            ) {
                continue;
            }
        }

        match rigidbody_positions.get_component_mut::<RigidBodyPositionComponent>(pickupable_entity) {
            Ok(mut rigidbody_pos) => {
                new_position.translation.y+=0.25;
                rigidbody_pos.position = transform_to_isometry(new_position);
            },
            Err(_rr) => {
                warn!("Couldn't find position of pickuper entity (3)!");
                continue;
            },
        }
        

        let mut inventory_item_component = inventory_items_query.get_mut(pickupable_entity)
        .expect("drop_current_item.rs couldnt find InventoryItem component of pickupable_entity from query.");

        let mut q0 = q.q0();

        let (
            mut pickupable_world_mode_component,
            mut pickupable_rigidbody_activation,
            mut pickupable_rigidbody_collider_flags,
            mut pickupable_rigidbody_forces,
            mut pickupable_rigidbody_link_transform_component,
        ) = q0.get_mut(pickupable_entity)
        .expect("drop_current_item.rs couldnt find pickupable_components of pickupable_entity from query.");
        
        drop_slot.slot_item = None;
        pickupable_world_mode_component.mode = WorldModes::Physics;
        inventory_item_component.in_inventory_of_entity = None;

        enable_rigidbody(
            &mut pickupable_rigidbody_activation,
            &mut pickupable_rigidbody_collider_flags,
            &mut pickupable_rigidbody_forces,
            &mut commands,
            pickupable_entity
        );

        pickupable_rigidbody_link_transform_component.active = false;

        commands.entity(pickupable_entity).remove::<RigidBodyLinkTransform>();
        
        

        match &drop_slot.slot_attachment {
            Some(attachment_path) => {

                // Create detachItem entityUpdate and send it to send_entity_update.rs 

                let mut root_entity_update = HashMap::new();

                let mut entity_update = HashMap::new();

                entity_update.insert("detachItem".to_string(), EntityUpdateData::AttachedItem(
                    pickupable_entity.to_bits(),
                    new_position.translation, 
                    new_position.rotation,
                    new_position.scale
                ));

                root_entity_update.insert(attachment_path.to_string(), entity_update);


                for entity_id in pickuper_components.1.sensed_by.iter() {

                    let handle_option = handle_to_entity.inv_map.get(&entity_id);
                    
                    match handle_option {
                        Some(handle) => {
                            
                            net_drop_current_item.send(NetDropCurrentItem {
                                handle: *handle,
                                message: ReliableServerMessage::EntityUpdate(
                                    entity_id.to_bits(),
                                    root_entity_update.clone(),
                                    false,
                                    EntityWorldType::Main,
                                )
                            });

                        },
                        None => {},
                    }


                }

            },
            None => {},
        }

        

        // Send UI/Control update to owning client.
        net_drop_current_item.send(NetDropCurrentItem {
            handle: event.handle,
            message: ReliableServerMessage::DropItem(drop_slot.slot_name.clone()),
        });

    }

}
