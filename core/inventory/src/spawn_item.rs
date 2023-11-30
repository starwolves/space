use bevy::ecs::query::With;
use bevy::prelude::{Commands, Entity, EventReader};
use bevy::prelude::{EventWriter, Query, Res};
use bevy_xpbd_3d::components::{AngularVelocity, LinearVelocity};
use bevy_xpbd_3d::prelude::RigidBody;
use entity::net::{EntityServerMessage, LinkPeer, LoadEntity, PhysicsData};
use entity::spawn::{EntityBuildData, SpawnEntity};
use entity::spawning_events::SpawnClientEntity;
use physics::entity::{RigidBodies, SFRigidBody};
use physics::mirror_physics_transform::MirrorPhysicsTransform;

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
            builder.insert(MirrorPhysicsTransform {
                mirrored_physics_entity: holder_entity,
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
    for spawn_event in spawn_events.read() {
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

use bevy::log::warn;
use bevy::prelude::Transform;

use entity::entity_types::EntityTypes;

use entity::entity_data::EntityData;
use networking::server::{ConnectedPlayer, EntityUpdatesSerialized, OutgoingReliableServerMessage};
/// Load an entity in for the client. Does not only apply to inventory items or holders.
/// Belongs in crate/entity but cyclic issues.
pub(crate) fn spawn_entity_for_client(
    entity_query: Query<(
        &EntityData,
        &Transform,
        Option<&RigidBody>,
        Option<&InventoryItem>,
        Option<&ConnectedPlayer>,
    )>,
    mut load_entity_events: EventReader<SpawnClientEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
    rigidbodies: Res<RigidBodies>,
    rigid_query: Query<(&LinearVelocity, &AngularVelocity), With<SFRigidBody>>,
    serialized_updates: Res<EntityUpdatesSerialized>,
) {
    for load_entity_event in load_entity_events.read() {
        match entity_query.get(load_entity_event.entity) {
            Ok((
                entity_data,
                transform,
                rigid_body_component_option,
                inventory_item_option,
                connected_option,
            )) => {
                let mut linear_velocity = LinearVelocity::default();
                let mut angular_velocity = AngularVelocity::default();

                match rigid_body_component_option {
                    Some(_) => match rigidbodies.get_entity_rigidbody(&load_entity_event.entity) {
                        Some(rb_entity) => match rigid_query.get(*rb_entity) {
                            Ok((linear_velocity2, angular_velocity2)) => {
                                linear_velocity = *linear_velocity2;
                                angular_velocity = *angular_velocity2;
                            }
                            Err(_) => {
                                warn!("Couldnt find rb_entity");
                                continue;
                            }
                        },
                        None => {
                            warn!("Couldnt find get_entity_rigidbody().");
                            continue;
                        }
                    },
                    None => {}
                }

                let mut holder_option = None;

                match inventory_item_option {
                    Some(t) => holder_option = t.in_inventory_of_entity,
                    None => {}
                }
                let mut reliable = vec![];
                match serialized_updates.reliable.get(&load_entity_event.entity) {
                    Some(a) => {
                        reliable = a.clone();
                    }
                    None => {}
                }
                let mut unreliable = vec![];
                match serialized_updates.unreliable.get(&load_entity_event.entity) {
                    Some(a) => {
                        unreliable = a.clone();
                    }
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
                        physics_data: PhysicsData {
                            translation: transform.translation,
                            scale: transform.scale,
                            rotation: transform.rotation,
                            velocity: *linear_velocity,
                            angular_velocity: *angular_velocity,
                        },
                        holder_entity: holder_option,
                        entity_updates_reliable: reliable,
                        entity_updates_unreliable: unreliable,
                    }),
                });

                match connected_option {
                    Some(h) => {
                        server.send(OutgoingReliableServerMessage {
                            handle: load_entity_event.loader_handle,
                            message: EntityServerMessage::LinkPeer(LinkPeer {
                                handle: h.handle.raw() as u16,
                                server_entity: load_entity_event.entity,
                            }),
                        });
                    }
                    None => {}
                }
            }
            Err(_) => {
                warn!("Couldnt find entity for load entity event.");
            }
        }
    }
}
