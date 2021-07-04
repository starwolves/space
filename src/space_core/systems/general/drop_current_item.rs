use bevy::prelude::{Commands, EventReader, EventWriter, Query};
use bevy_rapier3d::prelude::{ColliderFlags, RigidBodyActivation, RigidBodyForces};

use crate::space_core::{components::{inventory::Inventory, pickupable::Pickupable, world_mode::{WorldMode, WorldModes}}, events::{general::drop_current_item::DropCurrentItem, net::net_drop_current_item::NetDropCurrentItem}, functions::toggle_rigidbody::enable_rigidbody, structs::network_messages::ReliableServerMessage};

pub fn drop_current_item(
    mut drop_current_item_events : EventReader<DropCurrentItem>,
    mut inventory_entities : Query<&mut Inventory>,
    mut pickupable_entities : Query<(
        &mut Pickupable,
        &mut WorldMode,
        &mut RigidBodyActivation,
        &mut ColliderFlags,
        &mut RigidBodyForces,
    )>,
    mut commands : Commands,
    mut net_drop_current_item : EventWriter<NetDropCurrentItem>,
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

        let mut pickuper_inventory = pickuper_components;
        
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

        // Send UI/Control update to owning client.
        net_drop_current_item.send(NetDropCurrentItem {
            handle: event.handle,
            message: ReliableServerMessage::DropItem(drop_slot.slot_name.clone()),
        });

    }

}
