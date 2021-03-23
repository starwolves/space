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

use space_core::{
    systems::{
        launch_server::launch_server,
        handle_network_messages::handle_network_messages,
        handle_network_events::handle_network_events
    },
    resources::{
        world_environments::{WorldEnvironment,WorldEnvironmentRaw},
        network_reader::NetworkReader,
        tick_rate::TickRate
    }
};

const DEFAULT_MAP_ENVIRONMENT_LOCATION : &str = "content\\maps\\bullseye\\environment.json";

fn main() {


    let current_map_environment_raw_json : String = fs::read_to_string(&DEFAULT_MAP_ENVIRONMENT_LOCATION).expect("main.rs launch_server() Error reading map environment.json file from drive.");
    let current_map_raw_environment : WorldEnvironmentRaw = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs launch_server() Error parsing map environment.json String.");
    let current_map_environment : WorldEnvironment = WorldEnvironment::new(current_map_raw_environment);
    
    let tick_rate = TickRate {
        rate : 24
    };

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(launch_server.system())
        .add_resource(NetworkReader::default())
        .add_resource(current_map_environment)
        .add_resource(tick_rate)
        .add_system(handle_network_events.system())
        .add_system_to_stage(stage::PRE_UPDATE, handle_network_messages.system())
        .run();
}
