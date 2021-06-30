use std::{collections::HashMap, fs, path::Path};

use bevy::{app::CoreStage::{PreUpdate, Update, PostUpdate}, core::FixedTimestep, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*, transform::TransformPlugin};

use bevy_rapier3d::{na::Quaternion, physics::{
        RapierPhysicsPlugin
    }, prelude::NoUserData};

use bevy_networking_turbulence::{NetworkingPlugin};

mod space_core;

use space_core::{
    events::{
        general::{
            scene_ready::SceneReady, ui_input::UIInput, 
            ui_input_transmit_text::UIInputTransmitText
        },
        net::{
            net_done_boarding::NetDoneBoarding,
            net_on_boarding::NetOnBoarding, 
            net_on_new_player_connection::NetOnNewPlayerConnection, 
            net_on_setupui::NetOnSetupUI,
            net_load_entity::NetLoadEntity, 
            net_send_entity_updates::NetSendEntityUpdates
        }
    },
    resources::{
        all_ordered_cells::AllOrderedCells,
        authid_i::AuthidI, blackcells_data::BlackcellsData,
        handle_to_entity::HandleToEntity,
        non_blocking_cells_list::NonBlockingCellsList,
        server_id::ServerId,
        spawn_points::{SpawnPoint, SpawnPointRaw, SpawnPoints},
        tick_rate::TickRate, used_names::UsedNames,
        world_environments::{WorldEnvironment,WorldEnvironmentRaw}
    }, 
    systems::{
        general::{
            done_boarding::done_boarding,
            on_boarding::on_boarding,
            on_setupui::on_setupui,
            scene_ready_event::scene_ready_event,
            ui_input_event::ui_input_event,
            ui_input_transmit_data_event::ui_input_transmit_data_event,
            on_spawning::on_spawning,
            visible_checker::visible_checker,
            world_fov::world_fov as world_fov_system
        },
        net::{
            handle_network_events::handle_network_events,
            handle_network_messages::handle_network_messages,
            net_send_message_event::net_send_messages_event,
        },
        startup::{
            launch_server::launch_server,
        },
        entity_updates::{
            omni_light_update::omni_light_update,
            send_entity_updates::send_entity_updates
        }
    }
};

use crate::space_core::{events::{general::{boarding_player::BoardingPlayer, build_graphics::BuildGraphics, input_chat_message::InputChatMessage, movement_input::MovementInput}, net::{net_chat_message::NetChatMessage, net_on_spawning::NetOnSpawning, net_send_world_environment::NetSendWorldEnvironment, net_unload_entity::NetUnloadEntity}, physics::{air_lock_collision::AirLockCollision, counter_window_sensor_collision::CounterWindowSensorCollision}}, resources::{asana_boarding_announcements::AsanaBoardingAnnouncements, gridmap_main::GridmapMain, precalculated_fov_data::PrecalculatedFOVData, sfx_auto_destroy_timers::SfxAutoDestroyTimers, world_fov::WorldFOV, y_axis_rotations::PlayerYAxisRotations}, systems::{entity_updates::{air_lock_update::air_lock_update, counter_window_update::counter_window_update, gi_probe_update::gi_probe_update, standard_character_update::standard_character_update, reflection_probe_update::reflection_probe_update, repeating_sfx_update::repeating_sfx_update, sfx_update::sfx_update, world_mode_update::world_mode_update}, general::{air_lock_events::air_lock_events, build_graphics_event::build_graphics_event, chat_message_input_event::chat_message_input_event, counter_window_events::counter_window_events, move_player_bodies::move_player_bodies, movement_input_event::movement_input_event, physics_events::physics_events, tick_asana_boarding_announcements::tick_asana_boarding_announcements, tick_timers::tick_timers, tick_timers_slowed::tick_timers_slowed}, net::{broadcast_interpolation_transforms::broadcast_interpolation_transforms, broadcast_position_updates::broadcast_position_updates}}};


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum PreUpdateLabels {
    NetEvents
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum UpdateLabels {
    ProcessMovementInput,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum PostUpdateLabels {
    EntityUpdate,
    SendEntityUpdates,
    VisibleChecker,

}

const INTERPOLATION_LABEL: &str = "fixed_timestep_interpolation";
const INTERPOLATION_LABEL1: &str = "fixed_timestep_interpolation1";



fn main() {


    let environment_json_location = Path::new("content").join("maps").join("bullseye").join("environment.json");
    let current_map_environment_raw_json : String = fs::read_to_string(environment_json_location).expect("main.rs main() Error reading map environment.json file from drive.");
    let current_map_raw_environment : WorldEnvironmentRaw = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs main() Error parsing map environment.json String.");
    let current_map_environment : WorldEnvironment = WorldEnvironment::new(current_map_raw_environment);
    
    let blackcells_json_location = Path::new("content").join("maps").join("bullseye").join("blackcells.json");
    let current_map_blackcells_data_raw_json : String = fs::read_to_string(blackcells_json_location).expect("main.rs main() Error reading blackcells_data from drive.");
    let current_map_blackcells : BlackcellsData = serde_json::from_str(&current_map_blackcells_data_raw_json).expect("main.rs main() Error parsing map blackcells.json String.");

    let blocking_cells_json_location = Path::new("content").join("maps").join("bullseye").join("nonblockinglist.json");
    let current_map_blocking_cells_raw_json : String = fs::read_to_string(&blocking_cells_json_location).expect("main.rs main() Error reading map nonblockinglist.json from drive.");
    let current_map_blocking_cells_data : Vec<i64> = serde_json::from_str(&current_map_blocking_cells_raw_json).expect("main.rs main() Error parsing map nonblockinglist.json String.");

    let current_map_blocking_cells = NonBlockingCellsList{
        list : current_map_blocking_cells_data
    };

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

    

    let spawn_points = SpawnPoints {
        list : current_map_spawn_points,
        i : 0
    };

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

    let server_id = ServerId {
        id: Entity::new(0)
    };

    let used_names = UsedNames {
        names : vec![]
    };

    let handle_to_entity = HandleToEntity {
        map : HashMap::new(),
        inv_map : HashMap::new()
    };

    let y_axis_rotations = PlayerYAxisRotations {
        rotations: vec![
            //0deg
            Quaternion::new(1.,0.,0.,0.),
            //45deg
            Quaternion::new(0.9238795, 0. , 0.3826834, 0.),
            //90deg
            Quaternion::new(0.7071068, 0., 0.7071068, 0.),
            //135deg
            Quaternion::new(0.3826834 ,0. , 0.9238795, 0.),
            //180deg
            Quaternion::new(0. ,0., 1., 0.),
            //225deg
            Quaternion::new(-0.3826834, 0., 0.9238795, 0.),
            //270deg
            Quaternion::new(-0.7071068, 0., 0.7071068, 0.),
            //315deg
            Quaternion::new(-0.9238795, 0., 0.3826834,0.),
        ]
    };

    let sfx_auto_destroy_timers = SfxAutoDestroyTimers {
        timers : HashMap::new()
    };

    let asana_boarding_announcements = AsanaBoardingAnnouncements {
        announcements : HashMap::new()
    };

    let precalculated_fov_data = PrecalculatedFOVData {
        data : HashMap::new()
    };
    
    let world_fov = WorldFOV {
        data : HashMap::new(),
        to_be_recalculated: vec![],
        to_be_recalculated_priority: vec![],
        init: true,
        blocking_load_at_init: false,
    };

    let gridmap_main = GridmapMain {
        data : HashMap::new()
    };

    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(NetworkingPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(current_map_environment)
        .insert_resource(tick_rate)
        .insert_resource(current_map_blackcells)
        .insert_resource(current_map_blocking_cells)
        .insert_resource(all_ordered_cells)
        .insert_resource(authid_i)
        .insert_resource(server_id)
        .insert_resource(used_names)
        .insert_resource(handle_to_entity)
        .insert_resource(spawn_points)
        .insert_resource(y_axis_rotations)
        .insert_resource(sfx_auto_destroy_timers)
        .insert_resource(asana_boarding_announcements)
        .insert_resource(precalculated_fov_data)
        .insert_resource(world_fov)
        .insert_resource(gridmap_main)
        .add_event::<UIInput>()
        .add_event::<SceneReady>()
        .add_event::<UIInputTransmitText>()
        .add_event::<MovementInput>()
        .add_event::<BuildGraphics>()
        .add_event::<InputChatMessage>()
        .add_event::<NetOnNewPlayerConnection>()
        .add_event::<NetOnBoarding>()
        .add_event::<NetOnSetupUI>()
        .add_event::<NetDoneBoarding>()
        .add_event::<NetLoadEntity>()
        .add_event::<NetUnloadEntity>()
        .add_event::<NetSendEntityUpdates>()
        .add_event::<NetSendWorldEnvironment>()
        .add_event::<NetChatMessage>()
        .add_event::<AirLockCollision>()
        .add_event::<CounterWindowSensorCollision>()
        .add_event::<NetOnSpawning>()
        .add_event::<BoardingPlayer>()
        .add_startup_system(launch_server.system())
        .add_system_to_stage(
            PreUpdate, 
            handle_network_events.system()
            .label(PreUpdateLabels::NetEvents)
        )
        .add_system_to_stage(PreUpdate, 
            handle_network_messages.system()
            .after(PreUpdateLabels::NetEvents)
        )
        .add_system_to_stage(
            Update, 
            movement_input_event.system()
            .label(UpdateLabels::ProcessMovementInput)
        )
        .add_system_to_stage(
            Update,
            move_player_bodies.system()
            .after(UpdateLabels::ProcessMovementInput)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./2.)
                .with_label(INTERPOLATION_LABEL1))
                .with_system(broadcast_position_updates.system())
                .with_system(tick_timers_slowed.system())
        )
        .add_system(ui_input_event.system())
        .add_system(scene_ready_event.system())
        .add_system(on_boarding.system())
        .add_system(on_setupui.system())
        .add_system(build_graphics_event.system())
        .add_system(chat_message_input_event.system())
        .add_system(physics_events.system())
        .add_system(air_lock_events.system())
        .add_system(counter_window_events.system())
        .add_system(tick_timers.system())
        .add_system(tick_asana_boarding_announcements.system())
        .add_system(world_fov_system.system())
        .add_system(ui_input_transmit_data_event.system())
        .add_system(done_boarding.system())
        .add_system(on_spawning.system())
        .add_system_set_to_stage( PostUpdate,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./24.)
                .with_label(INTERPOLATION_LABEL))
                .with_system(broadcast_interpolation_transforms.system())
        )
        .add_system_set_to_stage(PostUpdate, 
            SystemSet::new()
            .label(PostUpdateLabels::EntityUpdate)
            .with_system(omni_light_update.system())
            .with_system(standard_character_update.system())
            .with_system(world_mode_update.system())
            .with_system(gi_probe_update.system())
            .with_system(reflection_probe_update.system())
            .with_system(air_lock_update.system())
            .with_system(sfx_update.system())
            .with_system(repeating_sfx_update.system())
            .with_system(counter_window_update.system())
            .with_system(counter_window_update.system())
        )
        .add_system_to_stage(PostUpdate, 
            send_entity_updates.system()
            .after(PostUpdateLabels::EntityUpdate)
            .label(PostUpdateLabels::SendEntityUpdates)
        )
        .add_system_to_stage(PostUpdate, 
            visible_checker.system()
            .after(PostUpdateLabels::SendEntityUpdates)
            .label(PostUpdateLabels::VisibleChecker)
        )
        .add_system_to_stage(PostUpdate, 
            net_send_messages_event.system()
            .after(PostUpdateLabels::VisibleChecker)
        )
        .run();
}
