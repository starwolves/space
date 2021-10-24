use bevy::prelude::{Commands, Entity, EventWriter, ResMut, Transform};

use crate::space_core::{components::persistent_player_data::PersistentPlayerData, events::net::net_showcase::NetShowcase, resources::{entity_data_resource::EntityDataResource, used_names::UsedNames}};

pub fn spawn_entity(
    entity_name : String,
    transform : Transform,
    commands: &mut Commands,
    correct_transform : bool,
    used_names_option : Option<&mut ResMut<UsedNames>>,
    entity_data : &ResMut<EntityDataResource>,
    held_data_option : Option<(
        Entity,
        bool,
        Option<u32>,
        &mut Option<&mut EventWriter<NetShowcase>>,
    )>,
) -> Option<Entity> {

    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(entity_type_id) => {

            let entity_properties = entity_data.data.get(*entity_type_id).unwrap();

            if entity_name == "humanDummy" {
                let passed_inventory_setup = vec![
                    ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
                    ("helmet".to_string(), "helmetSecurity".to_string()),
                ];

                let persistent_player_data_component = PersistentPlayerData {
                    character_name: "".to_string(),
                    user_name: "unknownSpawnEntityAssigned".to_string()
                };
                return_entity = Some((*entity_properties.spawn_function)
                    (transform,
                    commands,
                    correct_transform,
                    Some((
                        &persistent_player_data_component,
                        None,
                        passed_inventory_setup,
                        false,
                        true,
                        Some(used_names_option.unwrap()),
                        None,
                        None,
                        &entity_data,
                    )),
                    held_data_option,
                ));
            } else {
                return_entity = Some((*entity_properties.spawn_function)(
                    transform,
                    commands,
                    correct_transform,
                    None,
                    held_data_option
                ));
            }

        },
        None => {
            return_entity = None;
        },
    };

    return_entity

}

pub fn spawn_held_entity(
    entity_name : String,
    commands: &mut Commands,
    holder_entity : Entity,
    showcase_instance : bool,
    showcase_handle_option : Option<u32>,
    net_showcase : &mut Option<&mut EventWriter<NetShowcase>>,
    entity_data : &ResMut<EntityDataResource>,
) -> Option<Entity> {

    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(entity_type_id) => {

            let entity_properties = entity_data.data.get(*entity_type_id).unwrap();

            return_entity = Some((*entity_properties.spawn_function)(
                Transform::identity(),
                commands,
                false,
                None,
                Some((holder_entity, showcase_instance, showcase_handle_option, net_showcase))
            ));

        },
        None => {
            return_entity = None;
        }
    }

    return_entity

}
