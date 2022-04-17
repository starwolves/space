pub mod core;
pub mod entities;

use bevy_app::{
    App,
    CoreStage::{PostUpdate, PreUpdate},
    Plugin, ScheduleRunnerPlugin,
};
use bevy_core::{CorePlugin, FixedTimestep};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemLabel, SystemSet};
use bevy_log::LogPlugin;
use bevy_networking_turbulence::NetworkingPlugin;
use bevy_rapier3d::{
    physics::{PhysicsStages, PhysicsSystems},
    prelude::{NoUserData, RapierPhysicsPlugin},
};
use bevy_transform::TransformPlugin;

use self::{
    core::{
        asana::{
            resources::AsanaBoardingAnnouncements,
            systems::tick_asana_boarding_announcements::tick_asana_boarding_announcements,
        },
        atmospherics::{
            events::{NetAtmosphericsNotices, NetMapDisplayAtmospherics, NetMapHoverAtmospherics},
            resources::{AtmosphericsResource, MapHolders, RigidBodyForcesAccumulation},
            startup_atmospherics,
            systems::{
                diffusion::{atmos_diffusion, DIFFUSION_STEP},
                effects::atmos_effects,
                map::atmospherics_map,
                map_hover::atmospherics_map_hover,
                notices::atmospherics_notices,
                rigidbody_forces_atmospherics::rigidbody_forces_accumulation,
                rigidbody_forces_physics::rigidbody_forces_physics,
                sensing_ability::atmospherics_sensing_ability,
                zero_gravity::zero_gravity,
            },
        },
        chat::{
            events::{InputChatMessage, NetChatMessage},
            systems::chat_message_input_event,
        },
        combat::systems::attack,
        configuration::resources::{ServerId, TickRate, MOTD},
        connected_player::{
            events::{
                BoardingPlayer, InputAltItemAttack, InputAttackCell, InputAttackEntity,
                InputBuildGraphics, InputExamineEntity, InputExamineMap, InputMouseAction,
                InputMouseDirectionUpdate, InputMovementInput, InputSceneReady,
                InputSelectBodyPart, InputSprinting, InputTabDataEntity, InputTabDataMap,
                InputToggleAutoMove, InputToggleCombatMode, InputUIInput, InputUIInputTransmitText,
                InputUserName, NetDoneBoarding, NetExamineEntity, NetOnBoarding,
                NetOnNewPlayerConnection, NetOnSetupUI, NetOnSpawning, NetSendServerTime,
                NetSendWorldEnvironment, NetTabData, NetUIInputTransmitData, NetUpdatePlayerCount,
                NetUserName, TextTreeInputSelection,
            },
            resources::HandleToEntity,
            systems::{
                build_graphics_event::build_graphics_event, controller_input::controller_input,
                done_boarding::done_boarding, examine_entity::examine_entity,
                examine_map::examine_map, on_boarding::on_boarding, on_setupui::on_setupui,
                player_input_event::player_input_event, scene_ready_event::scene_ready_event,
                send_server_time::send_server_time,
                text_tree_input_selection::text_tree_input_selection,
                ui_input_event::ui_input_event,
                ui_input_transmit_data_event::ui_input_transmit_data_event,
                update_player_count::update_player_count, on_spawning::on_spawning, mouse_direction_update::mouse_direction_update,
            },
        },
        console_commands::{
            events::{InputConsoleCommand, NetConsoleCommands},
            systems::console_commands,
        },
        entity::{
            events::{NetLoadEntity, NetSendEntityUpdates, NetShowcase, NetUnloadEntity},
            resources::EntityDataResource,
            startup_entities,
            systems::{
                broadcast_position_updates::{broadcast_position_updates, INTERPOLATION_LABEL1},
                send_entity_updates::send_entity_updates,
            },
        },
        gridmap::{
            events::{NetGridmapUpdates, NetProjectileFOV, ProjectileFOV, RemoveCell},
            resources::{DoryenMap, GridmapData, GridmapDetails1, GridmapMain, SpawnPoints},
            startup_build_map, startup_map_cells, startup_misc_resources,
            systems::{
                gridmap_updates::gridmap_updates, projectile_fov::projectile_fov,
                remove_cell::remove_cell, senser_update_fov::senser_update_fov,
            },
        },
        health::{
            entity_update::health_ui_update,
            events::{Attack, NetHealthUpdate},
            resources::ClientHealthUICache,
        },
        humanoid::{
            entity_update::humanoid_update,
            systems::{humanoid::humanoids, toggle_combat_mode::toggle_combat_mode},
        },
        inventory::{
            entity_update::inventory_update,
            events::{
                InputDropCurrentItem, InputSwitchHands, InputTakeOffItem, InputThrowItem,
                InputUseWorldItem, InputWearItem, NetDropCurrentItem, NetPickupWorldItem,
                NetSwitchHands, NetTakeOffItem, NetThrowItem, NetWearItem,
            },
            systems::{
                drop_current_item::drop_current_item, inventory_tab_data::inventory_tab_data,
                pickup_world_item::pickup_world_item, switch_hands::switch_hands,
                take_off_item::take_off_item, throw_item::throw_item, wear_item::wear_item,
            },
        },
        inventory_item::entity_update::inventory_item_update,
        map::{
            events::{
                InputMap, InputMapChangeDisplayMode, InputMapRequestDisplayModes,
                NetRequestDisplayModes,
            },
            resources::MapData,
            systems::{
                change_display_mode::change_display_mode, map_input::map_input,
                request_display_modes::request_display_modes,
            },
        },
        networking::{
            connections, messages_outgoing, net_send_message_event, startup_listen_connections,
        },
        pawn::{
            resources::{AuthidI, UsedNames},
            systems::{
                user_name::user_name,
            },
        },
        physics::{entity_update::world_mode_update, systems::physics_events},
        rigid_body::systems::{
            broadcast_interpolation_transforms::broadcast_interpolation_transforms,
            out_of_bounds_check::out_of_bounds_check,
            rigidbody_link_transform::rigidbody_link_transform,
        },
        senser::systems::visible_checker::visible_checker,
        server_is_live,
        sfx::{
            entity_update::{repeating_sfx_update, sfx_update},
            resources::SfxAutoDestroyTimers,
            systems::tick_timers_slowed,
        },
        tab_actions::{
            events::InputTabAction,
            systems::{tab_action::tab_action, tab_data::tab_data},
        },
        world_environment::resources::WorldEnvironment,
    },
    entities::{
        air_locks::{
            entity_update::air_lock_update,
            events::AirLockCollision,
            systems::{
                air_lock_added, air_lock_default_map_added, air_lock_events, air_lock_tick_timers,
            },
        },
        computers::systems::computer_added,
        construction_tool_admin::{
            events::{
                InputConstruct, InputConstructionOptions, InputConstructionOptionsSelection,
                InputDeconstruct, NetConstructionTool,
            },
            systems::construction_tool,
        },
        counter_windows::{
            entity_update::counter_window_update,
            events::CounterWindowSensorCollision,
            systems::{
                counter_window_added, counter_window_default_map_added, counter_window_events,
                counter_window_tick_timers,
            },
        },
        gi_probe::entity_update::gi_probe_update,
        omni_light::entity_update::omni_light_update,
        reflection_probe::entity_update::reflection_probe_update,
    },
};

pub struct SpacePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum StartupLabels {
    Launch,
    InitDefaultGridmapData,
    BuildGridmap,
    InitAtmospherics,
    ListenConnections,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum MapLabels {
    ChangeMode,
    MapInput,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PreUpdateLabels {
    NetEvents,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum UpdateLabels {
    ProcessMovementInput,
    DropCurrentItem,
    StandardCharacters,
    TextTreeInputSelection,
    DeconstructCell,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum AtmosphericsLabels {
    Diffusion,
    Effects,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PostUpdateLabels {
    EntityUpdate,
    SendEntityUpdates,
    VisibleChecker,
}

const ATMOS_LABEL: &str = "fixed_timestep_map_atmos";
const ATMOS_DIFFUSION_LABEL: &str = "fixed_timestep_atmos";

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CorePlugin::default())
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(LogPlugin::default())
            .add_plugin(TransformPlugin::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(NetworkingPlugin {
                idle_timeout_ms: Some(40000),
                ..Default::default()
            })
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
            .init_resource::<SfxAutoDestroyTimers>()
            .init_resource::<AsanaBoardingAnnouncements>()
            .init_resource::<DoryenMap>()
            .init_resource::<MOTD>()
            .init_resource::<ClientHealthUICache>()
            .init_resource::<EntityDataResource>()
            .init_resource::<AtmosphericsResource>()
            .init_resource::<MapHolders>()
            .init_resource::<RigidBodyForcesAccumulation>()
            .init_resource::<MapData>()
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
            .add_event::<NetGridmapUpdates>()
            .add_event::<InputMapChangeDisplayMode>()
            .add_event::<InputMapRequestDisplayModes>()
            .add_event::<NetRequestDisplayModes>()
            .add_event::<NetMapDisplayAtmospherics>()
            .add_event::<InputMap>()
            .add_event::<NetMapHoverAtmospherics>()
            .add_event::<NetAtmosphericsNotices>()
            .add_startup_system(startup_misc_resources.label(StartupLabels::Launch))
            .add_startup_system(
                startup_map_cells
                    .label(StartupLabels::InitDefaultGridmapData)
                    .after(StartupLabels::Launch),
            )
            .add_startup_system(startup_entities.before(StartupLabels::BuildGridmap))
            .add_startup_system(
                startup_build_map
                    .label(StartupLabels::BuildGridmap)
                    .after(StartupLabels::InitDefaultGridmapData),
            )
            .add_startup_system(
                startup_atmospherics
                    .label(StartupLabels::InitAtmospherics)
                    .after(StartupLabels::BuildGridmap),
            )
            .add_startup_system(
                startup_listen_connections
                    .label(StartupLabels::ListenConnections)
                    .after(StartupLabels::InitAtmospherics),
            )
            .add_startup_system(server_is_live.after(StartupLabels::ListenConnections))
            .add_system_to_stage(PreUpdate, connections.label(PreUpdateLabels::NetEvents))
            .add_system_to_stage(
                PreUpdate,
                messages_outgoing.after(PreUpdateLabels::NetEvents),
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
            .add_system(air_lock_tick_timers)
            .add_system(counter_window_tick_timers)
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
            .add_system(change_display_mode.label(MapLabels::ChangeMode))
            .add_system(request_display_modes)
            .add_system(map_input.label(MapLabels::ChangeMode))
            .add_system(atmospherics_map_hover.after(MapLabels::ChangeMode))
            .add_system(atmospherics_sensing_ability)
            .add_system(out_of_bounds_check)
            .add_system(air_lock_added)
            .add_system(counter_window_added)
            .add_system(counter_window_default_map_added)
            .add_system(air_lock_default_map_added)
            .add_system(computer_added)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 4.).with_label(ATMOS_LABEL))
                    .with_system(atmospherics_notices)
                    .with_system(atmospherics_map.after(MapLabels::ChangeMode)),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / DIFFUSION_STEP).with_label(ATMOS_DIFFUSION_LABEL),
                    )
                    .with_system(atmos_diffusion.label(AtmosphericsLabels::Diffusion))
                    .with_system(
                        atmos_effects
                            .after(AtmosphericsLabels::Diffusion)
                            .label(AtmosphericsLabels::Effects),
                    )
                    .with_system(rigidbody_forces_accumulation.after(AtmosphericsLabels::Effects)),
            )
            .add_system(remove_cell.label(UpdateLabels::DeconstructCell))
            .add_system(text_tree_input_selection.label(UpdateLabels::TextTreeInputSelection))
            .add_system(
                construction_tool
                    .after(UpdateLabels::TextTreeInputSelection)
                    .before(UpdateLabels::DeconstructCell),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(broadcast_position_updates)
                    .with_system(tick_timers_slowed)
                    .with_system(gridmap_updates),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 4.))
                    .with_system(gridmap_updates),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.))
                    .with_system(send_server_time)
                    .with_system(update_player_count),
            )
            .add_system(drop_current_item.label(UpdateLabels::DropCurrentItem))
            .add_system(rigidbody_link_transform.after(UpdateLabels::DropCurrentItem))
            .add_system(player_input_event.label(UpdateLabels::ProcessMovementInput))
            .add_system(mouse_direction_update.before(UpdateLabels::StandardCharacters))
            .add_system(controller_input.before(UpdateLabels::StandardCharacters))
            .add_system(
                humanoids
                    .label(UpdateLabels::StandardCharacters)
                    .before(PhysicsSystems::StepWorld)
                    .after(UpdateLabels::ProcessMovementInput),
            )
            .add_system(attack.after(UpdateLabels::StandardCharacters))
            .add_system_to_stage(
                PhysicsStages::SyncTransforms,
                broadcast_interpolation_transforms.after(PhysicsSystems::SyncTransforms),
            )
            .add_system_to_stage(
                PhysicsStages::SyncTransforms,
                rigidbody_forces_physics.after(PhysicsSystems::SyncTransforms),
            )
            .add_system_to_stage(
                PhysicsStages::SyncTransforms,
                zero_gravity.after(PhysicsSystems::SyncTransforms),
            )
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(omni_light_update)
                    .with_system(humanoid_update)
                    .with_system(world_mode_update)
                    .with_system(gi_probe_update)
                    .with_system(reflection_probe_update)
                    .with_system(air_lock_update)
                    .with_system(sfx_update)
                    .with_system(repeating_sfx_update)
                    .with_system(counter_window_update)
                    .with_system(inventory_update)
                    .with_system(inventory_item_update)
                    .with_system(health_ui_update),
            )
            .add_system_to_stage(
                PostUpdate,
                send_entity_updates
                    .after(PostUpdateLabels::EntityUpdate)
                    .label(PostUpdateLabels::SendEntityUpdates),
            )
            .add_system_to_stage(
                PostUpdate,
                visible_checker
                    .after(PostUpdateLabels::SendEntityUpdates)
                    .label(PostUpdateLabels::VisibleChecker),
            )
            .add_system_to_stage(
                PostUpdate,
                net_send_message_event.after(PostUpdateLabels::VisibleChecker),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
