use bevy::prelude::{ResMut, info};

use crate::space_core::{bundles::{helmet_security::HelmetSecurityBundle, human_male_pawn::HumanMalePawnBundle, jumpsuit_security::JumpsuitSecurityBundle, pistol_l1::PistolL1Bundle, security_airlock::SecurityAirlockBundle, security_counter_window::SecurityCounterWindowBundle}, resources::entity_data_resource::{EntityDataProperties, EntityDataResource}};



pub fn startup_init_entities(
    mut entity_data : ResMut<EntityDataResource>,
) {

    let mut entities = vec![];

    entities.push(EntityDataProperties {
        name: "jumpsuitSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(JumpsuitSecurityBundle::spawn),
    });

    entities.push(EntityDataProperties {
        name: "helmetSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HelmetSecurityBundle::spawn),
    });

    entities.push(EntityDataProperties {
        name: "pistolL1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(PistolL1Bundle::spawn),
    });

    entities.push(EntityDataProperties {
        name: "humanDummy".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HumanMalePawnBundle::spawn),
    });

    entities.push(EntityDataProperties {
        name: "securityAirLock1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(SecurityAirlockBundle::spawn),
    });

    entities.push(EntityDataProperties {
        name: "securityCounterWindow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(SecurityCounterWindowBundle::spawn),
    });

    info!("Loaded {} different entity types.", entities.len());

    for entity_properties in entities {

        entity_data.id_to_name.insert(entity_properties.id, entity_properties.name.clone());
        entity_data.name_to_id.insert(entity_properties.name.clone(), entity_properties.id);

        entity_data.data.push(entity_properties);

    }

}
