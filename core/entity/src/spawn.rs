use std::collections::HashMap;
pub struct BaseEntityBundle {
    pub default_transform: Transform,
    pub examinable: Examinable,
    pub entity_name: String,
    pub health: Health,
    pub entity_group: EntityGroup,
    pub default_map_spawn: bool,
}

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

pub struct BaseEntityData {
    pub entity_type: String,
    pub examinable: Examinable,
    pub sensable: Sensable,
    pub health: Health,
    pub is_item_in_storage: bool,
    pub entity_group: EntityGroup,
    pub default_map_spawn: bool,
    pub showcase_handle_option: Option<ShowcaseData>,
}

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

pub fn base_entity_builder(commands: &mut Commands, data: BaseEntityData, entity: Entity) {
    let mut builder = commands.entity(entity);
    builder.insert_bundle((
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
            builder.insert_bundle((
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

pub trait BaseEntitySummonable<Y> {
    fn get_bundle(&self, spawn_data: &SpawnData, entity_data_option: Y) -> BaseEntityBundle;
}

pub fn summon_base_entity<T: BaseEntitySummonable<NoData> + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
    mut net_showcase: EventWriter<NetShowcase>,
) {
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
                net_showcase.send(NetShowcase {
                    handle: showcase_data.handle,
                    message: ReliableServerMessage::LoadEntity(
                        "entity".to_string(),
                        base_entity_bundle.entity_name,
                        HashMap::new(),
                        spawn_event.spawn_data.entity.to_bits(),
                        true,
                        "main".to_string(),
                        ENTITY_SPAWN_PARENT.to_string(),
                        false,
                    ),
                });
            }
            None => {}
        }
    }
}

use api::{
    console_commands::ConsoleCommandVariantValues,
    data::{EntityDataResource, NoData, Showcase, ShowcaseData},
    entity_updates::{EntityData, EntityGroup, EntityUpdates},
    examinable::Examinable,
    health::{Health, HealthComponent},
    network::ReliableServerMessage,
    sensable::Sensable,
};
use bevy::prelude::{warn, Commands, Entity, EventReader, EventWriter, ResMut, Transform};
use serde::Deserialize;

use crate::entity_data::{CachedBroadcastTransform, RawEntity, ENTITY_SPAWN_PARENT};

use super::entity_data::{DefaultMapEntity, NetShowcase};

#[derive(Deserialize)]
pub struct ExportProperty {
    pub value_type: i64,
    pub value: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct ExportDataRaw {
    pub properties: Vec<ExportProperty>,
}

pub struct ExportData {
    pub properties: HashMap<String, ConsoleCommandVariantValues>,
}

impl ExportData {
    pub fn new(raw: ExportDataRaw) -> ExportData {
        let mut hashmap = HashMap::new();
        for property in raw.properties {
            let v;
            if property.value_type == 4 {
                v = ConsoleCommandVariantValues::String(property.value)
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

#[derive(Clone)]
pub struct SpawnData {
    pub entity_transform: Transform,
    pub correct_transform: bool,
    pub holder_entity_option: Option<Entity>,
    pub held_entity_option: Option<Entity>,
    pub default_map_spawn: bool,
    pub raw_entity_option: Option<RawEntity>,
    pub showcase_data_option: Option<ShowcaseData>,
    pub entity_name: String,
    pub entity: Entity,
}
impl Default for SpawnData {
    fn default() -> Self {
        Self {
            entity_transform: Transform::identity(),
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
pub struct DefaultSpawnEvent {
    pub spawn_data: SpawnData,
}
pub struct SpawnEvent<T> {
    pub spawn_data: SpawnData,
    pub summoner: T,
}
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
            return_entity = Some(commands.spawn().id());

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
