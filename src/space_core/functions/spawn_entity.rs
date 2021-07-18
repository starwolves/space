use bevy::prelude::{Commands, Entity, Transform, warn};

use crate::space_core::bundles::{helmet_security::HelmetSecurityBundle, jumpsuit_security::JumpsuitSecurityBundle};

pub fn _spawn_entity(
    entity_name : String,
    transform : Transform,
    commands: &mut Commands,
) -> Entity {

    let return_entity;

    if entity_name == "jumpsuitSecurity" {

        return_entity = Some(JumpsuitSecurityBundle::spawn(transform,commands));

    } else if entity_name == "helmetSecurity" {

        return_entity = Some(HelmetSecurityBundle::spawn(transform,commands));

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
) -> Entity {

    let return_entity;

    if entity_name == "jumpsuitSecurity" {

        return_entity = Some(JumpsuitSecurityBundle::spawn_held(commands, holder_entity));

    } else if entity_name == "helmetSecurity" {

        return_entity = Some(HelmetSecurityBundle::spawn_held(commands, holder_entity));

    } else {
        warn!("Attempted to spawn an unknown entity.");
        return_entity = None;
    }

    return_entity.unwrap()

}
