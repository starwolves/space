use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, Query, Res, ResMut},
};
use bevy_hierarchy::Children;
use bevy_log::warn;
use bevy_math::Vec3;

use bevy_rapier3d::prelude::{
    CollisionGroups, ExternalForce, ExternalImpulse, GravityScale, Sleeping,
};
use bevy_transform::prelude::Transform;
use rand::Rng;

use crate::{
    core::{
        atmospherics::components::ZeroGravity,
        connected_player::resources::HandleToEntity,
        examinable::components::Examinable,
        gridmap::resources::GridmapMain,
        humanoid::components::{CharacterAnimationState, Humanoid},
        inventory::{
            components::Inventory,
            events::{InputThrowItem, NetThrowItem},
        },
        inventory_item::components::InventoryItem,
        networking::resources::{EntityUpdateData, EntityWorldType, ReliableServerMessage},
        pawn::{
            components::{ControllerInput, Pawn},
            functions::entity_spawn_position_for_player::entity_spawn_position_for_player,
        },
        physics::components::{WorldMode, WorldModes},
        rigid_body::{components::RigidBodyLinkTransform, functions::enable_rigidbody},
        sensable::components::Sensable,
        sfx::{
            components::sfx_auto_destroy, functions::sfx_builder, resources::SfxAutoDestroyTimers,
        },
    },
    entities::sfx::actions::{throw1_sfx::Throw1SfxBundle, throw2_sfx::Throw2SfxBundle},
};

pub fn throw_item(
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
