use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    system::{Commands, ResMut},
};
use bevy_transform::components::Transform;

use crate::core::{
    entity::resources::{
        EntityDataResource, PawnDesignation, ShowcaseData, SpawnData, SpawnPawnData,
    },
    networking::resources::ConsoleCommandVariantValues,
    pawn::{components::PersistentPlayerData, resources::UsedNames},
};

pub fn spawn_entity<'a, 'b, 'c, 'd, 'w, 's>(
    entity_name: String,
    transform: Transform,
    commands: &mut Commands,
    correct_transform: bool,
    used_names_option: Option<&'a mut ResMut<'b, UsedNames>>,
    entity_data: &'a ResMut<'a, EntityDataResource>,
    held_data_option: Option<Entity>,
    pawn_data_option: Option<(Vec<(String, String)>, PersistentPlayerData)>,
    properties: HashMap<String, ConsoleCommandVariantValues>,
    mut showcase_handle_option: Option<ShowcaseData<'b, 'c, 'd>>,
) -> Option<Entity> {
    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(entity_type_id) => {
            let entity_properties = entity_data.data.get(*entity_type_id).unwrap();

            let held;

            match held_data_option {
                Some(entity) => {
                    held = Some(entity);
                }
                None => {
                    held = None;
                }
            }

            match pawn_data_option {
                Some(data) => {
                    let pawn = Some(SpawnPawnData {
                        data: (
                            &data.1,
                            None,
                            data.0,
                            PawnDesignation::Dummy,
                            Some(used_names_option.unwrap()),
                            None,
                            &entity_data,
                        ),
                    });
                    return_entity = Some((*entity_properties.spawn_function)(SpawnData {
                        entity_transform: transform,
                        commands,
                        correct_transform,
                        pawn_data_option: pawn,
                        held_data_option: held,
                        default_map_spawn: false,
                        properties: properties,
                        showcase_data_option: &mut showcase_handle_option,
                        entity_name,
                    }));
                }
                None => {
                    return_entity = Some((*entity_properties.spawn_function)(SpawnData {
                        entity_transform: transform,
                        commands,
                        correct_transform,
                        pawn_data_option: None,
                        held_data_option: held,
                        default_map_spawn: false,
                        properties: properties,
                        showcase_data_option: &mut showcase_handle_option,

                        entity_name,
                    }));
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

pub fn spawn_held_entity<'a, 'b, 'c, 'd>(
    entity_name: String,
    commands: &mut Commands,
    holder_entity: Entity,
    showcase_handle_option: &mut Option<ShowcaseData<'b, 'c, 'd>>,
    entity_data: &ResMut<EntityDataResource>,
) -> Option<Entity> {
    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(entity_type_id) => {
            let entity_properties = entity_data.data.get(*entity_type_id).unwrap();

            let map = HashMap::new();

            return_entity = Some((*entity_properties.spawn_function)(SpawnData {
                entity_transform: Transform::identity(),
                commands,
                correct_transform: false,
                pawn_data_option: None,
                held_data_option: Some(holder_entity),
                default_map_spawn: true,
                properties: map,
                showcase_data_option: showcase_handle_option,
                entity_name,
            }));
        }
        None => {
            return_entity = None;
        }
    }

    match return_entity {
        Some(_entity) => {
            //info!("(0) {:?}",entity);
        }
        None => {}
    }

    return_entity
}
