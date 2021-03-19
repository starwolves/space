use bevy::{ecs::{Commands, ResMut}, prelude::info};

use bevy_networking_turbulence::NetworkResource;
use bevy_rapier3d::{
    rapier::{
        dynamics::{
            RigidBodyBuilder
        },
        geometry::{
            ColliderBuilder
        }
    }
};

use std::{fs, net::{SocketAddr}};

use crate::space_core::functions::{string_to_type_converters::json_vec3_to_vec3, gridmap_functions::cell_id_to_world};

use serde::{Deserialize};

#[derive(Deserialize)]
struct CellData {
    id: String,
    item: i64,
    orientation: i64
}

const SERVER_PORT: u16 = 57713;

const DEFAULT_MAP_MAIN_LOCATION : &str = "content\\maps\\bullseye\\main.json";

pub fn launch_server(mut net: ResMut<NetworkResource>, commands: &mut Commands) {

    // Load map json data into real static bodies.

    let current_map_main_raw_json : String = fs::read_to_string(&DEFAULT_MAP_MAIN_LOCATION).expect("main.rs launch_server() Error reading map main.json file from drive.");
    let current_map_main_data : Vec<CellData> = serde_json::from_str(&current_map_main_raw_json).expect("main.rs launch_server() Error parsing map main.json String.");
    
    

    for cell_data in current_map_main_data.iter() {
        
        let cell_id = json_vec3_to_vec3(&cell_data.id);

        let world_position = cell_id_to_world(cell_id);

        commands.spawn((
            RigidBodyBuilder::new_static().translation(world_position.x, world_position.y, world_position.z),
            ColliderBuilder::cuboid(1., 1., 1.),
        ));

    }


    info!("Loaded map bullseye with {} main gridmap cells.", current_map_main_data.len());

    let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("main.rs launch_server() Error cannot find IP address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

    net.listen(socket_address);
    info!("Server is ready");

}
