use bevy::{prelude::{info, ResMut, Transform}, math::Quat};

use crate::space::{
    core::entity::resources::{ConstructableData, EntityDataProperties},
    entities::{
        air_lock_security::spawn::SecurityAirlockBundle,
        construction_tool_admin::spawn::ConstructionToolBundle,
        counter_window_security::spawn::SecurityCounterWindowBundle,
        helmet_security::spawn::HelmetSecurityBundle, human_male_pawn::spawn::HumanMalePawnBundle,
        jumpsuit_security::spawn::JumpsuitSecurityBundle, pistol_l1::spawn::PistolL1Bundle,
    },
};

use self::resources::EntityDataResource;

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

pub fn startup_entities(mut entity_data: ResMut<EntityDataResource>) {
    let mut entities = vec![];

    entities.push(EntityDataProperties {
        name: "jumpsuitSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(JumpsuitSecurityBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "helmetSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HelmetSecurityBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "pistolL1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(PistolL1Bundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "humanDummy".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HumanMalePawnBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "securityAirLock1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(SecurityAirlockBundle::spawn),
        constructable: Some(ConstructableData {
            transform_offset: Transform::identity(),
        }),
    });

    let mut transform =  Transform::identity();
    transform.translation.y=0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    entities.push(EntityDataProperties {
        name: "securityCounterWindow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(SecurityCounterWindowBundle::spawn),
        constructable: Some(ConstructableData {
            transform_offset: transform,
        }),
    });

    entities.push(EntityDataProperties {
        name: "constructionTool".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(ConstructionToolBundle::spawn),
        ..Default::default()
    });

    info!("Loaded {} different entity types.", entities.len());

    for entity_properties in entities {
        entity_data
            .id_to_name
            .insert(entity_properties.id, entity_properties.name.clone());
        entity_data
            .name_to_id
            .insert(entity_properties.name.clone(), entity_properties.id);

        entity_data.data.push(entity_properties);
    }
}
