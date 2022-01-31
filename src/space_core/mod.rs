pub mod resources;
pub mod functions;
pub mod systems;
pub mod components;
pub mod events;
pub mod bundles;

use bevy::{MinimalPlugins, core::{FixedTimestep}, log::LogPlugin, prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemLabel, SystemSet}, transform::TransformPlugin};
use bevy_networking_turbulence::NetworkingPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin, Real};

use crate::space_core::{events::{general::{attack::Attack, boarding_player::BoardingPlayer, build_graphics::InputBuildGraphics, console_command::InputConsoleCommand, drop_current_item::InputDropCurrentItem, examine_entity::InputExamineEntity, examine_map::InputExamineMap, input_alt_item_attack::InputAltItemAttack, input_attack_cell::InputAttackCell, input_attack_entity::InputAttackEntity, input_chat_message::InputChatMessage, input_mouse_action::InputMouseAction, input_select_body_part::InputSelectBodyPart, input_sprinting::InputSprinting, input_tab_action::InputTabAction, input_throw_item::InputThrowItem, input_toggle_auto_move::InputToggleAutoMove, input_toggle_combat_mode::InputToggleCombatMode, input_user_name::InputUserName, mouse_direction_update::InputMouseDirectionUpdate, movement_input::InputMovementInput, projectile_fov::ProjectileFOV, scene_ready::InputSceneReady, switch_hands::InputSwitchHands, tab_data_entity::InputTabDataEntity, tab_data_map::InputTabDataMap, take_off_item::InputTakeOffItem, ui_input::InputUIInput, ui_input_transmit_text::InputUIInputTransmitText, use_world_item::InputUseWorldItem, wear_item::InputWearItem}, net::{net_chat_message::NetChatMessage, net_console_commands::NetConsoleCommands, net_done_boarding::NetDoneBoarding, net_drop_current_item::NetDropCurrentItem, net_examine_entity::NetExamineEntity, net_health_update::NetHealthUpdate, net_load_entity::NetLoadEntity, net_on_boarding::NetOnBoarding, net_on_new_player_connection::NetOnNewPlayerConnection, net_on_setupui::NetOnSetupUI, net_on_spawning::NetOnSpawning, net_pickup_world_item::NetPickupWorldItem, net_projectile_fov::NetProjectileFOV, net_send_entity_updates::NetSendEntityUpdates, net_send_server_time::NetSendServerTime, net_send_world_environment::NetSendWorldEnvironment, net_showcase::NetShowcase, net_switch_hands::NetSwitchHands, net_tab_data_entity::NetTabData, net_takeoff_item::NetTakeOffItem, net_throw_item::NetThrowItem, net_ui_input_transmit_data::NetUIInputTransmitData, net_unload_entity::NetUnloadEntity, net_update_player_count::NetUpdatePlayerCount, net_user_name::NetUserName, net_wear_item::NetWearItem}, physics::{air_lock_collision::AirLockCollision, counter_window_sensor_collision::CounterWindowSensorCollision}}, resources::{asana_boarding_announcements::AsanaBoardingAnnouncements, authid_i::AuthidI, client_health_ui_cache::ClientHealthUICache, doryen_fov::DoryenMap, entity_data_resource::EntityDataResource, gridmap_data::GridmapData, gridmap_details1::GridmapDetails1, gridmap_main::GridmapMain, handle_to_entity::HandleToEntity, motd::MOTD, server_id::ServerId, sfx_auto_destroy_timers::SfxAutoDestroyTimers, spawn_points::SpawnPoints, tick_rate::TickRate, used_names::UsedNames, world_environments::WorldEnvironment, y_axis_rotations::PlayerYAxisRotations}};

use self::{systems::{entity_updates::{air_lock_update::air_lock_update, counter_window_update::counter_window_update, gi_probe_update::gi_probe_update, health_ui_update::health_ui_update, inventory_item_update::inventory_item_update, inventory_update::inventory_update, omni_light_update::omni_light_update, reflection_probe_update::reflection_probe_update, repeating_sfx_update::repeating_sfx_update, send_entity_updates::send_entity_updates, sfx_update::sfx_update, standard_character_update::standard_character_update, world_mode_update::world_mode_update}, general::{air_lock::air_lock_events, attack::attack, broadcast_interpolation_transforms::{BROADCAST_INTERPOLATION_TRANSFORM_RATE, broadcast_interpolation_transforms}, broadcast_position_updates::broadcast_position_updates, build_graphics_event::build_graphics_event, chat_message_input_event::chat_message_input_event, console_commands::console_commands, counter_window::counter_window_events, done_boarding::done_boarding, drop_current_item::drop_current_item, examine_entity::examine_entity, examine_map::examine_map, handle_network_events::handle_network_events, handle_network_messages::handle_network_messages, mouse_direction_update::mouse_direction_update, net::net_send_message_event, on_boarding::on_boarding, on_setupui::on_setupui, on_spawning::on_spawning, physics_events::physics_events, pickup_world_item::pickup_world_item, player_input_event::player_input_event, projectile_fov::projectile_fov, rigidbody_link_transform::rigidbody_link_transform, scene_ready_event::scene_ready_event, send_server_time::send_server_time, senser_update_fov::senser_update_fov, standard_characters::standard_characters, startup_build_map::startup_build_map, startup_init_entities::startup_init_entities, startup_init_gridmap_cells::startup_init_gridmap_cells, startup_init_misc_resources::startup_init_misc_resources, startup_launch_server::startup_launch_server, switch_hands::switch_hands, tab_action::tab_action, tab_data::tab_data, take_off_item::take_off_item, throw_item::throw_item, tick_asana_boarding_announcements::tick_asana_boarding_announcements, tick_timers::tick_timers, tick_timers_slowed::tick_timers_slowed, toggle_combat_mode::toggle_combat_mode, ui_input_event::ui_input_event, ui_input_transmit_data_event::ui_input_transmit_data_event, update_player_count::update_player_count, user_name::user_name, visible_checker::visible_checker, wear_item::wear_item, inventory_tab_data::inventory_tab_data, construction_tool::construction_tool, text_tree_input_selection::text_tree_input_selection, remove_cell::remove_cell, out_of_bounds_check::out_of_bounds_check}}, events::{general::{input_construct::InputConstruct, input_deconstruct::InputDeconstruct, input_construction_options::InputConstructionOptions, input_construction_options_selection::InputConstructionOptionsSelection, text_tree_input_selection::TextTreeInputSelection, remove_cell::RemoveCell}, net::{net_construction_tool::NetConstructionTool, net_remove_cell::NetRemoveCell}}};
use bevy::app::CoreStage::{PostUpdate,PreUpdate};

pub struct SpaceCore;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum StartupLabels {
    Launch,
    InitDefaultGridmapData,
    BuildGridmap,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PreUpdateLabels {
    NetEvents
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum UpdateLabels {
    ProcessMovementInput,
    DropCurrentItem,
    StandardCharacters,
    TextTreeInputSelection,
    DeconStructCell
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PostUpdateLabels {
    EntityUpdate,
    SendEntityUpdates,
    VisibleChecker,

}

const INTERPOLATION_LABEL: &str = "fixed_timestep_interpolation";
const INTERPOLATION_LABEL1: &str = "fixed_timestep_interpolation1";

pub const RAPIER_DT : Real = 1. / 24.;

impl Plugin for SpaceCore {
    fn build(&self, app: &mut App) {

        app
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(NetworkingPlugin {
            idle_timeout_ms: Some(40000),
            ..Default::default()
        })
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .init_resource::<WorldEnvironment>()
        .init_resource::<GridmapData>()
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
        .init_resource::<EntityDataResource>()
        .add_event::<InputUIInput>()
        .add_event::<InputSceneReady>()
        .add_event::<InputUIInputTransmitText>()
        .add_event::<InputMovementInput>()
        .add_event::<InputBuildGraphics>()
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
        .add_event::<InputExamineEntity>()
        .add_event::<InputExamineMap>()
        .add_event::<InputUseWorldItem>()
        .add_event::<NetPickupWorldItem>()
        .add_event::<InputDropCurrentItem>()
        .add_event::<NetDropCurrentItem>()
        .add_event::<InputSwitchHands>()
        .add_event::<NetSwitchHands>()
        .add_event::<InputWearItem>()
        .add_event::<NetWearItem>()
        .add_event::<InputTakeOffItem>()
        .add_event::<NetTakeOffItem>()
        .add_event::<NetShowcase>()
        .add_event::<InputConsoleCommand>()
        .add_event::<NetConsoleCommands>()
        .add_event::<InputToggleCombatMode>()
        .add_event::<InputMouseDirectionUpdate>()
        .add_event::<InputMouseAction>()
        .add_event::<InputSelectBodyPart>()
        .add_event::<InputToggleAutoMove>()
        .add_event::<InputUserName>()
        .add_event::<NetUserName>()
        .add_event::<NetUIInputTransmitData>()
        .add_event::<NetHealthUpdate>()
        .add_event::<NetExamineEntity>()
        .add_event::<Attack>()
        .add_event::<NetProjectileFOV>()
        .add_event::<ProjectileFOV>()
        .add_event::<InputAttackEntity>()
        .add_event::<InputAltItemAttack>()
        .add_event::<InputThrowItem>()
        .add_event::<NetThrowItem>()
        .add_event::<InputAttackCell>()
        .add_event::<InputTabDataEntity>()
        .add_event::<InputTabDataMap>()
        .add_event::<NetTabData>()
        .add_event::<InputTabAction>()
        .add_event::<NetSendServerTime>()
        .add_event::<NetUpdatePlayerCount>()
        .add_event::<InputConstruct>()
        .add_event::<InputDeconstruct>()
        .add_event::<InputConstructionOptions>()
        .add_event::<NetConstructionTool>()
        .add_event::<InputConstructionOptionsSelection>()
        .add_event::<TextTreeInputSelection>()
        .add_event::<RemoveCell>()
        .add_event::<NetRemoveCell>()
        .add_startup_system(startup_init_misc_resources.label(StartupLabels::Launch))
        .add_startup_system(startup_init_gridmap_cells.label(StartupLabels::InitDefaultGridmapData).after(StartupLabels::Launch))
        .add_startup_system(startup_init_entities.before(StartupLabels::BuildGridmap))
        .add_startup_system(startup_build_map.label(StartupLabels::BuildGridmap).after(StartupLabels::InitDefaultGridmapData))
        .add_startup_system(startup_launch_server.after(StartupLabels::BuildGridmap))
        .add_system_to_stage(PreUpdate, 
            handle_network_events
            .label(PreUpdateLabels::NetEvents)
        )
        .add_system_to_stage(PreUpdate, 
            handle_network_messages
            .after(PreUpdateLabels::NetEvents)
        )
        .add_system(ui_input_event)
        .add_system(scene_ready_event)
        .add_system(on_boarding)
        .add_system(on_setupui)
        .add_system(build_graphics_event)
        .add_system(chat_message_input_event)
        .add_system(physics_events)
        .add_system(air_lock_events)
        .add_system(counter_window_events)
        .add_system(tick_timers)
        .add_system(tick_asana_boarding_announcements)
        .add_system(ui_input_transmit_data_event)
        .add_system(done_boarding)
        .add_system(on_spawning)
        .add_system(examine_entity)
        .add_system(examine_map)
        .add_system(pickup_world_item)
        .add_system(switch_hands)
        .add_system(wear_item)
        .add_system(take_off_item)
        .add_system(console_commands)
        .add_system(senser_update_fov)
        .add_system(toggle_combat_mode)
        .add_system(user_name)
        .add_system(projectile_fov)
        .add_system(throw_item)
        .add_system(tab_data)
        .add_system(tab_action)
        .add_system(inventory_tab_data)
        .add_system(remove_cell.label(UpdateLabels::DeconStructCell))
        .add_system(text_tree_input_selection.label(UpdateLabels::TextTreeInputSelection))
        .add_system(construction_tool.after(UpdateLabels::TextTreeInputSelection).before(UpdateLabels::DeconStructCell))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./2.)
                .with_label(INTERPOLATION_LABEL1))
                .with_system(broadcast_position_updates)
                .with_system(tick_timers_slowed)
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(3.))
            .with_system(send_server_time)
            .with_system(update_player_count)
            .with_system(out_of_bounds_check)
        )
        .add_system(drop_current_item.label(UpdateLabels::DropCurrentItem))
        .add_system(rigidbody_link_transform.after(UpdateLabels::DropCurrentItem))
        .add_system(player_input_event.label(UpdateLabels::ProcessMovementInput))
        .add_system(mouse_direction_update.before(UpdateLabels::StandardCharacters))
        .add_system(standard_characters.label(UpdateLabels::StandardCharacters).after(UpdateLabels::ProcessMovementInput))
        .add_system(attack.after(UpdateLabels::StandardCharacters))
        .add_system(broadcast_interpolation_transforms
            .with_run_criteria(FixedTimestep::step(1./BROADCAST_INTERPOLATION_TRANSFORM_RATE)
            .with_label(INTERPOLATION_LABEL))
        )
        .add_system_set_to_stage(PostUpdate, 
            SystemSet::new()
            .label(PostUpdateLabels::EntityUpdate)
            .with_system(omni_light_update)
            .with_system(standard_character_update)
            .with_system(world_mode_update)
            .with_system(gi_probe_update)
            .with_system(reflection_probe_update)
            .with_system(air_lock_update)
            .with_system(sfx_update)
            .with_system(repeating_sfx_update)
            .with_system(counter_window_update)
            .with_system(inventory_update)
            .with_system(inventory_item_update)
            .with_system(health_ui_update)
        )
        .add_system_to_stage(PostUpdate, 
            send_entity_updates
            .after(PostUpdateLabels::EntityUpdate)
            .label(PostUpdateLabels::SendEntityUpdates)
        )
        .add_system_to_stage(PostUpdate, 
            visible_checker
            .after(PostUpdateLabels::SendEntityUpdates)
            .label(PostUpdateLabels::VisibleChecker)
        )
        .add_system_to_stage(PostUpdate, 
            net_send_message_event
            .after(PostUpdateLabels::VisibleChecker)
        );
    }
    
}
