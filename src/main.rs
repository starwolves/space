use std::{fs};

use bevy::{
    prelude::*
};

use bevy_rapier3d::{
    physics::{
        RapierPhysicsPlugin
    }
};

use bevy_networking_turbulence::{NetworkingPlugin};

mod space_core;

use space_core::{resources::{server_id::ServerId,all_ordered_cells::AllOrderedCells, authid_i::AuthidI, blackcells_data::BlackcellsData, non_blocking_cells_list::NonBlockingCellsList, network_reader::NetworkReader, tick_rate::TickRate, unique_entity_id::UniqueEntityId, world_environments::{WorldEnvironment,WorldEnvironmentRaw}}, systems::{
        launch_server::launch_server,
        handle_network_messages::handle_network_messages,
        handle_network_events::handle_network_events
    }};

const DEFAULT_MAP_ENVIRONMENT_LOCATION : &str = "content\\maps\\bullseye\\environment.json";
const DEFAULT_MAP_BLACKCELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\blackcells.json";
const DEFAULT_MAP_BLOCKING_CELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\nonblockinglist.json";
const DEFAULT_MAP_MAINORDERED_CELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\mainordered.json";
const DEFAULT_MAP_DETAILS1ORDERED_CELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\details1ordered.json";


fn main() {


    let current_map_environment_raw_json : String = fs::read_to_string(&DEFAULT_MAP_ENVIRONMENT_LOCATION).expect("main.rs launch_server() Error reading map environment.json file from drive.");
    let current_map_raw_environment : WorldEnvironmentRaw = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs launch_server() Error parsing map environment.json String.");
    let current_map_environment : WorldEnvironment = WorldEnvironment::new(current_map_raw_environment);
    
    let current_map_blackcells_data_raw_json : String = fs::read_to_string(&DEFAULT_MAP_BLACKCELLS_DATA_LOCATION).expect("main.rs launch_server() Error reading blackcells_data from drive.");
    let current_map_blackcells : BlackcellsData = serde_json::from_str(&current_map_blackcells_data_raw_json).expect("main.rs launch_server() Error parsing map blackcells.json String.");

    let current_map_blocking_cells_raw_json : String = fs::read_to_string(&DEFAULT_MAP_BLOCKING_CELLS_DATA_LOCATION).expect("main.rs launch_server() Error reading map blockinglist.json from drive.");
    let current_map_blocking_cells_data : Vec<i64> = serde_json::from_str(&current_map_blocking_cells_raw_json).expect("main.rs launch_server() Error parsing map blockinglist.json String.");

    let current_map_blocking_cells = NonBlockingCellsList{
        list : current_map_blocking_cells_data
    };

    let current_map_mainordered_cells_raw_json : String = fs::read_to_string(&DEFAULT_MAP_MAINORDERED_CELLS_DATA_LOCATION).expect("main.rs launch_server() Error reading map mainordered.json drive.");
    let current_map_mainordered_cells : Vec<String> = serde_json::from_str(&current_map_mainordered_cells_raw_json).expect("main.rs launch_server() Error parsing map mainordered.json String.");
    let current_map_details1ordered_cells_raw_json : String = fs::read_to_string(&DEFAULT_MAP_DETAILS1ORDERED_CELLS_DATA_LOCATION).expect("main.rs launch_server() Error reading map details1ordered.json drive.");
    let current_map_details1ordered_cells : Vec<String> = serde_json::from_str(&current_map_details1ordered_cells_raw_json).expect("main.rs launch_server() Error parsing map details1ordered.json String.");

    let all_ordered_cells = AllOrderedCells{
        main: current_map_mainordered_cells,
        details1: current_map_details1ordered_cells
    };

    let tick_rate = TickRate {
        rate : 24
    };

    let authid_i = AuthidI {
        i : 0
    };

    let unique_entity_id = UniqueEntityId {
        i:0
    };

    let server_id = ServerId {
        id:0
    };
    
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(launch_server.system())
        .add_resource(NetworkReader::default())
        .add_resource(current_map_environment)
        .add_resource(tick_rate)
        .add_resource(current_map_blackcells)
        .add_resource(current_map_blocking_cells)
        .add_resource(all_ordered_cells)
        .add_resource(authid_i)
        .add_resource(unique_entity_id)
        .add_resource(server_id)
        .add_system(handle_network_events.system())
        .add_system_to_stage(stage::PRE_UPDATE, handle_network_messages.system())
        .run();
}
