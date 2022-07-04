use bevy::{
    app::ScheduleRunnerPlugin,
    asset::AssetPlugin,
    core::CorePlugin,
    log::LogPlugin,
    prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemLabel},
    render::{settings::WgpuSettings, RenderPlugin},
    scene::ScenePlugin,
    transform::TransformPlugin,
    window::WindowPlugin,
};
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

use crate::{
    core::{
        atmospherics::plugin::AtmosphericsPlugin, chat::plugin::ChatPlugin,
        combat::plugin::CombatPlugin, configuration::plugin::ConfigurationPlugin,
        connected_player::plugin::ConnectedPlayerPlugin,
        console_commands::plugins::ConsoleCommandsPlugin, entity::plugin::EntityPlugin,
        gridmap::plugin::GridmapPlugin, health::plugin::HealthPlugin,
        humanoid::plugin::HumanoidPlugin, inventory::plugin::InventoryPlugin,
        inventory_item::plugin::InventoryItemPlugin, map::plugin::MapPlugin,
        networking::plugin::NetworkingPlugin, pawn::plugin::PawnPlugin,
        physics::plugin::PhysicsPlugin, rigid_body::plugin::RigidBodyPlugin,
        senser::plugin::SenserPlugin, sfx::plugin::SfxPlugin,
        tab_actions::plugin::TabActionsPlugin, world_environment::plugin::WorldEnvironmentPlugin,
    },
    entities::{
        air_locks::plugin::AirLocksPlugin,
        asana::plugin::AsanaPlugin,
        computers::plugin::ComputersPlugin,
        construction_tool_admin::plugin::ConstructionToolAdminPlugin,
        counter_windows::plugin::CounterWindowsPlugin,
        helmet_security::plugin::HelmetsPlugin,
        human_male::plugin::HumanMalePlugin,
        jumpsuit_security::plugin::JumpsuitsPlugin,
        line_arrow::plugin::{LineArrowPlugin, PointArrowPlugin},
        omni_light::plugin::OmniLightPlugin,
        pistol_l1::plugin::PistolL1Plugin,
        reflection_probe::plugin::ReflectionProbePlugin,
    },
    server_is_live,
};

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
    Net,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SummoningLabels {
    TriggerSummon,
    DefaultSummon,
    NormalSummon,
}

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;

        app.add_plugin(CorePlugin::default())
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(LogPlugin::default())
            .add_plugin(TransformPlugin::default())
            .insert_resource(wgpu_settings)
            .add_plugin(WindowPlugin {
                add_primary_window: false,
                exit_on_close: false,
            })
            .add_plugin(AssetPlugin)
            .add_plugin(ScenePlugin)
            .add_plugin(RenderPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(ConfigurationPlugin)
            .add_plugin(ConnectedPlayerPlugin)
            .add_plugin(AsanaPlugin)
            .add_plugin(WorldEnvironmentPlugin)
            .add_plugin(GridmapPlugin)
            .add_plugin(PawnPlugin)
            .add_plugin(HumanMalePlugin)
            .add_plugin(SfxPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(EntityPlugin)
            .add_plugin(AtmosphericsPlugin)
            .add_plugin(ConsoleCommandsPlugin)
            .add_plugin(ConstructionToolAdminPlugin)
            .add_plugin(TabActionsPlugin)
            .add_plugin(ChatPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(AirLocksPlugin)
            .add_plugin(CounterWindowsPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(NetworkingPlugin)
            .add_plugin(LivePlugin)
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
            .add_plugin(LineArrowPlugin)
            .add_plugin(PointArrowPlugin);
    }
}

pub struct LivePlugin;

impl Plugin for LivePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            server_is_live
                .label(StartupLabels::ServerIsLive)
                .after(StartupLabels::InitAtmospherics),
        );
    }
}
