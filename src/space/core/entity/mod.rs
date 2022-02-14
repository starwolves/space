use bevy::prelude::{ResMut, info};

use crate::space::{core::entity::resources::EntityDataProperties, entities::{jumpsuit_security::spawn::JumpsuitSecurityBundle, helmet_security::spawn::HelmetSecurityBundle, pistol_l1::spawn::PistolL1Bundle, human_male_pawn::spawn::HumanMalePawnBundle, air_lock_security::spawn::SecurityAirlockBundle, counter_window_security::spawn::SecurityCounterWindowBundle, construction_tool_admin::spawn::ConstructionToolBundle}};

use self::resources::EntityDataResource;

pub mod components;
pub mod resources;
pub mod functions;
pub mod events;
pub mod systems;




pub fn startup_entities(
    mut entity_data : ResMut<EntityDataResource>,
) {

    let mut entities = vec![];

    entities.push(EntityDataProperties {
        name: "jumpsuitSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(JumpsuitSecurityBundle::spawn),
        constructable: false,
    });

    entities.push(EntityDataProperties {
        name: "helmetSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HelmetSecurityBundle::spawn),
        constructable: false,
    });

    entities.push(EntityDataProperties {
        name: "pistolL1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(PistolL1Bundle::spawn),
        constructable: false,
    });

    entities.push(EntityDataProperties {
        name: "humanDummy".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HumanMalePawnBundle::spawn),
        constructable: false,
    });

    entities.push(EntityDataProperties {
        name: "securityAirLock1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(SecurityAirlockBundle::spawn),
        constructable: true,
    });

    entities.push(EntityDataProperties {
        name: "securityCounterWindow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(SecurityCounterWindowBundle::spawn),
        constructable: true,
    });

    entities.push(EntityDataProperties {
        name: "constructionTool".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(ConstructionToolBundle::spawn),
        constructable: false,
    });

    info!("Loaded {} different entity types.", entities.len());

    for entity_properties in entities {

        entity_data.id_to_name.insert(entity_properties.id, entity_properties.name.clone());
        entity_data.name_to_id.insert(entity_properties.name.clone(), entity_properties.id);

        entity_data.data.push(entity_properties);

    }

}
