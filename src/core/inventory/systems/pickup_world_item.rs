use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, Query, Res},
};
use bevy_log::warn;
use bevy_math::Vec3;
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{CollisionGroups, ExternalForce, GravityScale, Sleeping},
};
use bevy_transform::prelude::Transform;

use crate::core::{
    connected_player::resources::HandleToEntity,
    entity::components::EntityData,
    gridmap::{
        components::Cell,
        resources::{GridmapData, GridmapMain},
    },
    health::components::Health,
    inventory::{
        components::Inventory,
        events::{InputUseWorldItem, NetPickupWorldItem},
    },
    inventory_item::components::InventoryItem,
    networking::resources::ReliableServerMessage,
    pawn::functions::can_reach_entity::{can_reach_entity, REACH_DISTANCE},
    physics::components::{WorldMode, WorldModes},
    rigid_body::{components::RigidBodyLinkTransform, functions::disable_rigidbody},
};

pub fn pickup_world_item<'a>(
    mut use_world_item_events: EventReader<InputUseWorldItem>,
    mut inventory_entities: Query<&mut Inventory>,
    mut inventory_items_query: Query<&mut InventoryItem>,
    health_query: Query<&Health>,
    mut q: Query<(
        &mut WorldMode,
        &mut Sleeping,
        &mut CollisionGroups,
        &mut ExternalForce,
        &EntityData,
        &mut GravityScale,
    )>,
    rigidbody_positions: Query<&Transform>,
    mut commands: Commands,
    mut net_pickup_world_item: EventWriter<NetPickupWorldItem>,
    query_pipeline: Res<RapierContext>,
    handle_to_entity: Res<HandleToEntity>,
    gridmap_main: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
    cell_query: Query<&Cell>,
) {
    for event in use_world_item_events.iter() {
        let pickuper_components_option = inventory_entities.get_mut(event.pickuper_entity);
        let pickuper_components;

        match pickuper_components_option {
            Ok(components) => {
                pickuper_components = components;
            }
            Err(_rr) => {
                warn!("Couldnt find Inventory component belonging to pickuper_entity.");
                continue;
            }
        }

        let mut pickuper_inventory = pickuper_components;

        let pickup_slot = pickuper_inventory.active_slot.clone();

        let pickup_slot = pickuper_inventory.get_slot_mut(&pickup_slot);

        if !matches!(pickup_slot.slot_item, None) {
            continue;
        }

        let pickupable_entity = Entity::from_bits(event.pickupable_entity_bits);

        match inventory_items_query.get_component_mut::<InventoryItem>(pickupable_entity) {
            Ok(pickupable_inventory_item_component) => {
                if !matches!(
                    pickupable_inventory_item_component.in_inventory_of_entity,
                    None
                ) {
                    continue;
                }
            }
            Err(_rr) => {
                warn!("Couldnt find InventoryItem component belonging to pickupable_entity.");
                continue;
            }
        }

        let pickupable_position : Vec3 = rigidbody_positions.get(pickupable_entity)
        .expect("pickup_world_item.rs pickupable_entity was not found in rigidbody_positions query.")
        .translation.into();

        let pickuper_position: Vec3 = rigidbody_positions
            .get(event.pickuper_entity)
            .expect(
                "pickup_world_item.rs pickuper_entity was not found in rigidbody_positions query.",
            )
            .translation
            .into();

        if pickupable_position.distance(pickuper_position) > REACH_DISTANCE {
            continue;
        }

        if !can_reach_entity(
            &query_pipeline,
            pickuper_position,
            pickupable_position,
            &pickupable_entity,
            &event.pickuper_entity,
            &health_query,
            &cell_query,
            &gridmap_main,
            &gridmap_data,
            false,
        ) {
            continue;
        }

        let pickupable_entities_components;

        match q.get_mut(pickupable_entity) {
            Ok(s) => {
                pickupable_entities_components = s;
            }
            Err(_rr) => {
                warn!("Couldnt find components belonging to pickupable_entity.");
                continue;
            }
        }

        let mut pickupable_world_mode = pickupable_entities_components.0;
        let mut pickupable_rigid_body_activation = pickupable_entities_components.1;
        let mut pickupable_collider_bundle = pickupable_entities_components.2;
        let mut _pickupable_rigid_body_forces = pickupable_entities_components.3;
        let mut pickupable_rigid_body_gravity = pickupable_entities_components.5;

        let pickupable_entity_data = pickupable_entities_components.4;

        disable_rigidbody(
            &mut pickupable_rigid_body_activation,
            &mut pickupable_collider_bundle,
            &mut pickupable_rigid_body_gravity,
            &mut commands,
            pickupable_entity,
        );

        let mut pickupable_inventory_item_component;

        match inventory_items_query.get_mut(pickupable_entity) {
            Ok(s) => {
                pickupable_inventory_item_component = s;
            }
            Err(_rr) => {
                warn!("Couldnt find InventoryItem component of pickupable entity.");
                continue;
            }
        }

        pickupable_inventory_item_component.in_inventory_of_entity = Some(event.pickuper_entity);
        pickup_slot.slot_item = Some(pickupable_entity);
        pickupable_world_mode.mode = WorldModes::Held;

        commands
            .entity(pickupable_entity)
            .insert(RigidBodyLinkTransform {
                follow_entity: event.pickuper_entity,
                ..Default::default()
            });

        match handle_to_entity.inv_map.get(&event.pickuper_entity) {
            Some(handle) => {
                net_pickup_world_item.send(NetPickupWorldItem {
                    handle: *handle,
                    message: ReliableServerMessage::PickedUpItem(
                        pickupable_entity_data.entity_name.clone(),
                        event.pickupable_entity_bits,
                        pickup_slot.slot_name.clone(),
                    ),
                });
            }
            None => {}
        }
    }
}
