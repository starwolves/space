use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;
use bevy_log::info;

pub mod artificial_unintelligence;
pub mod asana;
pub mod atmospherics;
pub mod chat;
pub mod combat;
pub mod configuration;
pub mod connected_player;
pub mod console_commands;
pub mod data_link;
pub mod entity;
pub mod examinable;
pub mod gridmap;
pub mod health;
pub mod humanoid;
pub mod inventory;
pub mod inventory_item;
pub mod map;
pub mod networking;
pub mod pawn;
pub mod physics;
pub mod rigid_body;
pub mod sensable;
pub mod senser;
pub mod sfx;
pub mod static_body;
pub mod tab_actions;
pub mod world_environment;

pub fn server_is_live() {
    info!("Live.");
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            server_is_live
                .label(StartupLabels::ServerIsLive)
                .after(StartupLabels::ListenConnections),
        );
    }
}

use bevy_app::ScheduleRunnerPlugin;
use bevy_asset::AssetPlugin;
use bevy_core::CorePlugin as BCorePlugin;
use bevy_log::LogPlugin;
use bevy_networking_turbulence::NetworkingPlugin as TBNetworkingPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_render::{settings::WgpuSettings, RenderPlugin};
use bevy_transform::TransformPlugin;
use bevy_window::WindowPlugin;

use self::{humanoid::HumanoidPlugin, physics::PhysicsPlugin};

use super::{
    core::{
        artificial_unintelligence::ArtificialUnintelligencePlugin, asana::AsanaPlugin,
        atmospherics::AtmosphericsPlugin, chat::ChatPlugin, combat::CombatPlugin,
        configuration::ConfigurationPlugin, connected_player::ConnectedPlayerPlugin,
        console_commands::ConsoleCommandsPlugin, entity::EntityPlugin, gridmap::GridmapPlugin,
        health::HealthPlugin, inventory::InventoryPlugin, inventory_item::InventoryItemPlugin,
        map::MapPlugin, networking::NetworkingPlugin, pawn::PawnPlugin,
        rigid_body::systems::RigidBodyPlugin, senser::SenserPlugin, sfx::SfxPlugin,
        tab_actions::TabActionsPlugin, world_environment::WorldEnvironmentPlugin,
    },
    entities::{
        air_locks::AirLocksPlugin,
        computers::ComputersPlugin,
        construction_tool_admin::ConstructionToolAdminPlugin,
        counter_windows::CounterWindowsPlugin,
        helmet_security::HelmetsPlugin,
        jumpsuit_security::JumpsuitsPlugin,
        line_arrow::{LineArrowPlugin, PointArrowPlugin},
        omni_light::OmniLightPlugin,
        pistol_l1::PistolL1Plugin,
        reflection_probe::ReflectionProbePlugin,
    },
};

use bevy_ecs::schedule::SystemLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum StartupLabels {
    ConsoleCommands,
    MiscResources,
    InitDefaultGridmapData,
    BuildGridmap,
    InitAtmospherics,
    ListenConnections,
    InitEntities,
    ServerIsLive,
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
pub enum PostUpdateLabels {
    EntityUpdate,
    SendEntityUpdates,
    VisibleChecker,
}

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;

        app.add_plugin(BCorePlugin::default())
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(LogPlugin::default())
            .add_plugin(TransformPlugin::default())
            .insert_resource(wgpu_settings)
            .add_plugin(WindowPlugin {
                add_primary_window: false,
                exit_on_close: false,
            })
            .add_plugin(AssetPlugin)
            .add_plugin(RenderPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(TBNetworkingPlugin {
                idle_timeout_ms: Some(40000),
                ..Default::default()
            })
            .add_plugin(ConfigurationPlugin)
            .add_plugin(ConnectedPlayerPlugin)
            .add_plugin(AsanaPlugin)
            .add_plugin(WorldEnvironmentPlugin)
            .add_plugin(GridmapPlugin)
            .add_plugin(PawnPlugin)
            .add_plugin(SfxPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(EntityPlugin)
            .add_plugin(AtmosphericsPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(ChatPlugin)
            .add_plugin(AirLocksPlugin)
            .add_plugin(CounterWindowsPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(ConsoleCommandsPlugin)
            .add_plugin(TabActionsPlugin)
            .add_plugin(ConstructionToolAdminPlugin)
            .add_plugin(NetworkingPlugin)
            .add_plugin(CorePlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(HumanoidPlugin)
            .add_plugin(RigidBodyPlugin)
            .add_plugin(ComputersPlugin)
            .add_plugin(CombatPlugin)
            .add_plugin(OmniLightPlugin)
            .add_plugin(ReflectionProbePlugin)
            .add_plugin(InventoryItemPlugin)
            .add_plugin(SenserPlugin)
            .add_plugin(JumpsuitsPlugin)
            .add_plugin(HelmetsPlugin)
            .add_plugin(PistolL1Plugin)
            .add_plugin(ArtificialUnintelligencePlugin)
            .add_plugin(LineArrowPlugin)
            .add_plugin(PointArrowPlugin);
    }
}
