//! Launcher and loop initializer.

use actions::plugin::ActionsPlugin;
use airlocks::plugin::AirLocksPlugin;
use asana::plugin::AsanaPlugin;
use basic_console_commands::plugin::BasicConsoleCommandsPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::info;
use bevy::prelude::App;
use bevy::prelude::AssetPlugin;
use bevy::prelude::FixedTime;
use bevy::prelude::FrameCountPlugin;
use bevy::prelude::HierarchyPlugin;
use bevy::prelude::ImagePlugin;
use bevy::prelude::IntoSystemConfig;
use bevy::prelude::PluginGroup;
use bevy::prelude::TaskPoolOptions;
use bevy::prelude::TaskPoolPlugin;
use bevy::prelude::TypeRegistrationPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::time::TimePlugin;
use bevy::transform::TransformPlugin;
use bevy::window::PresentMode;
use bevy::window::Window;
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
use escape_menu::plugin::EscapeMenuPlugin;
use graphics::plugin::GraphicsPlugin;
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
use resources::core::ClientInformation;
use resources::is_server::is_server;
use resources::labels::StartupLabels;
use resources::plugin::ResourcesPlugin;
use setup_menu::plugin::SetupMenuPlugin;
use sfx::plugin::SfxPlugin;
use sounds::plugin::SoundsPlugin;
use ui::plugin::UiPlugin;
use world::plugin::WorldPlugin;

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
            .add_plugin(TaskPoolPlugin {
                task_pool_options: TaskPoolOptions::with_num_threads(4),
            })
            .add_plugin(TypeRegistrationPlugin)
            .add_plugin(FrameCountPlugin)
            .add_plugin(AssetPlugin {
                asset_folder: ASSET_FOLDER.to_string(),
                ..Default::default()
            })
            .add_plugin(WindowPlugin::default())
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(TimePlugin::default())
            .add_plugin(TransformPlugin::default())
            .add_plugin(HierarchyPlugin::default())
            .add_plugin(DiagnosticsPlugin::default())
            .add_plugin(ScenePlugin::default())
            .add_plugin(RenderPlugin {
                wgpu_settings: wgpu_settings,
            })
            .add_plugin(ImagePlugin::default());
    } else {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Frontiers ".to_string() + APP_VERSION,
                        present_mode: PresentMode::AutoNoVsync,
                        position: WindowPosition::Automatic,
                        mode: WindowMode::Windowed,
                        transparent: true,
                        ..Default::default()
                    }),
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
        .add_plugin(GraphicsPlugin);
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
            live.in_set(StartupLabels::ServerIsLive)
                .after(StartupLabels::InitAtmospherics),
        )
        .insert_resource(MOTD::new_default(APP_VERSION.to_string()))
        .add_plugin(MainMenuPlugin)
        .add_plugin(EscapeMenuPlugin)
        .add_plugin(ControllerPlugin::default())
        .add_plugin(WorldPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .run();
}
