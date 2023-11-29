use std::collections::HashMap;

use crate::{
    entity_data::BlankEntityType,
    entity_types::{BoxedEntityType, EntityType},
    net::{LoadEntity, PhysicsData},
    showcase::{Showcase, ShowcaseData},
};
use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, Event, EventReader, EventWriter, Resource, SystemSet, Transform},
};
use bevy_renet::renet::ClientId;
use serde::Deserialize;

use crate::{
    entity_data::{CachedBroadcastTransform, EntityData, EntityGroup},
    examine::Examinable,
    health::{Health, HealthComponent},
    sensable::Sensable,
};

use super::entity_data::DefaultMapEntity;

/// A base bundle for the basis of entities. Should be used by almost all entities.

pub struct BaseEntityBundle {
    pub default_transform: Transform,
    pub examinable: Examinable,
    pub entity_type: BoxedEntityType,
    pub health: Health,
    pub entity_group: EntityGroup,
    /// If this entity was spawned by default from map data.
    pub default_map_spawn: bool,
}

impl Default for BaseEntityBundle {
    fn default() -> Self {
        Self {
            default_transform: Transform::default(),
            examinable: Examinable::default(),
            entity_type: Box::<BlankEntityType>::new(EntityType::new()),
            health: Health::default(),
            entity_group: EntityGroup::default(),
            default_map_spawn: bool::default(),
        }
    }
}

/// Base entity data.

pub struct BaseEntityData {
    /// Entity type ID.
    pub entity_type: BoxedEntityType,
    pub examinable: Examinable,
    pub sensable: Sensable,
    pub health: Health,
    /// If item is spawned within another storage container.
    pub is_item_in_storage: bool,
    pub entity_group: EntityGroup,
    /// If this entity was spawned by default from map data.
    pub default_map_spawn: bool,
    /// If this entity is part of a showcase pass entity id.
    pub showcase_handle_option: Option<ShowcaseData>,
}

impl Default for BaseEntityData {
    fn default() -> Self {
        Self {
            examinable: Examinable::default(),
            entity_type: Box::<BlankEntityType>::new(EntityType::new()),
            health: Health::default(),
            entity_group: EntityGroup::default(),
            default_map_spawn: bool::default(),
            sensable: Sensable::default(),
            is_item_in_storage: bool::default(),
            showcase_handle_option: None,
        }
    }
}
/// Spawn a base entity.

pub fn base_entity_builder(commands: &mut Commands, data: BaseEntityData, entity: Entity) {
    let mut builder = commands.entity(entity);
    builder.insert((
        EntityData {
            entity_type: data.entity_type,
            entity_group: data.entity_group,
        },
        CachedBroadcastTransform::default(),
    ));

    match data.showcase_handle_option {
        Some(showcase_data) => {
            builder.insert(Showcase {
                handle: showcase_data.handle,
            });
        }
        None => {
            builder.insert((
                data.sensable,
                data.examinable,
                HealthComponent {
                    health: data.health,
                },
            ));
        }
    }

    match data.default_map_spawn {
        true => {
            builder.insert(DefaultMapEntity);
        }
        false => {}
    }
}

/// BaseEntity trait.

pub trait BaseEntityBuilder<Y>: Send + Sync {
    fn get_bundle(&self, spawn_data: &EntityBuildData, entity_data_option: Y) -> BaseEntityBundle;
}
use crate::init::RawEntityRon;
use networking::server::OutgoingReliableServerMessage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpawnItemSet {
    SpawnHeldItem,
    AddingComponent,
}

use bevy::prelude::Res;

use crate::entity_types::EntityTypes;
use crate::net::EntityServerMessage;
/// Spawn base entity components handler.

pub fn build_base_entities<T: BaseEntityBuilder<NoData> + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
) {
    for spawn_event in spawn_events.read() {
        let base_entity_bundle = spawn_event
            .entity_type
            .get_bundle(&spawn_event.spawn_data, NoData);

        let entity_type = base_entity_bundle.entity_type.get_identity();

        base_entity_builder(
            &mut commands,
            BaseEntityData {
                entity_type: base_entity_bundle.entity_type,
                examinable: base_entity_bundle.examinable,
                health: base_entity_bundle.health,
                entity_group: base_entity_bundle.entity_group,
                default_map_spawn: base_entity_bundle.default_map_spawn,
                is_item_in_storage: spawn_event.spawn_data.holder_entity_option.is_some(),
                ..Default::default()
            },
            spawn_event.spawn_data.entity,
        );

        match &spawn_event.spawn_data.showcase_data_option {
            Some(showcase_data) => {
                server.send(OutgoingReliableServerMessage {
                    handle: showcase_data.handle,
                    message: EntityServerMessage::LoadEntity(LoadEntity {
                        type_id: *types.netcode_types.get(&entity_type).unwrap(),
                        entity: spawn_event.spawn_data.entity,

                        physics_data: PhysicsData {
                            translation: spawn_event.spawn_data.entity_transform.translation,
                            scale: spawn_event.spawn_data.entity_transform.scale,
                            rotation: spawn_event.spawn_data.entity_transform.rotation,
                            velocity: Vec3::ZERO,
                            angular_velocity: Vec3::ZERO,
                        },
                        holder_entity: spawn_event.spawn_data.holder_entity_option,
                    }),
                });
            }
            None => {}
        }
    }
}

/// Additional ron properties contained by a raw ron entity.
#[derive(Deserialize)]

pub struct RonDataRaw {
    pub data: String,
    pub entity_type: String,
}

/// Spawn data used to spawn in entities.
#[derive(Clone)]

pub struct EntityBuildData {
    /// Transform of the to be spawned entity.
    pub entity_transform: Transform,
    /// Whether the transform (rotation) should be corrected.
    pub correct_transform: bool,
    /// If the entity is held by another entity in its inventory.
    pub holder_entity_option: Option<Entity>,
    /// If the entity is holding another entity.
    pub held_entity_option: Option<Entity>,
    /// If the spawn is part of the default map data.
    pub default_map_spawn: bool,
    /// Entity as ron.
    pub raw_entity_option: Option<RawEntityRon>,
    /// If the entity is spawned in a showcase find its data here.
    pub showcase_data_option: Option<ShowcaseData>,
    pub entity: Entity,
    /// For client to keep reference to server-side entity id.
    pub server_entity: Option<Entity>,
}

impl Default for EntityBuildData {
    fn default() -> Self {
        Self {
            entity_transform: Transform::IDENTITY,
            correct_transform: true,
            held_entity_option: None,
            holder_entity_option: None,
            default_map_spawn: false,
            raw_entity_option: None,
            showcase_data_option: None,
            entity: Entity::from_bits(0),
            server_entity: None,
        }
    }
}

/// Standard spawn event.
#[derive(Event)]
pub struct SpawnEntity<T> {
    pub spawn_data: EntityBuildData,
    pub entity_type: T,
}
/// A function to spawn an entity.

pub fn spawn_entity<T: EntityType + Send + Sync + 'static>(
    entity_type: T,
    transform: Transform,
    commands: &mut Commands,
    correct_transform: bool,
    held_data_option: Option<Entity>,
    raw_entity_option: Option<RawEntityRon>,
    showcase_handle_option: Option<ShowcaseData>,
    default_spawner: &mut EventWriter<SpawnEntity<T>>,
) -> Entity {
    let return_entity;

    let held;

    match held_data_option {
        Some(entity) => {
            held = Some(entity);
        }
        None => {
            held = None;
        }
    }
    return_entity = commands.spawn(()).id();
    default_spawner.send(SpawnEntity {
        spawn_data: EntityBuildData {
            entity_transform: transform,
            correct_transform,
            holder_entity_option: held,
            raw_entity_option: raw_entity_option,
            showcase_data_option: showcase_handle_option,
            entity: return_entity,

            ..Default::default()
        },
        entity_type,
    });

    return_entity
}

pub struct NoData;
#[derive(Resource, Default)]
pub struct ServerEntityClientEntity {
    // Server, Client.
    pub map: HashMap<Entity, Entity>,
}

#[derive(Resource, Default)]
/// Resource stores the server-side entity ID of the players pawn. Useful for the client to store.
pub struct PawnId {
    pub server: Option<Entity>,
    pub client: Option<Entity>,
}
#[derive(Resource, Default)]
pub struct PeerPawns {
    pub map: HashMap<ClientId, Entity>,
}
