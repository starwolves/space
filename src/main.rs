

use bevy::{app::CoreStage::{PreUpdate, PostUpdate}, core::FixedTimestep, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*, transform::TransformPlugin};

use bevy_rapier3d::{physics::{
        RapierPhysicsPlugin
    }, prelude::NoUserData};

use bevy_networking_turbulence::{NetworkingPlugin};

mod space_core;

use space_core::{events::{general::{input_mouse_action::InputMouseAction, input_user_name::InputUserName, input_select_body_part::InputSelectBodyPart, input_toggle_auto_move::InputToggleAutoMove, input_toggle_combat_mode::InputToggleCombatMode, mouse_direction_update::MouseDirectionUpdate, scene_ready::SceneReady, ui_input::UIInput, ui_input_transmit_text::UIInputTransmitText}, net::{net_done_boarding::NetDoneBoarding, net_health_update::NetHealthUpdate, net_load_entity::NetLoadEntity, net_on_boarding::NetOnBoarding, net_on_new_player_connection::NetOnNewPlayerConnection, net_on_setupui::NetOnSetupUI, net_send_entity_updates::NetSendEntityUpdates, net_ui_input_transmit_data::NetUIInputTransmitData, net_user_name::NetUserName}}, resources::{all_ordered_cells::AllOrderedCells, authid_i::AuthidI, blackcells_data::BlackcellsData, client_health_ui_cache::ClientHealthUICache, doryen_fov::{DoryenMap}, handle_to_entity::HandleToEntity, motd::MOTD, non_blocking_cells_list::NonBlockingCellsList, server_id::ServerId, spawn_points::{SpawnPoints}, tick_rate::TickRate, used_names::UsedNames, world_environments::{WorldEnvironment}}, systems::{entity_updates::{health_ui_update::health_ui_update, omni_light_update::omni_light_update, send_entity_updates::send_entity_updates}, general::{broadcast_interpolation_transforms::BROADCAST_INTERPOLATION_TRANSFORM_RATE, done_boarding::done_boarding, mouse_direction_update::mouse_direction_update, on_boarding::on_boarding, on_setupui::on_setupui, on_spawning::on_spawning, user_name::user_name, scene_ready_event::scene_ready_event, toggle_combat_mode::toggle_combat_mode, ui_input_event::ui_input_event, ui_input_transmit_data_event::ui_input_transmit_data_event, visible_checker::visible_checker}}};

use crate::space_core::{events::{general::{boarding_player::BoardingPlayer, build_graphics::BuildGraphics, console_command::ConsoleCommand, drop_current_item::DropCurrentItem, examine_entity::ExamineEntity, examine_map::ExamineMap, input_chat_message::InputChatMessage, input_sprinting::InputSprinting, movement_input::MovementInput, switch_hands::SwitchHands, take_off_item::TakeOffItem, use_world_item::UseWorldItem, wear_item::WearItem}, net::{net_chat_message::NetChatMessage, net_console_commands::NetConsoleCommands, net_drop_current_item::NetDropCurrentItem, net_on_spawning::NetOnSpawning, net_pickup_world_item::NetPickupWorldItem, net_send_world_environment::NetSendWorldEnvironment, net_showcase::NetShowcase, net_switch_hands::NetSwitchHands, net_takeoff_item::NetTakeOffItem, net_unload_entity::NetUnloadEntity, net_wear_item::NetWearItem}, physics::{air_lock_collision::AirLockCollision, counter_window_sensor_collision::CounterWindowSensorCollision}}, resources::{asana_boarding_announcements::AsanaBoardingAnnouncements, gridmap_details1::GridmapDetails1, gridmap_main::GridmapMain, sfx_auto_destroy_timers::SfxAutoDestroyTimers, y_axis_rotations::PlayerYAxisRotations}, systems::{entity_updates::{air_lock_update::air_lock_update, counter_window_update::counter_window_update, gi_probe_update::gi_probe_update, inventory_item_update::inventory_item_update, inventory_update::inventory_update, reflection_probe_update::reflection_probe_update, repeating_sfx_update::repeating_sfx_update, sfx_update::sfx_update, standard_character_update::standard_character_update, world_mode_update::world_mode_update}, general::{air_lock::air_lock_events, broadcast_interpolation_transforms::broadcast_interpolation_transforms, broadcast_position_updates::broadcast_position_updates, build_graphics_event::build_graphics_event, chat_message_input_event::chat_message_input_event, console_commands::console_commands, counter_window::counter_window_events, drop_current_item::drop_current_item, examine_entity::examine_entity, examine_map::examine_map, handle_network_events::handle_network_events, handle_network_messages::handle_network_messages, launch_server::launch_server, standard_characters::move_standard_characters, net_send_message_event::net_send_message_event, physics_events::physics_events, pickup_world_item::pickup_world_item, player_input_event::player_input_event, rigidbody_link_transform::rigidbody_link_transform, senser_update_fov::senser_update_fov, switch_hands::switch_hands, take_off_item::take_off_item, tick_asana_boarding_announcements::tick_asana_boarding_announcements, tick_timers::tick_timers, tick_timers_slowed::tick_timers_slowed, wear_item::wear_item}}};


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum PreUpdateLabels {
    NetEvents
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum UpdateLabels {
    ProcessMovementInput,
    DropCurrentItem,
    MoveStandardCharacters,
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

    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(NetworkingPlugin {
            idle_timeout_ms: Some(4000),
            ..Default::default()
        })
        .add_plugin(DiagnosticsPlugin::default())
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .init_resource::<WorldEnvironment>()
        .init_resource::<BlackcellsData>()
        .init_resource::<NonBlockingCellsList>()
        .init_resource::<AllOrderedCells>()
        .init_resource::<SpawnPoints>()
        .init_resource::<TickRate>()
        .init_resource::<GridmapMain>()
        .init_resource::<GridmapDetails1>()
        .init_resource::<AuthidI>()
        .init_resource::<ServerId>()
        .init_resource::<UsedNames>()
        .init_resource::<HandleToEntity>()
        .init_resource::<PlayerYAxisRotations>()
        .init_resource::<SfxAutoDestroyTimers>()
        .init_resource::<AsanaBoardingAnnouncements>()
        .init_resource::<DoryenMap>()
        .init_resource::<MOTD>()
        .init_resource::<ClientHealthUICache>()
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
        .add_event::<InputSprinting>()
        .add_event::<ExamineEntity>()
        .add_event::<ExamineMap>()
        .add_event::<UseWorldItem>()
        .add_event::<NetPickupWorldItem>()
        .add_event::<DropCurrentItem>()
        .add_event::<NetDropCurrentItem>()
        .add_event::<SwitchHands>()
        .add_event::<NetSwitchHands>()
        .add_event::<WearItem>()
        .add_event::<NetWearItem>()
        .add_event::<TakeOffItem>()
        .add_event::<NetTakeOffItem>()
        .add_event::<NetShowcase>()
        .add_event::<ConsoleCommand>()
        .add_event::<NetConsoleCommands>()
        .add_event::<InputToggleCombatMode>()
        .add_event::<MouseDirectionUpdate>()
        .add_event::<InputMouseAction>()
        .add_event::<InputSelectBodyPart>()
        .add_event::<InputToggleAutoMove>()
        .add_event::<InputUserName>()
        .add_event::<NetUserName>()
        .add_event::<NetUIInputTransmitData>()
        .add_event::<NetHealthUpdate>()
        .add_startup_system(launch_server.system())
        .add_system_to_stage(PreUpdate, 
            handle_network_events.system()
            .label(PreUpdateLabels::NetEvents)
        )
        .add_system_to_stage(PreUpdate, 
            handle_network_messages.system()
            .after(PreUpdateLabels::NetEvents)
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
        .add_system(ui_input_transmit_data_event.system())
        .add_system(done_boarding.system())
        .add_system(on_spawning.system())
        .add_system(examine_entity.system())
        .add_system(examine_map.system())
        .add_system(pickup_world_item.system())
        .add_system(switch_hands.system())
        .add_system(wear_item.system())
        .add_system(take_off_item.system())
        .add_system(console_commands.system())
        .add_system(senser_update_fov.system())
        .add_system(toggle_combat_mode.system())
        .add_system(user_name.system())
        .add_system(drop_current_item.system().label(UpdateLabels::DropCurrentItem))
        .add_system(rigidbody_link_transform.system().after(UpdateLabels::DropCurrentItem))
        .add_system(player_input_event.system().label(UpdateLabels::ProcessMovementInput))
        .add_system(mouse_direction_update.system().before(UpdateLabels::MoveStandardCharacters))
        .add_system(move_standard_characters.system().label(UpdateLabels::MoveStandardCharacters).after(UpdateLabels::ProcessMovementInput))
        .add_system(broadcast_interpolation_transforms.system()
            .with_run_criteria(FixedTimestep::step(1./BROADCAST_INTERPOLATION_TRANSFORM_RATE)
            .with_label(INTERPOLATION_LABEL))
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
            .with_system(inventory_update.system())
            .with_system(inventory_item_update.system())
            .with_system(health_ui_update.system())
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
            net_send_message_event.system()
            .after(PostUpdateLabels::VisibleChecker)
        )
        .run();
}
