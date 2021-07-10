use bevy::{math::Vec3, prelude::{Commands, Entity, EventReader, EventWriter, Query}};
use bevy_rapier3d::prelude::{ColliderFlags, RigidBodyActivation, RigidBodyForces, RigidBodyPosition};

use crate::space_core::{components::{entity_data::EntityData, inventory::Inventory, inventory_item::InventoryItem, rigidbody_link_transform::RigidBodyLinkTransform, world_mode::{WorldMode, WorldModes}}, events::{general::use_world_item::UseWorldItem, net::net_pickup_world_item::NetPickupWorldItem}, functions::{toggle_rigidbody::disable_rigidbody}, structs::network_messages::ReliableServerMessage};

pub fn pickup_world_item(
    mut use_world_item_events : EventReader<UseWorldItem>,
    mut inventory_entities : Query<&mut Inventory>,
    mut pickupable_entities : Query<(
        &mut InventoryItem,
        &mut WorldMode,
        &mut RigidBodyActivation,
        &mut ColliderFlags,
        &mut RigidBodyForces,
        &EntityData,
    )>,
    rigidbody_positions : Query<&RigidBodyPosition>,
    mut commands : Commands,
    mut net_pickup_world_item : EventWriter<NetPickupWorldItem>,
) {

    for event in use_world_item_events.iter() {

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


        let mut pickuper_inventory = pickuper_components;

        let pickup_slot = pickuper_inventory.pickup_slot.clone();

        let pickup_slot = pickuper_inventory.get_slot_mut(&pickup_slot);

        if !matches!(pickup_slot.slot_item, None) {
            continue;
        }

        let pickupable_entities_components;

        let pickupable_entity = Entity::new(event.pickupable_entity_id);

        match pickupable_entities.get_mut(pickupable_entity) {
            Ok(components) => {
                pickupable_entities_components = components;
            },
            Err(_rr) => {
                continue;
            },
        }

        let mut pickupable_component = pickupable_entities_components.0;

        if !matches!(pickupable_component.in_inventory_of_entity, None) {
            continue;
        }

        let pickupable_position : Vec3 = rigidbody_positions.get(pickupable_entity)
        .expect("pickup_world_item.rs pickupable_entity was not found in rigidbody_positions query.")
        .position.translation.into();

        let pickuper_position : Vec3 = rigidbody_positions.get(event.pickuper_entity)
        .expect("pickup_world_item.rs pickuper_entity was not found in rigidbody_positions query.")
        .position.translation.into();


        if pickupable_position.distance(pickuper_position) > 2. {
            continue;
        }


        let mut pickupable_world_mode = pickupable_entities_components.1;
        let mut pickupable_rigid_body_activation = pickupable_entities_components.2;
        let mut pickupable_collider_bundle = pickupable_entities_components.3;
        let mut pickupable_rigid_body_forces = pickupable_entities_components.4;

        let pickupable_entity_data = pickupable_entities_components.5;

        disable_rigidbody(
            &mut pickupable_rigid_body_activation,
            &mut pickupable_collider_bundle,
            &mut pickupable_rigid_body_forces,
            &mut commands,
            pickupable_entity
        );
        
        pickupable_component.in_inventory_of_entity = Some(event.pickuper_entity);
        pickup_slot.slot_item = Some(pickupable_entity);
        pickupable_world_mode.mode = WorldModes::Held;

        commands.entity(pickupable_entity).insert(RigidBodyLinkTransform{
            follow_entity: event.pickuper_entity,
        });

        net_pickup_world_item.send(NetPickupWorldItem {
            handle: event.handle,
            message: ReliableServerMessage::PickedUpItem(pickupable_entity_data.entity_type.clone(), pickupable_entity.id(), pickup_slot.slot_name.clone()),
        });


    }

}
