use bevy::prelude::{Commands, Entity, EventWriter, Transform, warn};

use crate::space_core::{bundles::{helmet_security::HelmetSecurityBundle, jumpsuit_security::JumpsuitSecurityBundle}, events::net::net_showcase::NetShowcase};

pub fn spawn_entity(
    entity_name : String,
    transform : Transform,
    commands: &mut Commands,
    correct_transform : bool,
) -> Entity {

    let return_entity;

    if entity_name == "jumpsuitSecurity" {

        return_entity = Some(JumpsuitSecurityBundle::spawn(transform,commands, correct_transform));

    } else if entity_name == "helmetSecurity" {

        return_entity = Some(HelmetSecurityBundle::spawn(transform,commands, correct_transform));

    } else {
        warn!("Attempted to spawn an unknown entity.");
        return_entity = None;
    }

    return_entity.unwrap()

}

pub fn spawn_held_entity(
    entity_name : String,
    commands: &mut Commands,
    holder_entity : Entity,
    showcase_instance : bool,
    showcase_handle_option : Option<u32>,
    net_showcase : &mut Option<&mut EventWriter<NetShowcase>>,
) -> Entity {

    let return_entity;

    if entity_name == "jumpsuitSecurity" {

        return_entity = Some(JumpsuitSecurityBundle::spawn_held(commands, holder_entity, showcase_instance, showcase_handle_option, net_showcase));

    } else if entity_name == "helmetSecurity" {

        return_entity = Some(HelmetSecurityBundle::spawn_held(commands, holder_entity, showcase_instance, showcase_handle_option, net_showcase));

    } else {
        warn!("Attempted to spawn an unknown entity.");
        return_entity = None;
    }

    return_entity.unwrap()

}
