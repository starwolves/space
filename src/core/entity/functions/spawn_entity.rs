use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    system::{Commands, ResMut},
};
use bevy_transform::components::Transform;

use crate::core::{
    entity::{
        resources::{EntityDataResource, PawnDesignation, ShowcaseData, SpawnData, SpawnPawnData},
        spawn::DefaultSpawnEvent,
    },
    networking::resources::ConsoleCommandVariantValues,
    pawn::components::PersistentPlayerData,
};

pub fn spawn_entity(
    entity_name: String,
    transform: Transform,
    commands: &mut Commands,
    correct_transform: bool,
    entity_data: &ResMut<EntityDataResource>,
    held_data_option: Option<Entity>,
    pawn_data_option: Option<(Vec<(String, String)>, PersistentPlayerData)>,
    properties: HashMap<String, ConsoleCommandVariantValues>,
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

            match pawn_data_option {
                Some(data) => {
                    let pawn = Some(SpawnPawnData {
                        data: (data.1, None, data.0, PawnDesignation::Dummy, None),
                    });
                    default_spawner.send(DefaultSpawnEvent {
                        spawn_data: SpawnData {
                            entity_transform: transform,
                            correct_transform,
                            pawn_data_option: pawn,
                            held_data_option: held,
                            default_map_spawn: false,
                            properties: properties,
                            showcase_data_option: showcase_handle_option,
                            entity_name,
                            entity: return_entity.unwrap(),
                        },
                    });
                }
                None => {
                    default_spawner.send(DefaultSpawnEvent {
                        spawn_data: SpawnData {
                            entity_transform: transform,
                            correct_transform,
                            pawn_data_option: None,
                            held_data_option: held,
                            default_map_spawn: false,
                            properties: properties,
                            showcase_data_option: showcase_handle_option,
                            entity_name,
                            entity: return_entity.unwrap(),
                        },
                    });
                }
            }
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

pub fn spawn_held_entity(
    entity_name: String,
    commands: &mut Commands,
    holder_entity: Entity,
    showcase_handle_option: Option<ShowcaseData>,
    entity_data: &ResMut<EntityDataResource>,
    default_spawner: &mut EventWriter<DefaultSpawnEvent>,
) -> Option<Entity> {
    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(_id) => {
            let map = HashMap::new();

            return_entity = Some(commands.spawn().id());

            default_spawner.send(DefaultSpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: Transform::identity(),
                    correct_transform: false,
                    pawn_data_option: None,
                    held_data_option: Some(holder_entity),
                    default_map_spawn: false,
                    properties: map,
                    showcase_data_option: showcase_handle_option,
                    entity_name,
                    entity: return_entity.unwrap(),
                },
            });
        }
        None => {
            return_entity = None;
        }
    }

    return_entity
}
