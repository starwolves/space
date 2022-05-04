use bevy::DefaultPlugins;
use bevy_app::{App, Plugin};
use bevy_networking_turbulence::NetworkingPlugin as TBNetworkingPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

use super::{
    super::{
        core::{
            artificial_unintelligence::ArtificialUnintelligencePlugin, asana::AsanaPlugin,
            atmospherics::AtmosphericsPlugin, chat::ChatPlugin, combat::CombatPlugin,
            configuration::ConfigurationPlugin, connected_player::ConnectedPlayerPlugin,
            console_commands::ConsoleCommandsPlugin, entity::EntityPlugin, gridmap::GridmapPlugin,
            health::HealthPlugin, inventory::InventoryPlugin, inventory_item::InventoryItemPlugin,
            map::MapPlugin, networking::NetworkingPlugin, pawn::PawnPlugin,
            physics::systems::PhysicsPlugin, rigid_body::systems::RigidBodyPlugin,
            senser::SenserPlugin, sfx::SfxPlugin, tab_actions::TabActionsPlugin,
            world_environment::WorldEnvironmentPlugin, CorePlugin,
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
    },
    humanoid::HumanoidPlugin,
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
        app //.add_plugin(BCorePlugin::default())
            //.add_plugin(ScheduleRunnerPlugin::default())
            //.add_plugin(LogPlugin::default())
            //.add_plugin(TransformPlugin::default())
            .add_plugins(DefaultPlugins)
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
