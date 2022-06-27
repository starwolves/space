use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    system::{Commands, ResMut},
};
use bevy_transform::components::Transform;

use crate::core::entity::{
    resources::{EntityDataResource, ShowcaseData, SpawnData},
    spawn::DefaultSpawnEvent,
};

use super::raw_entity::RawEntity;

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
            return_entity = Some(commands.spawn().id());

            default_spawner.send(DefaultSpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: Transform::identity(),
                    correct_transform: false,
                    holder_entity_option: Some(holder_entity),
                    default_map_spawn: false,
                    raw_entity_option: None,
                    showcase_data_option: showcase_handle_option,
                    entity_name,
                    entity: return_entity.unwrap(),
                    held_entity_option: return_entity,
                },
            });
        }
        None => {
            return_entity = None;
        }
    }

    return_entity
}
