use bevy::{ ecs::{system::{Commands, ResMut}}, prelude::{Res, Transform, info}};

use bevy_networking_turbulence::{ConnectionChannelsBuilder, MessageChannelMode, MessageChannelSettings, NetworkResource, ReliableChannelSettings};

use std::{collections::HashMap, fs, net::{SocketAddr}, path::Path, time::Duration};

use crate::space_core::{bundles::ambience_sfx::{AmbienceSfxBundle}, components::{ server::Server}, functions::{load_main_map_data::load_main_map_data}, process_content::{
        entities::{
            raw_entity::RawEntity,
            load_raw_map_entities::load_raw_map_entities
        }
    }, resources::{all_ordered_cells::AllOrderedCells, precalculated_fov_data::PrecalculatedFOVData, server_id::ServerId, world_fov::WorldFOV}};

use serde::{Deserialize};

use crate::space_core::structs::network_messages::*;

#[derive(Deserialize)]
pub struct CellData {
    pub id: String,
    pub item: i64,
    pub orientation: i64
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
        max_message_len: 10240,
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

const SERVER_MESSAGE_UNRELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 2,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 1600,
    packet_buffer_size: 1600,
};

const SERVER_PORT: u16 = 57713;

pub fn launch_server(
    mut net: ResMut<NetworkResource>, 
    mut server_id : ResMut<ServerId>,
    mut precalculated_fov_data_resource : ResMut<PrecalculatedFOVData>,
    mut world_fov : ResMut<WorldFOV>,
    all_ordered_cells : Res<AllOrderedCells>,
    mut commands: Commands
    ) {


    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ReliableServerMessage>(SERVER_MESSAGE_RELIABLE)
            .unwrap();
        builder
            .register::<ReliableClientMessage>(CLIENT_MESSAGE_RELIABLE)
            .unwrap();
        builder
            .register::<UnreliableServerMessage>(SERVER_MESSAGE_UNRELIABLE)
            .unwrap();
    });

    // Load map json data into real static bodies.
    let main_json = Path::new("content").join("maps").join("bullseye").join("main.json");
    let current_map_main_raw_json : String = fs::read_to_string(main_json).expect("main.rs launch_server() Error reading map main.json file from drive.");
    let current_map_main_data : Vec<CellData> = serde_json::from_str(&current_map_main_raw_json).expect("main.rs launch_server() Error parsing map main.json String.");
    
    load_main_map_data(&current_map_main_data, &mut commands, &all_ordered_cells);

    
    // Load precalculated FOV data.
    let precalculated_fov_path = Path::new("data").join("FOVData.json");
    let precalculated_fov_raw_json = fs::read_to_string(precalculated_fov_path).expect("main.rs launch_server() Error reading FOVData.json file from drive.");
    let precalculated_fov_data: HashMap<String,Vec<String>> = serde_json::from_str(&precalculated_fov_raw_json).expect("main.rs launch_server() Error parsing FOVData.json file from String.");

    precalculated_fov_data_resource.data = PrecalculatedFOVData::new(precalculated_fov_data);

    // Spawn ambience SFX
    commands.spawn().insert_bundle(AmbienceSfxBundle::new(Transform::identity()));

    // So we have one reserved Id that isnt an entity for sure
    let server_component = Server;

    server_id.id = commands.spawn().insert(server_component).id();

    let entities_json = Path::new("content").join("maps").join("bullseye").join("entities.json");
    let current_map_entities_raw_json : String = fs::read_to_string(entities_json).expect("main.rs launch_server() Error reading map entities.json file from drive.");
    let current_map_entities_data : Vec<RawEntity> = serde_json::from_str(&current_map_entities_raw_json).expect("main.rs launch_server() Error parsing map entities.json String.");
    
    load_raw_map_entities(&current_map_entities_data, &mut commands);

    info!("Loaded map bullseye with {} cells(main) and {} entities.", current_map_main_data.len(), current_map_entities_data.len());

    let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("main.rs launch_server() Error cannot find IP address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

    net.listen(socket_address, None, None);
    info!("Server is ready");

}
