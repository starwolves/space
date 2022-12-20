use actions::plugin::ActionsPlugin;
use air_locks::plugin::AirLocksPlugin;
use asana::plugin::AsanaPlugin;
use atmospherics::plugin::AtmosphericsPlugin;
use basic_console_commands::plugin::BasicConsoleCommandsPlugin;
use bevy::{
    app::ScheduleRunnerPlugin,
    core::CorePlugin,
    diagnostic::DiagnosticsPlugin,
    log::LogPlugin,
    prelude::{
        App, AssetPlugin, HierarchyPlugin, ImagePlugin, IntoSystemDescriptor, Plugin,
        TaskPoolOptions,
    },
    render::{settings::WgpuSettings, RenderPlugin},
    scene::ScenePlugin,
    time::TimePlugin,
    transform::TransformPlugin,
    window::WindowPlugin,
};
use chat::plugin::ChatPlugin;
use combat::plugin::CombatPlugin;
use computers::plugin::ComputersPlugin;
use console_commands::plugins::ConsoleCommandsPlugin;
use construction_tool_admin::plugin::ConstructionToolAdminPlugin;
use controller::plugin::ControllerPlugin;
use counter_windows::plugin::CounterWindowsPlugin;
use entity::plugin::EntityPlugin;
use gridmap::plugin::GridmapPlugin;
use helmet_security::plugin::HelmetsPlugin;
use human_male::plugin::HumanMalePlugin;
use humanoid::plugin::HumanoidPlugin;
use inventory::plugin::InventoryPlugin;
use jumpsuit_security::plugin::JumpsuitsPlugin;
use line_arrow::plugin::{LineArrowPlugin, PointArrowPlugin};
use map::plugin::MapPlugin;
use motd::motd::MOTD;
use networking::plugin::NetworkingPlugin;
use omni_light::plugin::OmniLightPlugin;
use pawn::plugin::PawnPlugin;
use physics::plugin::PhysicsPlugin;
use pistol_l1::plugin::PistolL1Plugin;
use player::plugin::PlayerPlugin;
use reflection_probe::plugin::ReflectionProbePlugin;
use resources::{labels::StartupLabels, plugin::ResourcesPlugin};
use setup_ui::plugin::SetupUiPlugin;
use sfx::plugin::SfxPlugin;
use sounds::plugin::SoundsPlugin;
use ui::plugin::UiPlugin;
use world_environment::plugin::WorldEnvironmentPlugin;

use crate::server_is_live;

/// The main plugin to add to execute the server.
pub struct ServerPlugin {
    pub custom_motd: Option<String>,
    pub threads_amount: Option<u8>,
    pub give_all_rcon: bool,
    pub version: String,
}
impl Default for ServerPlugin {
    fn default() -> Self {
        Self {
            custom_motd: None,
            version: "0.0.0".to_string(),
            // Dev values.
            threads_amount: None,
            give_all_rcon: true,
        }
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let task_pool;

        match self.threads_amount {
            Some(amn) => {
                task_pool = TaskPoolOptions::with_num_threads(amn.into());
            }
            None => {
                task_pool = TaskPoolOptions::default();
            }
        }
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;

        app.add_plugin(LogPlugin::default())
            .add_plugin(CorePlugin {
                task_pool_options: task_pool,
                ..Default::default()
            })
            .add_plugin(AssetPlugin::default())
            .insert_resource(wgpu_settings)
            .add_plugin(WindowPlugin {
                add_primary_window: false,
                exit_on_all_closed: false,
                ..Default::default()
            })
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(TimePlugin::default())
            .add_plugin(TransformPlugin::default())
            .add_plugin(HierarchyPlugin::default())
            .add_plugin(DiagnosticsPlugin::default())
            .add_plugin(ScenePlugin::default())
            .add_plugin(RenderPlugin::default())
            .add_plugin(ImagePlugin::default())
            .add_plugin(ControllerPlugin {
                custom_motd: self.custom_motd.clone(),
            })
            .add_plugin(AsanaPlugin)
            .add_plugin(WorldEnvironmentPlugin)
            .add_plugin(GridmapPlugin)
            .add_plugin(ResourcesPlugin)
            .add_plugin(PawnPlugin)
            .add_plugin(HumanMalePlugin)
            .add_plugin(SfxPlugin)
            .add_plugin(EntityPlugin)
            .add_plugin(AtmosphericsPlugin)
            .add_plugin(ConsoleCommandsPlugin)
            .add_plugin(ConstructionToolAdminPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(AirLocksPlugin)
            .add_plugin(CounterWindowsPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(NetworkingPlugin)
            .add_plugin(HumanoidPlugin)
            .add_plugin(ComputersPlugin)
            .add_plugin(CombatPlugin)
            .add_plugin(OmniLightPlugin)
            .add_plugin(ReflectionProbePlugin)
            .add_plugin(JumpsuitsPlugin)
            .add_plugin(HelmetsPlugin)
            .add_plugin(PistolL1Plugin)
            .add_plugin(LineArrowPlugin)
            .add_plugin(PointArrowPlugin)
            .add_plugin(SoundsPlugin)
            .add_plugin(ChatPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SetupUiPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(BasicConsoleCommandsPlugin {
                give_all_rcon: self.give_all_rcon,
            })
            .add_startup_system(
                server_is_live
                    .label(StartupLabels::ServerIsLive)
                    .after(StartupLabels::InitAtmospherics),
            );

        match &self.custom_motd {
            Some(motd) => {
                app.insert_resource(MOTD::new_motd(motd.clone()));
            }
            None => {
                app.insert_resource(MOTD::new_default(self.version.clone()));
            }
        }
    }
}
