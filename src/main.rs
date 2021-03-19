use std::{fs};

use bevy::{
    prelude::*
};

use bevy_rapier3d::{
    physics::{
        RapierPhysicsPlugin,
        RigidBodyHandleComponent
    },
    rapier::{
        dynamics::{
            RigidBodyBuilder,
            RigidBodySet
        },
        geometry::{
            ColliderBuilder
        }
    }
};

use serde::{Serialize, Deserialize};


use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, MessageChannelMode , MessageChannelSettings, ConnectionChannelsBuilder, ReliableChannelSettings};
use std::{net::{SocketAddr}, time::Duration};

mod space_core;

use space_core::resources::world_environments::main::{WorldEnvironment,WorldEnvironmentRaw};

#[derive(Default)]
struct NetworkReader {
    network_events: EventReader<NetworkEvent>,
}

struct PhysicsDynamicRigidBodyComponent;

fn main() {


    let current_map_environment_raw_json : String = fs::read_to_string(&DEFAULT_MAP_ENVIRONMENT_LOCATION).expect("main.rs launch_server() Error reading map environment.json file from drive.");
    let current_map_raw_environment : WorldEnvironmentRaw = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs launch_server() Error parsing map environment.json String.");
    
    let current_map_environment : WorldEnvironment = WorldEnvironment::new(current_map_raw_environment);
    

    App::build()
        /*.add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))*/
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(network_setup.system())
        .add_startup_system(launch_server.system())
        //.add_system(send_packets.system())
        .add_resource(NetworkReader::default())
        .add_system(handle_packets.system())
        .add_system(interpolate_entities_system.system())
        .add_system_to_stage(stage::PRE_UPDATE, handle_messages_server.system())
        .run();
}

const DEFAULT_MAP_MAIN_LOCATION : &str = "content\\maps\\bullseye\\main.json";
const DEFAULT_MAP_ENVIRONMENT_LOCATION : &str = "content\\maps\\bullseye\\environment.json";

#[derive(Deserialize)]
struct CellData {
    id: String,
    item: i64,
    orientation: i64
}


fn launch_server(mut net: ResMut<NetworkResource>, commands: &mut Commands) {

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

    

    info!("Loaded map bullseye with {} static cells.", current_map_main_data.len());
    
    commands.spawn((
        RigidBodyBuilder::new_dynamic().translation(0., 100., 0.),
        ColliderBuilder::ball(0.4),
        PhysicsDynamicRigidBodyComponent {}
    ));



    let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("main.rs launch_server() Error cannot find IP address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

    net.listen(socket_address);
    info!("Server is listening for connections...");

}

fn handle_messages_server(mut net: ResMut<NetworkResource>) {
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ClientMessage>() {
            info!("ClientMessage received on [{}]: {:?}",handle, client_message);
        }
    }
}

const SERVER_PORT: u16 = 57713;

/*fn send_packets(mut _net: ResMut<NetworkResource>, time: Res<Time>) {
    // This may be a heartbeat keep-alive func thats required. DISABLED ATM.
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        //net.broadcast(Packet::from("PING"));
    }
}*/

// Start In sync with client
#[derive(Serialize, Deserialize, Debug, Clone)]
enum ClientMessage {
    ConfigMessage(ConfigMessage)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ConfigMessage {
    Awoo
}

fn network_setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_MESSAGE_RELIABLE)
            .unwrap();
        /*builder
            .register::<GameStateMessage>(GAME_STATE_MESSAGE_SETTINGS)
            .unwrap();*/
    });
}

const CLIENT_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
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
    message_buffer_size: 8,
    packet_buffer_size: 8,
};
// End in sync with client



fn handle_packets(
    mut net: ResMut<NetworkResource>,
    mut state: ResMut<NetworkReader>,
    network_events: Res<Events<NetworkEvent>>,
) {

    for event in state.network_events.iter(&network_events) {

        info!("New network_events");

        match event {
            NetworkEvent::Packet(_handle, _packet) => {

                info!("New Packet!");

                // Recieved a new bytes packet from a client.





                /*let message = String::from_utf8_lossy(packet);
                if message == "PING" {
                    let message = format!("PONG @ {}", time.seconds_since_startup());
                    match net.send(*handle, Packet::from(message)) {
                        Ok(()) => {
                            info!("sent pong");
                        }
                        Err(error) => {
                            error!("{}", error);
                        }
                    }
                }*/
            },
            NetworkEvent::Connected(handle) => {
                
                // https://github.com/smokku/bevy_networking_turbulence/blob/master/examples/channels.rs
                
                info!("New Connection!");

                match net.connections.get_mut(handle) {

                    Some(connection) => {
                        match connection.remote_address() {
                            Some(remote_address) => {
    
                                info!(
                                    "Incoming connection on [{}] from [{}]",
                                    handle,
                                    remote_address
                                );
    
    
    
                            }
                            None => {
                                warn!("main.rs NetworkEvent::Connected: new connection with a weird remote_address [{}]", handle);
                            }
                        }
                    }
                    None => {
                        panic!("main.rs NetworkEvent::Connected: got packet for non-existing connection [{}]", handle);
                    }

                }

                
            }
            
            NetworkEvent::Disconnected(_) => {
                info!("New Disconnected!");
            }
        }
    }
    
}




fn interpolate_entities_system(
    query: Query<
        (&RigidBodyHandleComponent, &PhysicsDynamicRigidBodyComponent)
    >,
    bodies: ResMut<RigidBodySet>
) {

    for (rigid_body_handle, _trait_dynamic_rigid_body) in query.iter() {
        

        
        if let Some(_rigid_body) = bodies.get(rigid_body_handle.handle()) {
            
            //info!("Dynamic ball is at {} !", rigid_body.position().translation);

        }

    }

}

const CELL_SIZE : f32 = 2.;
const Y_CENTER_OFFSET : f32 = 1.;

fn cell_id_to_world(cell_id : Vec3) -> Vec3 {

    let mut world_position : Vec3 = Vec3::zero();

    world_position.x = (cell_id.x * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z * CELL_SIZE) + Y_CENTER_OFFSET;

    return world_position;

}

const STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE : &str = "main.rs json_vec3_to_vec3() Error cannot parse cell id string as Vector 3.";

fn json_vec3_to_vec3(string_vector : &str) -> Vec3 {

   

    let mut split_result : Vec<&str> = string_vector.split("(").collect();

    let mut new_string : &str = split_result[1];

    split_result = new_string.split(")").collect();

    new_string = split_result[0];

    split_result = new_string.split(",").collect();

    return Vec3::new(
        split_result[0].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[1].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[2].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE)
    );

}
