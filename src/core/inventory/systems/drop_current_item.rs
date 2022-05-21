use std::collections::HashMap;

use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::{Commands, Query, Res},
};
use bevy_hierarchy::Children;
use bevy_log::warn;
use bevy_math::Vec3;
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{CollisionGroups, ExternalForce, GravityScale, Sleeping},
};
use bevy_transform::components::Transform;

use crate::core::{
    connected_player::resources::HandleToEntity,
    gridmap::{
        components::Cell,
        resources::{GridmapData, GridmapMain},
    },
    health::components::Health,
    inventory::{
        components::Inventory,
        events::{InputDropCurrentItem, NetDropCurrentItem},
    },
    inventory_item::components::InventoryItem,
    networking::resources::{EntityUpdateData, EntityWorldType, ReliableServerMessage},
    pawn::{
        components::Pawn,
        functions::{
            can_reach_entity::{can_reach_entity, REACH_DISTANCE},
            entity_spawn_position_for_player::entity_spawn_position_for_player,
        },
    },
    physics::components::{WorldMode, WorldModes},
    rigid_body::{components::RigidBodyLinkTransform, functions::enable_rigidbody},
    sensable::components::Sensable,
};

pub fn drop_current_item<'a>(
    mut drop_current_item_events: EventReader<InputDropCurrentItem>,
    mut rigidbody_positions: Query<&mut Transform>,
    mut inventory_entities: Query<(&mut Inventory, &Sensable, &Pawn)>,
    mut inventory_items_query: Query<&mut InventoryItem>,
    health_query: Query<&Health>,
    cell_query: Query<&Cell>,
    mut q: Query<(
        &mut WorldMode,
        &mut Sleeping,
        &mut GravityScale,
        &mut ExternalForce,
        &mut RigidBodyLinkTransform,
        &Children,
    )>,
    mut collision_groups: Query<&mut CollisionGroups>,
    mut commands: Commands,
    mut net_drop_current_item: EventWriter<NetDropCurrentItem>,
    handle_to_entity: Res<HandleToEntity>,
    gridmap_main: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
    query_pipeline: Res<RapierContext>,
) {
    for event in drop_current_item_events.iter() {
        let pickuper_components_option = inventory_entities.get_mut(event.pickuper_entity);
        let pickuper_components;

        match pickuper_components_option {
            Ok(components) => {
                pickuper_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut pickuper_inventory = pickuper_components.0;

        let pickup_slot = &pickuper_inventory.active_slot.clone();

        let drop_slot = pickuper_inventory.get_slot_mut(pickup_slot);

        let pickupable_entity;

        match drop_slot.slot_item {
            Some(item) => {
                pickupable_entity = item;
            }
            None => {
                continue;
            }
        }

        let inventory_item_component_prev = inventory_items_query.get_component_mut::<InventoryItem>(pickupable_entity)
        .expect("drop_current_item.rs couldnt find pickupable_components of pickupable_entity from query. (0)");

        let mut new_position;

        let pawn_component = pickuper_components.2;

        let pickuper_position: Vec3;

        let pickuper_transform;

        match rigidbody_positions.get_component::<Transform>(event.pickuper_entity) {
            Ok(t) => {
                pickuper_transform = t.clone();
            }
            Err(_rr) => {
                warn!("!");
                continue;
            }
        }

        match event.input_position_option {
            Some(placing_position) => {
                match rigidbody_positions.get_component_mut::<Transform>(event.pickuper_entity) {
                    Ok(pickuper_position_rapier) => {
                        pickuper_position = pickuper_position_rapier.translation;

                        if pickuper_position.distance(placing_position) > REACH_DISTANCE {
                            continue;
                        }

                        new_position = Transform {
                            translation: placing_position,
                            rotation: inventory_item_component_prev.drop_transform.rotation,
                            scale: inventory_item_component_prev.drop_transform.scale,
                        };
                    }
                    Err(_) => {
                        warn!("Couldn't find position of pickuper entity (2)!");
                        continue;
                    }
                }
            }
            None => match rigidbody_positions.get_component_mut::<Transform>(pickupable_entity) {
                Ok(mut pickupable_rigidbody_position) => {
                    let new_results = entity_spawn_position_for_player(
                        pickuper_transform,
                        Some(&pawn_component.facing_direction),
                        None,
                        &gridmap_main,
                    );

                    pickupable_rigidbody_position.translation = new_results.0.translation;
                    pickupable_rigidbody_position.scale = new_results.0.scale;

                    pickupable_rigidbody_position.rotation =
                        inventory_item_component_prev.drop_transform.rotation;

                    new_position = pickupable_rigidbody_position.clone();

                    match rigidbody_positions.get_component_mut::<Transform>(event.pickuper_entity)
                    {
                        Ok(rigidbody_pos) => {
                            pickuper_position = rigidbody_pos.translation.into();
                        }
                        Err(_rr) => {
                            warn!("Couldn't find position of pickuper entity (3)!");
                            continue;
                        }
                    }
                }
                Err(_rr) => {
                    warn!("Couldn't find rigidbodyposition of pickupable_entity!");
                    continue;
                }
            },
        }

        if event.input_position_option.is_some() {
            if !can_reach_entity(
                &query_pipeline,
                pickuper_position,
                event.input_position_option.unwrap(),
                &pickupable_entity,
                &event.pickuper_entity,
                &health_query,
                &cell_query,
                &gridmap_main,
                &gridmap_data,
                true,
            ) {
                continue;
            }
        }

        match rigidbody_positions.get_component_mut::<Transform>(pickupable_entity) {
            Ok(mut rigidbody_pos) => {
                new_position.translation.y += 0.25;
                rigidbody_pos.translation = new_position.translation;
                rigidbody_pos.rotation = new_position.rotation;
                rigidbody_pos.scale = new_position.scale;
            }
            Err(_rr) => {
                warn!("Couldn't find position of pickuper entity (3)!");
                continue;
            }
        }

        let mut inventory_item_component = inventory_items_query.get_mut(pickupable_entity)
        .expect("drop_current_item.rs couldnt find InventoryItem component of pickupable_entity from query.");

        let (
            mut pickupable_world_mode_component,
            mut pickupable_rigidbody_activation,
            mut pickupable_rigidbody_gravity,
            mut _pickupable_rigidbody_forces,
            mut pickupable_rigidbody_link_transform_component,
            children,
        ) = q.get_mut(pickupable_entity)
        .expect("drop_current_item.rs couldnt find pickupable_components of pickupable_entity from query. (1)");

        drop_slot.slot_item = None;
        pickupable_world_mode_component.mode = WorldModes::Physics;
        inventory_item_component.in_inventory_of_entity = None;

        let mut collision_child_option = None;

        for child in children.iter() {
            match collision_groups.get(*child) {
                Ok(_r) => {
                    collision_child_option = Some(*child);
                }
                Err(_rr) => {}
            }
        }

        let mut group;

        match collision_child_option {
            Some(collision_entity) => {
                group = collision_groups.get_mut(collision_entity).unwrap();
            }
            None => {
                warn!("Couldnt find collider child!");
                break;
            }
        }

        enable_rigidbody(
            &mut pickupable_rigidbody_activation,
            &mut group,
            &mut pickupable_rigidbody_gravity,
            &mut commands,
            pickupable_entity,
        );

        pickupable_rigidbody_link_transform_component.active = false;

        commands
            .entity(pickupable_entity)
            .remove::<RigidBodyLinkTransform>();

        match &drop_slot.slot_attachment {
            Some(attachment_path) => {
                // Create detachItem entityUpdate and send it to send_entity_update.rs

                let mut root_entity_update = HashMap::new();

                let mut entity_update = HashMap::new();

                entity_update.insert(
                    "detachItem".to_string(),
                    EntityUpdateData::AttachedItem(
                        pickupable_entity.to_bits(),
                        new_position.translation,
                        new_position.rotation,
                        new_position.scale,
                    ),
                );

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
                                ),
                            });
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        match handle_to_entity.inv_map.get(&event.pickuper_entity) {
            Some(handle) => {
                // Send UI/Control update to owning client.
                net_drop_current_item.send(NetDropCurrentItem {
                    handle: *handle,
                    message: ReliableServerMessage::DropItem(drop_slot.slot_name.clone()),
                });
            }
            None => {
                continue;
            }
        }
    }
}
