use bevy::{ecs::{system::{Commands, ResMut}}, prelude::{Transform, info}};

use bevy_networking_turbulence::{ConnectionChannelsBuilder, MessageChannelMode, MessageChannelSettings, NetworkResource, ReliableChannelSettings};

use std::{collections::HashMap, fs, net::{SocketAddr}, path::Path, time::Duration};

use crate::space_core::{bundles::ambience_sfx::{AmbienceSfxBundle}, components::{ server::Server}, functions::{gridmap::build_gridmap_from_data::{build_details1, build_main}, process_content::{load_raw_map_entities::load_raw_map_entities, raw_entity::RawEntity, }}, resources::{all_ordered_cells::AllOrderedCells, blackcells_data::BlackcellsData, doryen_fov::{ DoryenMap}, gridmap_details1::GridmapDetails1, gridmap_main::{CellDataWID, GridmapMain}, network_messages::{ReliableClientMessage, ReliableServerMessage, UnreliableServerMessage}, non_blocking_cells_list::NonBlockingCellsList, precalculated_fov_data::PrecalculatedFOVData, server_id::ServerId, spawn_points::{SpawnPoint, SpawnPointRaw, SpawnPoints}, world_environments::{WorldEnvironment, WorldEnvironmentRaw}}};


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
    message_buffer_size: 256,
    packet_buffer_size: 256,
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
    message_buffer_size: 256,
    packet_buffer_size: 256,
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
    mut gridmap_main : ResMut<GridmapMain>,
    mut gridmap_details1 : ResMut<GridmapDetails1>,
    mut all_ordered_cells : ResMut<AllOrderedCells>,
    mut map_environment : ResMut<WorldEnvironment>,
    mut blackcells_data_res : ResMut<BlackcellsData>,
    mut non_blocking_cells : ResMut<NonBlockingCellsList>,
    mut spawn_points_res : ResMut<SpawnPoints>,
    mut fov_map : ResMut<DoryenMap>,
    mut commands: Commands
) {


    let environment_json_location = Path::new("content").join("maps").join("bullseye").join("environment.json");
    let current_map_environment_raw_json : String = fs::read_to_string(environment_json_location).expect("main.rs main() Error reading map environment.json file from drive.");
    let current_map_raw_environment : WorldEnvironmentRaw = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs main() Error parsing map environment.json String.");
    let current_map_environment : WorldEnvironment = WorldEnvironment::new(current_map_raw_environment);

    current_map_environment.adjust(&mut map_environment);

    let blackcells_json_location = Path::new("content").join("maps").join("bullseye").join("blackcells.json");
    let current_map_blackcells_data_raw_json : String = fs::read_to_string(blackcells_json_location).expect("main.rs main() Error reading blackcells_data from drive.");
    let current_map_blackcells : BlackcellsData = serde_json::from_str(&current_map_blackcells_data_raw_json).expect("main.rs main() Error parsing map blackcells.json String.");
    
    blackcells_data_res.blackcell_blocking_id = current_map_blackcells.blackcell_blocking_id;
    blackcells_data_res.blackcell_id = current_map_blackcells.blackcell_id;

    let blocking_cells_json_location = Path::new("content").join("maps").join("bullseye").join("nonblockinglist.json");
    let current_map_blocking_cells_raw_json : String = fs::read_to_string(&blocking_cells_json_location).expect("main.rs main() Error reading map nonblockinglist.json from drive.");
    let current_map_blocking_cells_data : Vec<i64> = serde_json::from_str(&current_map_blocking_cells_raw_json).expect("main.rs main() Error parsing map nonblockinglist.json String.");

    non_blocking_cells.list = current_map_blocking_cells_data;

    let mainordered_cells_json = Path::new("content").join("maps").join("bullseye").join("mainordered.json");
    let current_map_mainordered_cells_raw_json : String = fs::read_to_string(mainordered_cells_json).expect("main.rs main() Error reading map mainordered.json drive.");
    let current_map_mainordered_cells : Vec<String> = serde_json::from_str(&current_map_mainordered_cells_raw_json).expect("main.rs main() Error parsing map mainordered.json String.");

    let details1ordered_cells_json = Path::new("content").join("maps").join("bullseye").join("details1ordered.json");
    let current_map_details1ordered_cells_raw_json : String = fs::read_to_string(details1ordered_cells_json).expect("main.rs main() Error reading map details1ordered.json drive.");
    let current_map_details1ordered_cells : Vec<String> = serde_json::from_str(&current_map_details1ordered_cells_raw_json).expect("main.rs main() Error parsing map details1ordered.json String.");

    let spawnpoints_json = Path::new("content").join("maps").join("bullseye").join("spawnpoints.json");
    let current_map_spawn_points_raw_json : String = fs::read_to_string(spawnpoints_json).expect("main.rs main() Error reading map spawnpoints.json from drive.");
    let current_map_spawn_points_raw : Vec<SpawnPointRaw> = serde_json::from_str(&current_map_spawn_points_raw_json).expect("main.rs main() Error parsing map spawnpoints.json String.");
    let mut current_map_spawn_points : Vec<SpawnPoint> = vec![];

    for raw_point in current_map_spawn_points_raw.iter() {
        current_map_spawn_points.push(SpawnPoint::new(raw_point));
    }

    spawn_points_res.list = current_map_spawn_points;
    spawn_points_res.i = 0;

    all_ordered_cells.main =  current_map_mainordered_cells;
    all_ordered_cells.details1 = current_map_details1ordered_cells;


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
    let current_map_main_data : Vec<CellDataWID> = serde_json::from_str(&current_map_main_raw_json).expect("main.rs launch_server() Error parsing map main.json String.");

    build_main(
        &current_map_main_data,
        &mut commands,
        &all_ordered_cells,
        &mut gridmap_main,
        &mut fov_map,
        &mut non_blocking_cells,
    );

    let details1_json = Path::new("content").join("maps").join("bullseye").join("details1.json");
    let current_map_details1_raw_json : String = fs::read_to_string(details1_json).expect("main.rs launch_server() Error reading map details1_json file from drive.");
    let current_map_details1_data : Vec<CellDataWID> = serde_json::from_str(&current_map_details1_raw_json).expect("main.rs launch_server() Error parsing map details1_json String.");

    build_details1(
        &current_map_details1_data,
        &mut commands,
        &mut gridmap_details1,
    );

    
    // Load precalculated FOV data.
    let precalculated_fov_path = Path::new("content").join("FOVData.json");
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
