use bevy::prelude::{Commands, Entity, EventReader};
use bevy::prelude::{EventWriter, Query, Res};
use bevy_xpbd_3d::prelude::RigidBody;
use entity::net::{EntityServerMessage, LoadEntity};
use entity::spawn::{EntityBuildData, SpawnEntity};
use entity::spawning_events::SpawnClientEntity;

use crate::server::combat::{MeleeCombat, ProjectileCombat};

use super::item::InventoryItem;

/// Inventory item bundle.

pub struct InventoryItemBundle {
    pub inventory_item: InventoryItem,
    pub melee_combat: MeleeCombat,
    pub projectile_combat_option: Option<ProjectileCombat>,
}

/// Inventory item builder data.

pub struct InventoryBuilderData {
    pub inventory_item: InventoryItem,
    pub holder_entity_option: Option<Entity>,
    pub melee_combat: MeleeCombat,
    pub projectile_option: Option<ProjectileCombat>,
}
use physics::physics::RigidBodyLinkTransform;

/// Build inventory item at building stage.

pub(crate) fn inventory_item_builder(
    commands: &mut Commands,
    entity: Entity,
    data: InventoryBuilderData,
) {
    let mut builder = commands.entity(entity);
    builder.insert((data.inventory_item, data.melee_combat));
    match data.holder_entity_option {
        Some(holder_entity) => {
            builder.insert(RigidBodyLinkTransform {
                follow_entity: holder_entity,
                ..Default::default()
            });
            match data.projectile_option {
                Some(c) => {
                    builder.insert(c);
                }
                None => {}
            }
        }
        None => {}
    }
}
/// Inventory item buildable.

pub trait InventoryItemBuilder: Send + Sync {
    fn get_bundle(&self, spawn_data: &EntityBuildData) -> InventoryItemBundle;
}

/// Inventory item spawn handler.

pub fn build_inventory_items<T: InventoryItemBuilder + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        let inventory_item_bundle = spawn_event.entity_type.get_bundle(&spawn_event.spawn_data);

        inventory_item_builder(
            &mut commands,
            spawn_event.spawn_data.entity,
            InventoryBuilderData {
                inventory_item: inventory_item_bundle.inventory_item,
                holder_entity_option: spawn_event.spawn_data.holder_entity_option,
                melee_combat: inventory_item_bundle.melee_combat,
                projectile_option: inventory_item_bundle.projectile_combat_option,
            },
        );
    }
}

use bevy::prelude::warn;
use bevy::prelude::Transform;
use entity::entity_data::personalise;
use entity::entity_data::WorldModes;

use entity::entity_types::EntityTypes;
use std::collections::HashMap;

use entity::entity_data::{EntityData, EntityUpdates, WorldMode};
use networking::server::{EntityUpdateData, OutgoingReliableServerMessage};
/// Load an entity in for the client. Does not only apply to inventory items or holders.
/// Belongs in crate/entity but cyclic issues.
pub(crate) fn spawn_entity_for_client(
    entity_query: Query<(
        &EntityData,
        &EntityUpdates,
        &Transform,
        Option<&RigidBody>,
        Option<&WorldMode>,
        Option<&InventoryItem>,
    )>,
    mut load_entity_events: EventReader<SpawnClientEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
) {
    for load_entity_event in load_entity_events.iter() {
        match entity_query.get(load_entity_event.entity) {
            Ok((
                entity_data,
                entity_update,
                transform,
                rigid_body_component_option,
                entity_world_mode_option,
                inventory_item_option,
            )) => {
                let mut is_interpolated = false;

                match rigid_body_component_option {
                    Some(rigid_body_component) => match rigid_body_component {
                        RigidBody::Dynamic => match entity_world_mode_option {
                            Some(entity_world_mode) => {
                                if matches!(entity_world_mode.mode, WorldModes::Held)
                                    || matches!(entity_world_mode.mode, WorldModes::Worn)
                                {
                                    is_interpolated = false;
                                } else {
                                    is_interpolated = true;
                                }
                            }
                            None => {
                                is_interpolated = false;
                            }
                        },
                        RigidBody::Static => {}
                        _ => {
                            warn!("Unexpected rigidbody type.");
                            continue;
                        }
                    },
                    None => {}
                }

                let mut hash_map;

                hash_map = entity_update.updates.clone();

                personalise(
                    &mut hash_map,
                    load_entity_event.loader_handle,
                    entity_update,
                );

                let transform_entity_update = EntityUpdateData::Transform(
                    transform.translation,
                    transform.rotation,
                    transform.scale,
                );

                match is_interpolated {
                    true => {
                        let mut transform_hash_map = HashMap::new();
                        transform_hash_map.insert("transform".to_string(), transform_entity_update);

                        hash_map.insert("rawTransform".to_string(), transform_hash_map);
                    }
                    false => {
                        let root_map_option = hash_map.get_mut(&".".to_string());

                        match root_map_option {
                            Some(root_map) => {
                                root_map.insert("transform".to_string(), transform_entity_update);
                            }
                            None => {
                                let mut transform_hash_map = HashMap::new();
                                transform_hash_map
                                    .insert("transform".to_string(), transform_entity_update);

                                hash_map.insert(".".to_string(), transform_hash_map);
                            }
                        }
                    }
                }

                let mut holder_option = None;

                match inventory_item_option {
                    Some(t) => holder_option = t.in_inventory_of_entity,
                    None => {}
                }

                server.send(OutgoingReliableServerMessage {
                    handle: load_entity_event.loader_handle,
                    message: EntityServerMessage::LoadEntity(LoadEntity {
                        type_id: *types
                            .netcode_types
                            .get(&entity_data.entity_type.get_identity())
                            .unwrap(),
                        entity: load_entity_event.entity,
                        translation: transform.translation,
                        scale: transform.scale,
                        rotation: transform.rotation,
                        holder_entity: holder_option,
                    }),
                });
            }
            Err(_) => {
                warn!("Couldnt find entity for load entity event.");
            }
        }
    }
}
