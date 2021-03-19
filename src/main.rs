use std::{fs};

use bevy::{
    prelude::*,
    app::{
        ScheduleRunnerSettings
    }
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

use space_core::structs::RawWorldEnvironment;

#[derive(Default)]
struct NetworkReader {
    network_events: EventReader<NetworkEvent>,
}




struct PhysicsDynamicRigidBodyComponent;

fn main() {


    let current_map_environment_raw_json : String = fs::read_to_string(&DEFAULT_MAP_ENVIRONMENT_LOCATION).expect("main.rs launch_server() Error reading map environment.json file from drive.");
    let current_map_environment_data : RawWorldEnvironment = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs launch_server() Error parsing map environment.json String.");
    

    App::build()
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
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


impl RawWorldEnvironment {
    fn new(adjustment_brightness: f32, adjustment_contrast: f32, adjustment_enabled: bool, adjustment_saturation: f32, ambient_light_color: String, ambient_light_energy: f32, ambient_light_sky_contribution: f32, tonemap_auto_exposure: bool, tonemap_auto_exposure_max: f32, tonemap_auto_exposure_min: f32, tonemap_auto_exposure_grey: f32, tonemap_auto_exposure_speed: f32, camera_feed_id: i64, canvas_max_layer: i64, bg_color: String, bg_energy: f32, background_mode: u8, sky_custom_fov: f32, sky_custom_orientation: String, sky_rotation: String, skyRotationDegrees: String, dofBlurFarAmount: f32, dofBlurFarDistance: f32, dofBlurFarEnabled: bool, dofBlurFarQuality: u8, dofBlurFarTransition: f32, dofBlurNearAmount: f32, dofBlurNearDistance: f32, dofBlurNearEnabled: bool, dofBlurNearQuality: f32, dofBlurNearTransition: f32, fogColor: String, fogDepthBegin: f32, fogDepthCurve: f32, fogDepthEnabled: bool, fogDepthEnd: f32, fogEnabled: bool, fogHeightCurve: f32, fogHeightEnabled: bool, fogHeightMax: f32, fogHeightMin: f32, fogSunAmount: f32, fogSunColor: String, fogTransmitCurve: f32, fogTransmitEnabled: bool, glowBicubicUpscaleEnabled: bool, glowBlendMode: u8, glowBloom: f32, glowEnabled: bool, glowHdrLuminanceCap: f32, glowHdrBleedScale: f32, glowHdrBleedTreshold: f32, glowIntensity: f32, glowStrength: f32, ssrDepthTolerance: f32, ssrEnabled: bool, ssrFadeIn: f32, ssrFadeOut: f32, ssrMaxSteps: i64, ssrRough: bool, ssaoAoChannelAffect: f32, ssaoBias: f32, ssaoBlur: u8, ssaoColor: String, ssaoEdgeSharpness: f32, ssaoEnabled: bool, ssaoIntensity: f32, ssaoIntensity2: f32, ssaoDirectLightAffect: f32, ssaoQuality: u8, ssaoRadius: f32, toneMapExposure: f32, toneMapper: u8, toneMapWhite: f32) -> Self { Self { adjustment_brightness, adjustment_contrast, adjustment_enabled, adjustment_saturation, ambient_light_color, ambient_light_energy, ambient_light_sky_contribution, tonemap_auto_exposure, tonemap_auto_exposure_max, tonemap_auto_exposure_min, tonemap_auto_exposure_grey, tonemap_auto_exposure_speed, camera_feed_id, canvas_max_layer, bg_color, bg_energy, background_mode, sky_custom_fov, sky_custom_orientation, sky_rotation, skyRotationDegrees, dofBlurFarAmount, dofBlurFarDistance, dofBlurFarEnabled, dofBlurFarQuality, dofBlurFarTransition, dofBlurNearAmount, dofBlurNearDistance, dofBlurNearEnabled, dofBlurNearQuality, dofBlurNearTransition, fogColor, fog_depth_begin: fogDepthBegin, fog_depth_curve: fogDepthCurve, fog_depth_enabled: fogDepthEnabled, fog_depth_end: fogDepthEnd, fog_enabled: fogEnabled, fog_height_curve: fogHeightCurve, fog_height_enabled: fogHeightEnabled, fog_height_max: fogHeightMax, fog_height_min: fogHeightMin, fog_sun_amount: fogSunAmount, fog_sun_color: fogSunColor, fog_transmit_curve: fogTransmitCurve, fog_transmit_enabled: fogTransmitEnabled, glow_bicubic_upscale_enabled: glowBicubicUpscaleEnabled, glow_blend_mode: glowBlendMode, glow_bloom: glowBloom, glow_enabled: glowEnabled, glow_hdr_luminance_cap: glowHdrLuminanceCap, glow_hdr_bleed_scale: glowHdrBleedScale, glow_hdr_bleed_treshold: glowHdrBleedTreshold, glow_intensity: glowIntensity, glow_strength: glowStrength, ssr_depth_tolerance: ssrDepthTolerance, ssr_enabled: ssrEnabled, ssr_fade_in: ssrFadeIn, ssr_fade_out: ssrFadeOut, ssr_max_steps: ssrMaxSteps, ssr_rough: ssrRough, ssao_ao_channel_affect: ssaoAoChannelAffect, ssao_bias: ssaoBias, ssao_blur: ssaoBlur, ssao_color: ssaoColor, ssao_edge_sharpness: ssaoEdgeSharpness, ssao_enabled: ssaoEnabled, ssao_intensity: ssaoIntensity, ssao_intensity2: ssaoIntensity2, ssao_direct_light_affect: ssaoDirectLightAffect, ssao_quality: ssaoQuality, ssao_radius: ssaoRadius, tone_map_exposure: toneMapExposure, tone_mapper: toneMapper, tone_map_white: toneMapWhite } }
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

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        //net.broadcast(Packet::from("PING"));
    }
}

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
            NetworkEvent::Packet(handle, packet) => {

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
        

        
        if let Some(rigid_body) = bodies.get(rigid_body_handle.handle()) {
            
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
