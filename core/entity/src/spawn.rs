use std::collections::HashMap;

use crate::showcase::{Showcase, ShowcaseData};
use bevy::prelude::{warn, Commands, Entity, EventReader, EventWriter, ResMut, Transform};
use networking::server::GodotVariantValues;
use serde::Deserialize;

use crate::{
    entity_data::{
        CachedBroadcastTransform, EntityData, EntityGroup, EntityUpdates, RawEntity,
        ENTITY_SPAWN_PARENT,
    },
    examine::Examinable,
    health::{Health, HealthComponent},
    meta::EntityDataResource,
    sensable::Sensable,
};

use super::entity_data::DefaultMapEntity;

/// A base bundle for the basis of entities. Should be used by almost all entities.
#[cfg(feature = "server")]
pub struct BaseEntityBundle {
    pub default_transform: Transform,
    pub examinable: Examinable,
    pub entity_name: String,
    pub health: Health,
    pub entity_group: EntityGroup,
    /// If this entity was spawned by default from map data.
    pub default_map_spawn: bool,
}

#[cfg(feature = "server")]
impl Default for BaseEntityBundle {
    fn default() -> Self {
        Self {
            entity_group: EntityGroup::None,
            default_transform: Transform::default(),
            examinable: Examinable::default(),
            entity_name: "".to_string(),
            health: Health::default(),
            default_map_spawn: false,
        }
    }
}
/// Base entity data.
#[cfg(feature = "server")]
pub struct BaseEntityData {
    /// Entity type ID.
    pub entity_type: String,
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

#[cfg(feature = "server")]
impl Default for BaseEntityData {
    fn default() -> Self {
        Self {
            entity_group: EntityGroup::None,
            entity_type: "".to_string(),
            examinable: Examinable::default(),
            sensable: Sensable::default(),
            health: Health::default(),
            is_item_in_storage: false,
            default_map_spawn: false,
            showcase_handle_option: None,
        }
    }
}

/// Spawn a base entity.
#[cfg(feature = "server")]
pub fn base_entity_builder(commands: &mut Commands, data: BaseEntityData, entity: Entity) {
    let mut builder = commands.entity(entity);
    builder.insert((
        EntityData {
            entity_class: "entity".to_string(),
            entity_name: data.entity_type.to_string(),
            entity_group: data.entity_group,
        },
        EntityUpdates::default(),
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
#[cfg(feature = "server")]
pub trait BaseEntitySummonable<Y> {
    fn get_bundle(&self, spawn_data: &SpawnData, entity_data_option: Y) -> BaseEntityBundle;
}
use bevy_renet::renet::RenetServer;

/// Spawn base entity components handler.
#[cfg(feature = "server")]
pub fn summon_base_entity<T: BaseEntitySummonable<NoData> + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
) {
    use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

    use crate::networking::EntityServerMessage;

    for spawn_event in spawn_events.iter() {
        let base_entity_bundle = spawn_event
            .summoner
            .get_bundle(&spawn_event.spawn_data, NoData);

        base_entity_builder(
            &mut commands,
            BaseEntityData {
                entity_type: base_entity_bundle.entity_name.clone(),
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
                server.send_message(
                    showcase_data.handle,
                    RENET_RELIABLE_CHANNEL_ID,
                    bincode::serialize(&EntityServerMessage::LoadEntity(
                        "entity".to_string(),
                        base_entity_bundle.entity_name,
                        HashMap::new(),
                        spawn_event.spawn_data.entity.to_bits(),
                        true,
                        "main".to_string(),
                        ENTITY_SPAWN_PARENT.to_string(),
                        false,
                    ))
                    .unwrap(),
                );
            }
            None => {}
        }
    }
}

/// A json property.
#[derive(Deserialize)]
#[cfg(feature = "server")]
pub struct ExportProperty {
    pub value_type: i64,
    pub value: String,
    pub key: String,
}

/// Additional json properties contained by a json entity.
#[derive(Deserialize)]
#[cfg(feature = "server")]
pub struct ExportDataRaw {
    pub properties: Vec<ExportProperty>,
}

/// All export json properties turned into a variant hashmap.
#[cfg(feature = "server")]
pub struct ExportData {
    pub properties: HashMap<String, GodotVariantValues>,
}

#[cfg(feature = "server")]
impl ExportData {
    pub fn new(raw: ExportDataRaw) -> ExportData {
        let mut hashmap = HashMap::new();
        for property in raw.properties {
            let v;
            if property.value_type == 4 {
                v = GodotVariantValues::String(property.value)
            } else {
                warn!("Entity from entities.json had unknown type!");
                continue;
            }
            hashmap.insert(property.key, v);
        }
        ExportData {
            properties: hashmap,
        }
    }
}

/// Spawn data used to spawn in entities.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct SpawnData {
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
    /// Entity as json.
    pub raw_entity_option: Option<RawEntity>,
    /// If the entity is spawned in a showcase find its data here.
    pub showcase_data_option: Option<ShowcaseData>,
    /// Entity type ID.
    pub entity_name: String,
    pub entity: Entity,
}
#[cfg(feature = "server")]
impl Default for SpawnData {
    fn default() -> Self {
        Self {
            entity_transform: Transform::IDENTITY,
            correct_transform: true,
            held_entity_option: None,
            holder_entity_option: None,
            default_map_spawn: false,
            raw_entity_option: None,
            showcase_data_option: None,
            entity_name: "".to_string(),
            entity: Entity::from_bits(0),
        }
    }
}
/// Default spawn event.
#[cfg(feature = "server")]
pub struct DefaultSpawnEvent {
    pub spawn_data: SpawnData,
}

/// Standard spawn event.
#[cfg(feature = "server")]
pub struct SpawnEvent<T> {
    pub spawn_data: SpawnData,
    pub summoner: T,
}
/// A function to spawn an entity.
#[cfg(feature = "server")]
pub fn spawn_entity(
    entity_name: String,
    transform: Transform,
    commands: &mut Commands,
    correct_transform: bool,
    entity_data: &ResMut<EntityDataResource>,
    held_data_option: Option<Entity>,
    raw_entity_option: Option<RawEntity>,
    showcase_handle_option: Option<ShowcaseData>,
    default_spawner: &mut EventWriter<DefaultSpawnEvent>,
) -> Option<Entity> {
    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(_id) => {
            let held;

            match held_data_option {
                Some(entity) => {
                    held = Some(entity);
                }
                None => {
                    held = None;
                }
            }
            return_entity = Some(commands.spawn(()).id());

            default_spawner.send(DefaultSpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: transform,
                    correct_transform,
                    holder_entity_option: held,
                    raw_entity_option: raw_entity_option,
                    showcase_data_option: showcase_handle_option,
                    entity_name,
                    entity: return_entity.unwrap(),

                    ..Default::default()
                },
            });
        }
        None => {
            return_entity = None;
        }
    };

    match return_entity {
        Some(_entity) => {
            //info!("{:?}",entity);
        }
        None => {}
    }

    return_entity
}
#[cfg(feature = "server")]
pub struct NoData;
