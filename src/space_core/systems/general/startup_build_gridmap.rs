use std::{fs, path::Path};

use bevy::prelude::{Commands, ResMut, info};

use crate::space_core::{functions::{gridmap::{build_gridmap_floor::build_gridmap_floor, build_gridmap_from_data::{build_details1_gridmap, build_main_gridmap}}}, resources::{doryen_fov::DoryenMap, gridmap_data::GridmapData, gridmap_details1::GridmapDetails1, gridmap_main::{CellDataWID, GridmapMain}}};

pub fn startup_build_gridmap(
    mut gridmap_main : ResMut<GridmapMain>,
    mut gridmap_details1 : ResMut<GridmapDetails1>,
    mut gridmap_data : ResMut<GridmapData>,
    mut fov_map : ResMut<DoryenMap>,
    mut commands: Commands
) {

    // Load map json data into real static bodies.
    let main_json = Path::new("content").join("maps").join("bullseye").join("main.json");
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

    let details1_json = Path::new("content").join("maps").join("bullseye").join("details1.json");
    let current_map_details1_raw_json : String = fs::read_to_string(details1_json).expect("main.rs launch_server() Error reading map details1_json file from drive.");
    let current_map_details1_data : Vec<CellDataWID> = serde_json::from_str(&current_map_details1_raw_json).expect("main.rs launch_server() Error parsing map details1_json String.");

    build_details1_gridmap(
        &current_map_details1_data,
        &mut gridmap_details1,
        &mut gridmap_data,
    );
    
    info!("Loaded {} cells.", current_map_main_data.len()+current_map_details1_data.len());

}
