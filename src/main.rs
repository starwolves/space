use std::{collections::HashMap, fs};

use bevy::{app::CoreStage::{PreUpdate, Update, PostUpdate}, core::FixedTimestep, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*, transform::TransformPlugin};

use bevy_rapier3d::{
    physics::{
        RapierPhysicsPlugin
    }
};

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
            net_visible_checker::NetVisibleChecker, 
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
            ui_input_transmit_text_event::ui_input_transmit_text_event,
            on_spawning::on_spawning,
            visible_checker::visible_checker
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

use crate::space_core::{events::general::movement_input::MovementInput, systems::{entity_updates::world_mode_update::world_mode_update, general::{move_player_bodies::move_player_bodies, movement_input_event::movement_input_event}, net::broadcast_interpolation_transforms::broadcast_interpolation_transforms}};


const DEFAULT_MAP_ENVIRONMENT_LOCATION : &str = "content\\maps\\bullseye\\environment.json";
const DEFAULT_MAP_BLACKCELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\blackcells.json";
const DEFAULT_MAP_BLOCKING_CELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\nonblockinglist.json";
const DEFAULT_MAP_MAINORDERED_CELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\mainordered.json";
const DEFAULT_MAP_DETAILS1ORDERED_CELLS_DATA_LOCATION : &str = "content\\maps\\bullseye\\details1ordered.json";
const DEFAULT_MAP_SPAWNPOINTS_LOCATION : &str = "content\\maps\\bullseye\\spawnpoints.json";

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum SpaceStages {
    SendNetMessages,
    ProcessEntityUpdates,
    SendEntityUpdates,
    TransformInterpolation
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum PreUpdateLabels {
    NetEvents
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum UpdateLabels {
    ProcessMovementInput
}

const INTERPOLATION_LABEL: &str = "fixed_timestep_interpolation";


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

    let current_map_spawn_points_raw_json : String = fs::read_to_string(&DEFAULT_MAP_SPAWNPOINTS_LOCATION).expect("main.rs launch_server() Error reading map spawnpoints.json from drive.");
    let current_map_spawn_points_raw : Vec<SpawnPointRaw> = serde_json::from_str(&current_map_spawn_points_raw_json).expect("main.rs launch_server() Error parsing map spawnpoints.json String.");
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
    
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(RapierPhysicsPlugin)
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
        .add_stage_after(
            PostUpdate, 
            SpaceStages::ProcessEntityUpdates, 
            SystemStage::parallel()
        )
        .add_stage_after(
            SpaceStages::ProcessEntityUpdates, 
            SpaceStages::SendEntityUpdates, 
            SystemStage::parallel()
        )
        .add_stage_after(
            SpaceStages::SendEntityUpdates, 
            SpaceStages::SendNetMessages, 
            SystemStage::parallel()
        )
        .add_stage_after(
            SpaceStages::SendNetMessages,
            SpaceStages::TransformInterpolation,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(1./24.)
                    .with_label(INTERPOLATION_LABEL),
                )
                .with_system(broadcast_interpolation_transforms.system()),
        )
        .add_event::<UIInput>()
        .add_event::<SceneReady>()
        .add_event::<UIInputTransmitText>()
        .add_event::<MovementInput>()
        .add_event::<NetOnNewPlayerConnection>()
        .add_event::<NetOnBoarding>()
        .add_event::<NetOnSetupUI>()
        .add_event::<NetDoneBoarding>()
        .add_event::<NetVisibleChecker>()
        .add_event::<NetSendEntityUpdates>()
        .add_startup_system(launch_server.system())
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
        .add_system(ui_input_event.system())
        .add_system(scene_ready_event.system())
        .add_system(on_boarding.system())
        .add_system(on_setupui.system())
        .add_system(ui_input_transmit_text_event.system())
        .add_system(on_spawning.system())
        .add_system(visible_checker.system())
        .add_system_to_stage(
            PreUpdate, 
            handle_network_events.system()
            .label(PreUpdateLabels::NetEvents)
        )
        .add_system_to_stage(PreUpdate, 
            handle_network_messages.system()
            .after(PreUpdateLabels::NetEvents)
        )
        .add_system_to_stage(SpaceStages::ProcessEntityUpdates, 
            omni_light_update.system()
        )
        .add_system_to_stage(SpaceStages::ProcessEntityUpdates,
            world_mode_update.system()
        )
        .add_system_to_stage(SpaceStages::SendEntityUpdates, 
            send_entity_updates.system()
        )
        .add_system_to_stage(PostUpdate, done_boarding.system())
        .add_system_to_stage(SpaceStages::SendNetMessages, net_send_messages_event.system())
        .run();
}
