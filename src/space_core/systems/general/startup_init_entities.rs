use std::{fs, path::Path};

use bevy::prelude::{Commands, ResMut, info};

use crate::space_core::{bundles::{helmet_security::HelmetSecurityBundle, human_male_pawn::HumanMalePawnBundle, jumpsuit_security::JumpsuitSecurityBundle, pistol_l1::PistolL1Bundle, security_airlock::SecurityAirlockBundle, security_counter_window::SecurityCounterWindowBundle}, functions::process_content::{load_raw_map_entities::load_raw_map_entities, raw_entity::RawEntity}, resources::entity_data_resource::{EntityDataProperties, EntityDataResource}};



pub fn startup_init_entities(

    mut entity_data : ResMut<EntityDataResource>,
    mut commands : Commands,

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

    for entity_properties in entities {

        entity_data.id_to_name.insert(entity_properties.id, entity_properties.name.clone());
        entity_data.name_to_id.insert(entity_properties.name.clone(), entity_properties.id);

        entity_data.data.push(entity_properties);

    }


    let entities_json = Path::new("content").join("maps").join("bullseye").join("entities.json");
    let current_map_entities_raw_json : String = fs::read_to_string(entities_json).expect("main.rs launch_server() Error reading map entities.json file from drive.");
    let current_map_entities_data : Vec<RawEntity> = serde_json::from_str(&current_map_entities_raw_json).expect("main.rs launch_server() Error parsing map entities.json String.");
    
    load_raw_map_entities(&current_map_entities_data, &mut commands, &entity_data);

    info!("Loaded {} entities.", current_map_entities_data.len());


}
