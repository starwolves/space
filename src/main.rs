use std::{env::Args, fs};

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


use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};
use std::{net::SocketAddr};

#[derive(Default)]
struct NetworkReader {
    network_events: EventReader<NetworkEvent>,
}




struct PhysicsDynamicRigidBodyComponent;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(launch_server.system())
        .add_system(send_packets.system())
        .init_resource::<NetworkReader>()
        .add_system(handle_packets.system())
        .add_system(interpolate_entities_system.system())
        .run();
}

const DEFAULT_MAP_LOCATION : &str = "content\\maps\\bullseye\\main.json";

#[derive(Deserialize)]
struct CellData {
    id: String,
    item: i64,
    orientation: i64
}

fn launch_server(mut net: ResMut<NetworkResource>, commands: &mut Commands) {

    // Load map json data into real static bodies.

    let current_map_main_raw_json : String = fs::read_to_string(&DEFAULT_MAP_LOCATION).expect("main.rs launch_server() Error reading map main.json file from drive.");
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


}

const SERVER_PORT: u16 = 57713;

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        //net.broadcast(Packet::from("PING"));
    }
}

#[derive(Serialize, Deserialize)]
struct netCodedMessage {

}

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    time: Res<Time>,
    mut state: ResMut<NetworkReader>,
    network_events: Res<Events<NetworkEvent>>,
) {
    for event in state.network_events.iter(&network_events) {
        match event {
            NetworkEvent::Packet(handle, packet) => {

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
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                
                // https://github.com/smokku/bevy_networking_turbulence/blob/master/examples/channels.rs

                Some(_) => {}
                None => {}
            }
            
            NetworkEvent::Disconnected(_) => {}
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
        

        
        if let Some(rigid_body) = bodies.get(rigid_body_handle.handle()) {
            
            info!("Dynamic ball is at {} !", rigid_body.position().translation);

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
