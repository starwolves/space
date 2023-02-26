//! Launcher and loop initializer.

use actions::plugin::ActionsPlugin;
use airlocks::plugin::AirLocksPlugin;
use asana::plugin::AsanaPlugin;
use basic_console_commands::plugin::BasicConsoleCommandsPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::info;
use bevy::prelude::App;
use bevy::prelude::AssetPlugin;
use bevy::prelude::CorePlugin;
use bevy::prelude::HierarchyPlugin;
use bevy::prelude::ImagePlugin;
use bevy::prelude::IntoSystemDescriptor;
use bevy::prelude::PluginGroup;
use bevy::prelude::TaskPoolOptions;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;
use bevy::transform::TransformPlugin;
use bevy::window::PresentMode;
use bevy::window::WindowDescriptor;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin;
use bevy::window::WindowPosition;
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use chat::plugin::ChatPlugin;
use combat::plugin::CombatPlugin;
use computers::plugin::ComputersPlugin;
use console_commands::plugins::ConsoleCommandsPlugin;
use construction_tool::plugin::ConstructionToolAdminPlugin;
use controller::plugin::ControllerPlugin;
use counter_windows::plugin::CounterWindowsPlugin;
use entity::plugin::EntityPlugin;
use gridmap::plugin::GridmapPlugin;
use helmet_security::plugin::HelmetsPlugin;
use hud::plugin::HudPlugin;
use human_male::plugin::HumanMalePlugin;
use humanoid::plugin::HumanoidPlugin;
use inventory::plugin::InventoryPlugin;
use jumpsuit_security::plugin::JumpsuitsPlugin;
use line_arrow::plugin::LineArrowPlugin;
use line_arrow::plugin::PointArrowPlugin;
use main_menu::plugin::MainMenuPlugin;
use map::plugin::MapPlugin;
use motd::motd::MOTD;
use networking::plugin::NetworkingPlugin;
use pawn::plugin::PawnPlugin;
use physics::plugin::PhysicsPlugin;
use pistol_l1::plugin::PistolL1Plugin;
use player::plugin::PlayerPlugin;
use point_light::plugin::PointLightPlugin;
use quit::quit_application;
use resources::core::ClientInformation;
use resources::is_server::is_server;
use resources::labels::StartupLabels;
use resources::plugin::ResourcesPlugin;
use setup_menu::plugin::SetupMenuPlugin;
use sfx::plugin::SfxPlugin;
use sounds::plugin::SoundsPlugin;
use ui::plugin::UiPlugin;
use world::plugin::WorldPlugin;

mod quit;

/// The function that launches the server on application start.
fn main() {
    configure_and_start();
}

/// Prints "Live." from main module for fancy text output.
fn live() {
    info!("Live.");
}

/// Version of this crate as defined in this Cargo.toml.
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ASSET_FOLDER: &str = "../../assets";
pub(crate) fn configure_and_start() {
    let mut app = App::new();
    if is_server() {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;

        app.add_plugin(LogPlugin::default())
            .add_plugin(CorePlugin {
                task_pool_options: TaskPoolOptions::with_num_threads(4),
            })
            .add_plugin(AssetPlugin {
                asset_folder: ASSET_FOLDER.to_string(),
                ..Default::default()
            })
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
            .add_plugin(ImagePlugin::default());
    } else {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Space Frontiers ".to_string() + APP_VERSION,
                        width: 1280.,
                        height: 720.,
                        present_mode: PresentMode::AutoNoVsync,
                        position: WindowPosition::Centered,
                        mode: WindowMode::Windowed,
                        transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(AssetPlugin {
                    asset_folder: ASSET_FOLDER.to_string(),
                    ..Default::default()
                }),
        )
        .insert_resource(WinitSettings::game())
        .add_plugin(EguiPlugin)
        .insert_resource(ClientInformation {
            version: APP_VERSION.to_string(),
        })
        .add_system(quit_application);
    }
    app.add_plugin(AsanaPlugin)
        .add_plugin(GridmapPlugin)
        .add_plugin(ResourcesPlugin)
        .add_plugin(PawnPlugin)
        .add_plugin(HumanMalePlugin)
        .add_plugin(SfxPlugin)
        .add_plugin(EntityPlugin)
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
        .add_plugin(JumpsuitsPlugin)
        .add_plugin(HelmetsPlugin)
        .add_plugin(PistolL1Plugin)
        .add_plugin(LineArrowPlugin)
        .add_plugin(PointArrowPlugin)
        .add_plugin(SoundsPlugin)
        .add_plugin(ChatPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(SetupMenuPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PointLightPlugin)
        .add_plugin(BasicConsoleCommandsPlugin {
            give_all_rcon: true,
        })
        .add_startup_system(
            live.label(StartupLabels::ServerIsLive)
                .after(StartupLabels::InitAtmospherics),
        )
        .insert_resource(MOTD::new_default(APP_VERSION.to_string()))
        .add_plugin(MainMenuPlugin)
        .add_plugin(ControllerPlugin::default())
        .add_plugin(WorldPlugin)
        .add_plugin(HudPlugin)
        .run();
}
