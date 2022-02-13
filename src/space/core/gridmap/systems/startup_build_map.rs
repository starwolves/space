use std::{fs, path::Path};

use bevy::prelude::{Commands, Res, ResMut, info};

use crate::space::{core::{gridmap::{resources::{GridmapMain, GridmapDetails1, GridmapData, CellDataWID, DoryenMap}, functions::{build_gridmap_floor::build_gridmap_floor, build_gridmap_from_data::{build_main_gridmap, build_details1_gridmap}}}, entity::{resources::EntityDataResource, functions::{raw_entity::RawEntity, load_raw_map_entities::load_raw_map_entities}}}};

pub fn startup_build_map(
    mut gridmap_main : ResMut<GridmapMain>,
    mut gridmap_details1 : ResMut<GridmapDetails1>,
    mut gridmap_data : ResMut<GridmapData>,
    entity_data : Res<EntityDataResource>,
    mut fov_map : ResMut<DoryenMap>,
    mut commands: Commands,
) {

    // Load map json data into real static bodies.
    let main_json = Path::new("data").join("maps").join("bullseye").join("main.json");
    let current_map_main_raw_json : String = fs::read_to_string(main_json).expect("main.rs launch_server() Error reading map main.json file from drive.");
    let current_map_main_data : Vec<CellDataWID> = serde_json::from_str(&current_map_main_raw_json).expect("main.rs launch_server() Error parsing map main.json String.");

    build_gridmap_floor(&mut commands);

    build_main_gridmap(
        &current_map_main_data,
        &mut commands,
        &mut gridmap_main,
        &mut fov_map,
        &mut gridmap_data,
    );

    let details1_json = Path::new("data").join("maps").join("bullseye").join("details1.json");
    let current_map_details1_raw_json : String = fs::read_to_string(details1_json).expect("main.rs launch_server() Error reading map details1_json file from drive.");
    let current_map_details1_data : Vec<CellDataWID> = serde_json::from_str(&current_map_details1_raw_json).expect("main.rs launch_server() Error parsing map details1_json String.");

    build_details1_gridmap(
        &current_map_details1_data,
        &mut gridmap_details1,
        &mut gridmap_data,
    );
    
    info!("Spawned {} map cells.", current_map_main_data.len()+current_map_details1_data.len());

    let entities_json = Path::new("data").join("maps").join("bullseye").join("entities.json");
    let current_map_entities_raw_json : String = fs::read_to_string(entities_json).expect("main.rs launch_server() Error reading map entities.json file from drive.");
    let current_map_entities_data : Vec<RawEntity> = serde_json::from_str(&current_map_entities_raw_json).expect("main.rs launch_server() Error parsing map entities.json String.");
    
    load_raw_map_entities(&current_map_entities_data, &mut commands, &entity_data);

    info!("Spawned {} entities.", current_map_entities_data.len());

}
