use bevy::{ecs::{system::{Commands, ResMut}}, prelude::info};

use bevy_networking_turbulence::{ConnectionChannelsBuilder, MessageChannelMode, MessageChannelSettings, NetworkResource, ReliableChannelSettings};
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

use std::{fs, net::{SocketAddr}, time::Duration};

use crate::space_core::{
    components::{
        server::Server
    }, 
    functions::{
        string_to_type_converters::string_vec3_to_vec3, 
        gridmap_functions::cell_id_to_world
    },
    resources::{
        server_id::ServerId
    },
    process_content::{
        entities::{
            raw_entity::RawEntity,
            load_raw_map_entities::load_raw_map_entities
        }
    }
};

use serde::{Deserialize};

use crate::space_core::structs::network_messages::*;

#[allow(dead_code)]
#[derive(Deserialize)]
struct CellData {
    id: String,
    item: i64,
    orientation: i64
}



const SERVER_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 163840,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 128,
    packet_buffer_size: 128,
};

const CLIENT_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 163840,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 128,
    packet_buffer_size: 128,
};

const SERVER_PORT: u16 = 57713;

const DEFAULT_MAP_MAIN_LOCATION : &str = "content\\maps\\bullseye\\main.json";
const DEFAULT_MAP_ENTITIES_LOCATION : &str = "content\\maps\\bullseye\\entities.json";

pub fn launch_server(
    mut net: ResMut<NetworkResource>, 
    mut server_id : ResMut<ServerId>,
    mut commands: Commands
    ) {


    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ReliableServerMessage>(SERVER_MESSAGE_RELIABLE)
            .unwrap();
        builder
            .register::<ReliableClientMessage>(CLIENT_MESSAGE_RELIABLE)
            .unwrap();
    });



    // Load map json data into real static bodies.

    let current_map_main_raw_json : String = fs::read_to_string(&DEFAULT_MAP_MAIN_LOCATION).expect("main.rs launch_server() Error reading map main.json file from drive.");
    let current_map_main_data : Vec<CellData> = serde_json::from_str(&current_map_main_raw_json).expect("main.rs launch_server() Error parsing map main.json String.");
    
    

    for cell_data in current_map_main_data.iter() {
        
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let world_position = cell_id_to_world(cell_id);

        commands.spawn().insert_bundle((
            RigidBodyBuilder::new_static().translation(world_position.x, world_position.y, world_position.z),
            ColliderBuilder::cuboid(1., 1., 1.),
        ));

    }



    // So we have one reserved Id that isnt an entity for sure
    let server_component = Server;

    server_id.id = commands.spawn().insert(server_component).id();

    let current_map_entities_raw_json : String = fs::read_to_string(&DEFAULT_MAP_ENTITIES_LOCATION).expect("main.rs launch_server() Error reading map entities.json file from drive.");
    let current_map_entities_data : Vec<RawEntity> = serde_json::from_str(&current_map_entities_raw_json).expect("main.rs launch_server() Error parsing map entities.json String.");
    
    load_raw_map_entities(&current_map_entities_data, &mut commands);

    info!("Loaded map bullseye with {} cells(main) and {} entities.", current_map_main_data.len(), current_map_entities_data.len());

    let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("main.rs launch_server() Error cannot find IP address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

    net.listen(socket_address);
    info!("Server is ready");

}
