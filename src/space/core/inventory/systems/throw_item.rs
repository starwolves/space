use std::collections::HashMap;

use bevy::{
    math::Vec3,
    prelude::{warn, Commands, Entity, EventReader, EventWriter, Query, Res, ResMut},
};
use bevy_rapier3d::prelude::{
    ColliderFlagsComponent, RigidBodyActivationComponent, RigidBodyForcesComponent,
    RigidBodyMassPropsComponent, RigidBodyPositionComponent, RigidBodyVelocityComponent,
};
use rand::Rng;

use crate::space::{
    core::{
        atmospherics::components::ZeroGravity,
        entity::{
            components::{Examinable, Sensable},
            functions::{
                isometry_to_transform::isometry_to_transform,
                transform_to_isometry::transform_to_isometry,
            },
        },
        gridmap::resources::GridmapMain,
        inventory::{
            components::Inventory,
            events::{InputThrowItem, NetThrowItem},
        },
        inventory_item::components::InventoryItem,
        networking::resources::{EntityUpdateData, EntityWorldType, ReliableServerMessage},
        pawn::{
            components::{Pawn, PlayerInput, StandardCharacter},
            functions::entity_spawn_position_for_player::entity_spawn_position_for_player,
            resources::HandleToEntity,
        },
        physics::components::{WorldMode, WorldModes},
        rigid_body::{components::RigidBodyLinkTransform, functions::enable_rigidbody},
        sfx::{components::sfx_auto_destroy, resources::SfxAutoDestroyTimers},
    },
    entities::sfx::actions::{throw1_sfx::Throw1SfxBundle, throw2_sfx::Throw2SfxBundle},
};

pub fn throw_item(
    mut throw_item_events: EventReader<InputThrowItem>,
    mut rigidbody_positions: Query<&mut RigidBodyPositionComponent>,
    examinables: Query<&Examinable>,
    mut inventory_entities: Query<(
        &mut Inventory,
        &Sensable,
        &mut Pawn,
        &StandardCharacter,
        &mut PlayerInput,
        Option<&ZeroGravity>,
        Entity,
    )>,
    mut pickupable_entities: Query<(
        Entity,
        &mut InventoryItem,
        &mut WorldMode,
        &mut RigidBodyActivationComponent,
        &mut ColliderFlagsComponent,
        &mut RigidBodyForcesComponent,
        &mut RigidBodyLinkTransform,
        &RigidBodyMassPropsComponent,
    )>,
    mut rigidbody_velocites: Query<&mut RigidBodyVelocityComponent>,
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

        let (
            pickupable_entity,
            mut inventory_item_component,
            mut pickupable_world_mode_component,
            mut pickupable_rigidbody_activation,
            mut pickupable_rigidbody_collider_flags,
            mut pickupable_rigidbody_forces,
            mut pickupable_rigidbody_link_transform_component,
            pickupable_rigidbody_props,
        ) = pickupable_entities.get_mut(pickupable_entity)
        .expect("drop_current_item.rs couldnt find pickupable_components of pickupable_entity from query.");

        let item_examinable_component = examinables.get(pickupable_entity).unwrap();
        let character_examinable_component = examinables.get(event.entity).unwrap();

        drop_slot.slot_item = None;
        pickupable_world_mode_component.mode = WorldModes::Physics;
        inventory_item_component.in_inventory_of_entity = None;

        enable_rigidbody(
            &mut pickupable_rigidbody_activation,
            &mut pickupable_rigidbody_collider_flags,
            &mut pickupable_rigidbody_forces,
            &mut commands,
            pickupable_entity,
        );

        pickupable_rigidbody_link_transform_component.active = false;

        commands
            .entity(pickupable_entity)
            .remove::<RigidBodyLinkTransform>();

        let new_transform;

        match rigidbody_positions.get_component_mut::<RigidBodyPositionComponent>(pickupable_entity)
        {
            Ok(mut position) => {
                let mut new_pickupable_transform = isometry_to_transform(position.position);

                let results = entity_spawn_position_for_player(
                    new_pickupable_transform,
                    None,
                    Some(event.angle),
                    &gridmap_main,
                );

                match pickuper_components.3.current_lower_animation_state {
                    crate::space::core::pawn::components::CharacterAnimationState::Idle => {
                        if !pickuper_components.3.combat_mode {
                            pickuper_components.4.pending_direction = Some(results.1);
                        }
                    }
                    _ => (),
                }

                new_pickupable_transform = results.0;

                new_pickupable_transform.translation.y = 1.5;

                new_pickupable_transform.rotation =
                    inventory_item_component.drop_transform.rotation;

                new_transform = new_pickupable_transform.clone();

                position.position = transform_to_isometry(new_pickupable_transform);
            }
            Err(_rr) => {
                warn!("Couldn't find RigidBodyPosition of entity that is dropped.");
                continue;
            }
        }

        let thrower_vec3: Vec3;

        match rigidbody_positions.get(pickuper_components.6) {
            Ok(pos) => {
                thrower_vec3 = pos.position.translation.into();
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

        match rigidbody_velocites.get_mut(pickupable_entity) {
            Ok(mut pickupable_rigidbody_velocity) => {
                pickupable_rigidbody_velocity
                    .apply_impulse(pickupable_rigidbody_props, impulse.into());
            }
            Err(_rr) => {}
        }

        if pickuper_components.5.is_some() {
            // Thrower has zerogravity, apply inverse impulse energy.
            match rigidbody_velocites.get_mut(pickuper_components.6) {
                Ok(mut s) => {
                    s.apply_impulse(pickupable_rigidbody_props, (-impulse_absolute).into());
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

        let throw;

        if random_pick == 0 {
            throw = Throw1SfxBundle::new(new_transform);
        } else {
            throw = Throw2SfxBundle::new(new_transform);
        }

        let sfx_entity = commands.spawn().insert_bundle(throw).id();
        sfx_auto_destroy(sfx_entity, &mut sfx_auto_destroy_timers);

        // Send UI/Control update to owning client.
        net_throw_item.send(NetThrowItem {
            handle: event.handle,
            message: ReliableServerMessage::DropItem(drop_slot.slot_name.clone()),
        });
    }
}
