use std::collections::HashMap;

use actions::core::{ActionRequests, BuildingActions};

use atmospherics::zero_gravity::ZeroGravity;
use bevy::{
    hierarchy::{Children, Parent},
    math::Vec3,
    prelude::{
        warn, Commands, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform, With,
    },
};
use bevy_rapier3d::prelude::ExternalImpulse;
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{Collider, CollisionGroups, Damping, ExternalForce, GravityScale, Sleeping},
};
use entity::{
    entity_data::{EntityData, WorldMode, WorldModes},
    examine::Examinable,
    health::HealthComponent,
    sensable::Sensable,
};
use gridmap::{
    can_reach_entity::can_reach_entity,
    events::Cell,
    get_spawn_position::entity_spawn_position_for_player,
    grid::{GridmapData, GridmapMain},
};
use humanoid::humanoid::{CharacterAnimationState, Humanoid};
use inventory_api::core::Inventory;
use inventory_item::item::InventoryItem;
use networking::server::{
    EntityUpdateData, EntityWorldType, InputDropCurrentItem, InputTakeOffItem, InputThrowItem,
    InputUseWorldItem, InputWearItem, ReliableServerMessage,
};
use rand::Rng;
use server_instance::core::HandleToEntity;

use super::net::{
    NetDropCurrentItem, NetPickupWorldItem, NetTakeOffItem, NetThrowItem, NetWearItem,
};
use pawn::pawn::{ControllerInput, Pawn, REACH_DISTANCE};
use physics::physics::{disable_rigidbody, enable_rigidbody, RigidBodyLinkTransform};
use sfx::{builder::sfx_builder, entity_update::SfxAutoDestroyTimers};
use sounds::{
    actions::{throw1_sfx::Throw1SfxBundle, throw2_sfx::Throw2SfxBundle},
    shared::sfx_auto_destroy,
};

/// Perform drop current item action.
#[cfg(feature = "server")]
pub(crate) fn drop_current_item(
    mut drop_current_item_events: EventReader<InputDropCurrentItem>,
    mut rigidbody_positions: Query<&mut Transform>,
    mut inventory_entities: Query<(&mut Inventory, &Sensable, &Pawn)>,
    mut inventory_items_query: Query<&mut InventoryItem>,
    health_query: Query<&HealthComponent>,
    cell_query: Query<&Cell>,
    mut q: Query<(
        &mut WorldMode,
        &mut Sleeping,
        &mut GravityScale,
        &mut ExternalForce,
        &mut RigidBodyLinkTransform,
        &Children,
        &mut Damping,
    )>,
    colliders: Query<&Parent, With<Collider>>,

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
                &colliders,
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
            mut damping_component,
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
            &mut damping_component,
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

/// Register approved pickup action and fire as new event.
#[cfg(feature = "server")]
pub(crate) fn pickup_world_item_action(
    building_action_data: Res<BuildingActions>,
    mut use_world_item_events: EventWriter<InputUseWorldItem>,
    action_requests: Res<ActionRequests>,
) {
    for building in building_action_data.list.iter() {
        let building_action_id;
        match action_requests.list.get(&building.incremented_i) {
            Some(action_request) => {
                building_action_id = action_request.get_id().clone();
            }
            None => {
                continue;
            }
        }

        for action in building.actions.iter() {
            if action.is_approved()
                && action.data.id == "actions::inventory/pickup"
                && action.data.id == building_action_id
            {
                use_world_item_events.send(InputUseWorldItem {
                    using_entity: building.action_taker,
                    used_entity: building.target_entity_option.unwrap(),
                });
            }
        }
    }
}

/// Perform items picking up action.
#[cfg(feature = "server")]
pub(crate) fn pickup_world_item(
    mut use_world_item_events: EventReader<InputUseWorldItem>,
    mut inventory_entities: Query<&mut Inventory>,
    mut inventory_items_query: Query<&mut InventoryItem>,
    health_query: Query<&HealthComponent>,
    mut q: Query<(
        &mut WorldMode,
        &mut Sleeping,
        &Children,
        &mut ExternalForce,
        &EntityData,
        &mut GravityScale,
        &mut Damping,
    )>,
    mut collision_groups: Query<&mut CollisionGroups>,
    colliders: Query<&Parent, With<Collider>>,
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
        let pickuper_components_option = inventory_entities.get_mut(event.using_entity);
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

        let pickupable_entity = event.used_entity;

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
            .get(event.using_entity)
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
            &event.using_entity,
            &health_query,
            &cell_query,
            &gridmap_main,
            &gridmap_data,
            false,
            &colliders,
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
        let children = pickupable_entities_components.2;
        let mut _pickupable_rigid_body_forces = pickupable_entities_components.3;
        let mut pickupable_rigid_body_gravity = pickupable_entities_components.5;
        let mut damping_component = pickupable_entities_components.6;

        let pickupable_entity_data = pickupable_entities_components.4;

        let mut collision_entity_option = None;

        for child in children.iter() {
            match collision_groups.get(*child) {
                Ok(_col) => {
                    collision_entity_option = Some(child);
                    break;
                }
                Err(_rr) => {}
            }
        }

        let mut collision_group;

        match collision_entity_option {
            Some(ent) => collision_group = collision_groups.get_mut(*ent).unwrap(),
            None => {
                warn!("couldnt find collider child");
                break;
            }
        }

        disable_rigidbody(
            &mut pickupable_rigid_body_activation,
            &mut collision_group,
            &mut pickupable_rigid_body_gravity,
            &mut commands,
            pickupable_entity,
            &mut damping_component,
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

        pickupable_inventory_item_component.in_inventory_of_entity = Some(event.using_entity);
        pickup_slot.slot_item = Some(pickupable_entity);
        pickupable_world_mode.mode = WorldModes::Held;

        commands
            .entity(pickupable_entity)
            .insert(RigidBodyLinkTransform {
                follow_entity: event.using_entity,
                ..Default::default()
            });

        match handle_to_entity.inv_map.get(&event.using_entity) {
            Some(handle) => {
                net_pickup_world_item.send(NetPickupWorldItem {
                    handle: *handle,
                    message: ReliableServerMessage::PickedUpItem(
                        pickupable_entity_data.entity_name.clone(),
                        event.used_entity.to_bits(),
                        pickup_slot.slot_name.clone(),
                    ),
                });
            }
            None => {}
        }
    }
}

/// Perform taking off/unequiping action.
#[cfg(feature = "server")]
pub(crate) fn take_off_item(
    mut take_off_item_events: EventReader<InputTakeOffItem>,
    mut inventory_entities: Query<&mut Inventory>,
    mut pickupable_entities: Query<(&InventoryItem, &mut WorldMode, &EntityData)>,
    mut net_takeoff_item: EventWriter<NetTakeOffItem>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in take_off_item_events.iter() {
        let carrier_components_option = inventory_entities.get_mut(event.entity);
        let carrier_components;

        match carrier_components_option {
            Ok(components) => {
                carrier_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut carrier_inventory = carrier_components;
        let pickup_slot_name = carrier_inventory.active_slot.clone();

        let mut pickup_slot_option = None;
        let mut take_off_slot_option = None;

        for slot in carrier_inventory.slots.iter_mut() {
            if slot.slot_name == pickup_slot_name {
                pickup_slot_option = Some(slot);
            } else if slot.slot_name == event.slot_name {
                take_off_slot_option = Some(slot);
            }
        }

        let pickup_slot = pickup_slot_option.unwrap();
        let takeoff_slot = take_off_slot_option.unwrap();

        let takeoff_entity;

        match takeoff_slot.slot_item {
            Some(item) => {
                takeoff_entity = item;
            }
            None => {
                continue;
            }
        }

        match pickup_slot.slot_item {
            Some(_) => {
                continue;
            }
            None => {}
        }

        let takeoff_components_option = pickupable_entities.get_mut(takeoff_entity);
        let takeoff_components;

        match takeoff_components_option {
            Ok(components) => {
                takeoff_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut takeoff_worldmode = takeoff_components.1;

        pickup_slot.slot_item = Some(takeoff_entity);
        takeoff_slot.slot_item = None;
        takeoff_worldmode.mode = WorldModes::Held;

        match handle_to_entity.inv_map.get(&event.entity) {
            Some(handle) => {
                net_takeoff_item.send(NetTakeOffItem {
                    handle: *handle,
                    message: ReliableServerMessage::EquippedWornItem(
                        takeoff_components.2.entity_name.clone(),
                        takeoff_entity.to_bits(),
                        takeoff_slot.slot_name.clone(),
                    ),
                });
            }
            None => {}
        }
    }
}

/// Perform throwing item action.
#[cfg(feature = "server")]
pub(crate) fn throw_item(
    mut throw_item_events: EventReader<InputThrowItem>,
    mut rigidbody_positions: Query<&mut Transform>,
    examinables: Query<&Examinable>,
    mut inventory_entities: Query<(
        &mut Inventory,
        &Sensable,
        &mut Pawn,
        &Humanoid,
        &mut ControllerInput,
        Option<&ZeroGravity>,
        Entity,
    )>,
    mut pickupable_entities: Query<(
        Entity,
        &mut InventoryItem,
        &mut WorldMode,
        &mut Sleeping,
        &Children,
        &mut ExternalForce,
        &mut RigidBodyLinkTransform,
        &mut GravityScale,
        &mut Damping,
    )>,
    mut external_impulses: Query<&mut ExternalImpulse>,
    mut collision_groups: Query<&mut CollisionGroups>,
    mut commands: Commands,
    mut net_throw_item: EventWriter<NetThrowItem>,
    gridmap_main: Res<GridmapMain>,
    handle_to_entity: Res<HandleToEntity>,
    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
) {
    for event in throw_item_events.iter() {
        let pickuper_components_option = inventory_entities.get_mut(event.entity);
        let mut pickuper_components;

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

        let pickuper_transform;

        match rigidbody_positions.get_component::<Transform>(event.entity) {
            Ok(t) => {
                pickuper_transform = t.clone();
            }
            Err(_rr) => {
                warn!("!");
                continue;
            }
        }

        let (
            pickupable_entity,
            mut inventory_item_component,
            mut pickupable_world_mode_component,
            mut pickupable_rigidbody_activation,
            children,
            mut _external_force_component,
            mut pickupable_rigidbody_link_transform_component,
            mut gravity_component,
            mut damping_component,
        ) = pickupable_entities.get_mut(pickupable_entity)
        .expect("drop_current_item.rs couldnt find pickupable_components of pickupable_entity from query.");

        let item_examinable_component = examinables.get(pickupable_entity).unwrap();
        let character_examinable_component = examinables.get(event.entity).unwrap();

        drop_slot.slot_item = None;
        pickupable_world_mode_component.mode = WorldModes::Physics;
        inventory_item_component.in_inventory_of_entity = None;

        let mut collider_child_option = None;

        for child in children.iter() {
            match collision_groups.get(*child) {
                Ok(_l) => collider_child_option = Some(child),
                Err(_rr) => {}
            }
        }

        let mut collider_groups;

        match collider_child_option {
            Some(ent) => {
                collider_groups = collision_groups.get_mut(*ent).unwrap();
            }
            None => {
                warn!("Couldnt find collider child.");
                break;
            }
        }

        enable_rigidbody(
            &mut pickupable_rigidbody_activation,
            &mut collider_groups,
            &mut gravity_component,
            &mut commands,
            pickupable_entity,
            &mut damping_component,
        );

        pickupable_rigidbody_link_transform_component.active = false;

        commands
            .entity(pickupable_entity)
            .remove::<RigidBodyLinkTransform>();

        let new_transform;

        match rigidbody_positions.get_component_mut::<Transform>(pickupable_entity) {
            Ok(mut position) => {
                let mut new_pickupable_transform = pickuper_transform;

                let results = entity_spawn_position_for_player(
                    new_pickupable_transform,
                    None,
                    Some(event.angle),
                    &gridmap_main,
                );

                match pickuper_components.3.current_lower_animation_state {
                    CharacterAnimationState::Idle => {
                        if !pickuper_components.3.combat_mode {
                            pickuper_components.4.pending_direction = Some(results.1);
                        }
                    }
                    _ => (),
                }

                new_pickupable_transform.translation = results.0.translation;
                new_pickupable_transform.scale = results.0.scale;
                new_pickupable_transform.rotation = results.0.rotation;

                new_pickupable_transform.translation.y = 1.5;

                new_pickupable_transform.rotation =
                    inventory_item_component.drop_transform.rotation;

                new_transform = new_pickupable_transform.clone();

                position.translation = new_pickupable_transform.translation;
                position.rotation = new_pickupable_transform.rotation;
                position.scale = new_pickupable_transform.scale;
            }
            Err(_rr) => {
                warn!("Couldn't find RigidBodyPosition of entity that is dropped.");
                continue;
            }
        }

        let thrower_vec3: Vec3;

        match rigidbody_positions.get(pickuper_components.6) {
            Ok(pos) => {
                thrower_vec3 = pos.translation.into();
            }
            Err(_rr) => {
                warn!("Couldn't find rigidbodyposition of thrower!");
                continue;
            }
        }

        let mut impulse = (event.position - new_transform.translation).normalize() * 0.025;
        let mut impulse_absolute: Vec3 = (event.position - thrower_vec3).normalize() * 0.025;

        let mut distance = event.position.distance(new_transform.translation);
        let mut distance_absolute = event.position.distance(thrower_vec3);

        if distance > 10. {
            distance = 10.
        }
        if distance_absolute > 10. {
            distance_absolute = 10.
        }

        impulse.y = 0.;
        impulse_absolute.y = 0.;

        impulse *= distance;
        impulse_absolute *= distance_absolute;

        impulse *= inventory_item_component.throw_force_factor;
        impulse_absolute *= inventory_item_component.throw_force_factor;

        match external_impulses.get_mut(pickupable_entity) {
            Ok(mut external_force_component) => {
                external_force_component.impulse = impulse;
            }
            Err(_rr) => {}
        }

        if pickuper_components.5.is_some() {
            // Thrower has zerogravity, apply inverse impulse energy.
            match external_impulses.get_mut(pickuper_components.6) {
                Ok(mut s) => {
                    s.impulse = -impulse_absolute;
                }
                Err(_rr) => {}
            }
        }

        match &drop_slot.slot_attachment {
            Some(attachment_path) => {
                // Create detachItem entityUpdate and send it to send_entity_update.rs

                let mut root_entity_update = HashMap::new();

                let mut entity_update = HashMap::new();

                entity_update.insert(
                    "detachItem".to_string(),
                    EntityUpdateData::AttachedItem(
                        pickupable_entity.to_bits(),
                        new_transform.translation,
                        new_transform.rotation,
                        new_transform.scale,
                    ),
                );

                root_entity_update.insert(attachment_path.to_string(), entity_update);

                for entity_id in pickuper_components.1.sensed_by.iter() {
                    let handle_option = handle_to_entity.inv_map.get(&entity_id);

                    match handle_option {
                        Some(handle) => {
                            net_throw_item.send(NetThrowItem {
                                handle: *handle,
                                message: ReliableServerMessage::EntityUpdate(
                                    entity_id.to_bits(),
                                    root_entity_update.clone(),
                                    false,
                                    EntityWorldType::Main,
                                ),
                            });

                            net_throw_item.send(NetThrowItem {
                                handle: *handle,
                                message: ReliableServerMessage::ChatMessage(
                                    character_examinable_component.name.get_name().to_owned()
                                        + " throws "
                                        + &item_examinable_component.name.get_a_name()
                                        + "!",
                                ),
                            });
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        let mut rng = rand::thread_rng();
        let random_pick: i32 = rng.gen_range(0..2);

        let sfx_entity;

        if random_pick == 0 {
            sfx_entity = sfx_builder(&mut commands, new_transform, Box::new(Throw1SfxBundle::new));
        } else {
            sfx_entity = sfx_builder(&mut commands, new_transform, Box::new(Throw2SfxBundle::new));
        }

        sfx_auto_destroy(sfx_entity, &mut sfx_auto_destroy_timers);

        match handle_to_entity.inv_map.get(&event.entity) {
            Some(handle) => {
                // Send UI/Control update to owning client.
                net_throw_item.send(NetThrowItem {
                    handle: *handle,
                    message: ReliableServerMessage::DropItem(drop_slot.slot_name.clone()),
                });
            }
            None => {}
        }
    }
}

/// Perform wearing item action.
#[cfg(feature = "server")]
pub(crate) fn wear_item(
    mut wear_item_events: EventReader<InputWearItem>,
    mut inventory_entities: Query<&mut Inventory>,
    mut wearable_entities: Query<(&InventoryItem, &mut WorldMode, &EntityData)>,
    mut net_wear_item: EventWriter<NetWearItem>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in wear_item_events.iter() {
        let wearer_components_option = inventory_entities.get_mut(event.wearer_entity);
        let wearer_components;

        match wearer_components_option {
            Ok(components) => {
                wearer_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut wearer_inventory = wearer_components;

        let pickup_slot_name = wearer_inventory.active_slot.clone();

        let mut pickup_slot_option = None;
        let mut wear_slot_option = None;

        for slot in wearer_inventory.slots.iter_mut() {
            if slot.slot_name == pickup_slot_name {
                pickup_slot_option = Some(slot);
            } else if slot.slot_name == event.wear_slot {
                wear_slot_option = Some(slot);
            }
        }

        let pickup_slot = pickup_slot_option.unwrap();

        let wear_slot;

        match wear_slot_option {
            Some(slot) => {
                wear_slot = slot;
            }
            None => {
                continue;
            }
        }

        let wearable_entity;

        match pickup_slot.slot_item {
            Some(item) => {
                wearable_entity = item;
            }
            None => {
                continue;
            }
        }

        let wearable_components_option = wearable_entities.get_mut(wearable_entity);
        let mut wearable_components;

        match wearable_components_option {
            Ok(wearable) => {
                wearable_components = wearable;
            }
            Err(_rr) => {
                continue;
            }
        }

        let wearable_wearable = wearable_components.0;

        match wear_slot.slot_item {
            Some(_) => {
                continue;
            }
            None => {}
        }

        if wearable_wearable.slot_type != wear_slot.slot_type {
            continue;
        }

        pickup_slot.slot_item = None;
        wear_slot.slot_item = Some(wearable_entity);
        wearable_components.1.mode = WorldModes::Worn;

        match handle_to_entity.inv_map.get(&event.wearer_entity) {
            Some(handle) => {
                net_wear_item.send(NetWearItem {
                    handle: *handle,
                    message: ReliableServerMessage::PickedUpItem(
                        wearable_components.2.entity_name.clone(),
                        wearable_entity.to_bits(),
                        wear_slot.slot_name.clone(),
                    ),
                });
            }
            None => {}
        }
    }
}
