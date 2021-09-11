use bevy::prelude::{Commands, Entity, EventWriter, ResMut, Transform, warn};

use crate::space_core::{bundles::{helmet_security::HelmetSecurityBundle, human_male_pawn::HumanMalePawnBundle, jumpsuit_security::JumpsuitSecurityBundle}, components::persistent_player_data::PersistentPlayerData, events::net::net_showcase::NetShowcase, resources::used_names::UsedNames};

pub fn spawn_entity(
    entity_name : String,
    transform : Transform,
    commands: &mut Commands,
    correct_transform : bool,
    used_names : &mut ResMut<UsedNames>,
) -> Option<Entity> {

    let return_entity;

    if entity_name == "jumpsuitSecurity" {

        return_entity = Some(JumpsuitSecurityBundle::spawn(transform,commands, correct_transform));

    } else if entity_name == "helmetSecurity" {

        return_entity = Some(HelmetSecurityBundle::spawn(transform,commands, correct_transform));

    } else if entity_name == "humanDummy" {

        let passed_inventory_setup = vec![
            ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
            ("helmet".to_string(), "helmetSecurity".to_string()),
        ];

        let persistent_player_data_component = PersistentPlayerData {
            character_name: "".to_string(),
            ooc_name: "unknownSpawnEntityAssigned".to_string()
        };

        return_entity = Some(HumanMalePawnBundle::spawn( 
            transform,
            commands,
            &persistent_player_data_component,
            None,
            passed_inventory_setup,
            false,
            true,
            Some(used_names),
            None,
            true,
            None,
        ));


    } else {
        warn!("Attempted to spawn an unknown entity.");
        return_entity = None;
    }

    return_entity

}

pub fn spawn_held_entity(
    entity_name : String,
    commands: &mut Commands,
    holder_entity : Entity,
    showcase_instance : bool,
    showcase_handle_option : Option<u32>,
    net_showcase : &mut Option<&mut EventWriter<NetShowcase>>,
) -> Option<Entity> {

    let return_entity;

    if entity_name == "jumpsuitSecurity" {

        return_entity = Some(JumpsuitSecurityBundle::spawn_held(commands, holder_entity, showcase_instance, showcase_handle_option, net_showcase));

    } else if entity_name == "helmetSecurity" {

        return_entity = Some(HelmetSecurityBundle::spawn_held(commands, holder_entity, showcase_instance, showcase_handle_option, net_showcase));

    } else {
        warn!("Attempted to spawn an unknown entity.");
        return_entity = None;
    }

    return_entity

}
