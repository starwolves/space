use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Commands, EventReader, EventWriter, Query, Res}};
use bevy_rapier3d::prelude::{ColliderFlags, RigidBodyActivation, RigidBodyForces, RigidBodyPosition};

use crate::space_core::{components::{inventory::Inventory, inventory_item::InventoryItem, rigidbody_link_transform::RigidBodyLinkTransform, sensable::Sensable, world_mode::{WorldMode, WorldModes}}, events::{general::drop_current_item::DropCurrentItem, net::{net_drop_current_item::NetDropCurrentItem, net_send_entity_updates::NetSendEntityUpdates}}, functions::{toggle_rigidbody::enable_rigidbody, transform_to_isometry::transform_to_isometry}, resources::handle_to_entity::HandleToEntity, structs::network_messages::{EntityUpdateData, ReliableServerMessage}};

pub fn drop_current_item(
    mut drop_current_item_events : EventReader<DropCurrentItem>,
    mut rigidbody_positions : Query<&mut RigidBodyPosition>,
    mut inventory_entities : Query<(
        &mut Inventory,
        &Sensable,
    )>,
    mut pickupable_entities : Query<(
        &mut InventoryItem,
        &mut WorldMode,
        &mut RigidBodyActivation,
        &mut ColliderFlags,
        &mut RigidBodyForces,
    )>,
    mut commands : Commands,
    mut net_drop_current_item : EventWriter<NetDropCurrentItem>,
    mut net_send_entity_updates: EventWriter<NetSendEntityUpdates>,
    handle_to_entity : Res<HandleToEntity>,
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
        
        let pickup_slot = &pickuper_inventory.pickup_slot.clone();

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

        let (
            mut pickupable_component,
            mut pickupable_world_mode_component,
            mut pickupable_rigidbody_activation,
            mut pickupable_rigidbody_collider_flags,
            mut pickupable_rigidbody_forces,
        ) = pickupable_entities.get_mut(pickupable_entity)
        .expect("drop_current_item.rs couldnt find pickupable_components of pickupable_entity from query.");

        

        drop_slot.slot_item = None;
        pickupable_world_mode_component.mode = WorldModes::Physics;
        pickupable_component.in_inventory_of_entity = None;

        enable_rigidbody(
            &mut pickupable_rigidbody_activation,
            &mut pickupable_rigidbody_collider_flags,
            &mut pickupable_rigidbody_forces,
            &mut commands,
            pickupable_entity
        );

        

        commands.entity(pickupable_entity).remove_bundle::<(RigidBodyLinkTransform,)>();

        let new_position;

        
        match rigidbody_positions.get_component_mut::<RigidBodyPosition>(pickupable_entity) {
            Ok(mut position) => {

                let mut new_pickupable_transform = pickupable_component.drop_transform.clone();

                new_pickupable_transform.translation +=
                Vec3::new(
                    position.position.translation.x,
                    position.position.translation.y,
                    position.position.translation.z
                );
                //+ Vec3::new(0.45,0.,0.);

                new_position = new_pickupable_transform.clone();
                
                position.position = transform_to_isometry(new_pickupable_transform);

            },
            Err(_rr) => {
                continue;
            },
        }
        
        

        
        

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

                    let entity_idy = entity_id.id();

                    let handle_option = handle_to_entity.inv_map.get(&entity_idy);
                    
                    match handle_option {
                        Some(handle) => {
                            
                            net_send_entity_updates.send(NetSendEntityUpdates {
                                handle: *handle,
                                message: ReliableServerMessage::EntityUpdate(
                                    entity_id.to_bits(),
                                    root_entity_update.clone(),
                                    false,
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
