//! Launcher and loop initializer.

use std::env::current_dir;
use std::time::Duration;

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
use bevy::prelude::FixedTime;
use bevy::prelude::FrameCountPlugin;
use bevy::prelude::HierarchyPlugin;
use bevy::prelude::ImagePlugin;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::PluginGroup;
use bevy::prelude::Startup;
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
use metadata::MetadataPlugin;
use motd::motd::MOTD;
use networking::plugin::NetworkingPlugin;
use pawn::plugin::PawnPlugin;
use physics::plugin::PhysicsPlugin;
use pistol_l1::plugin::PistolL1Plugin;
use player::plugin::PlayerPlugin;
use point_light::plugin::PointLightPlugin;
use resources::core::ClientInformation;
use resources::core::TickRate;
use resources::is_server::is_server;
use resources::labels::StartupLabels;
use resources::plugin::ResourcesPlugin;
use setup_menu::plugin::SetupMenuPlugin;
use sfx::plugin::SfxPlugin;
use sounds::plugin::SoundsPlugin;
use token::plugin::TokenPlugin;
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
pub(crate) fn configure_and_start() {
    let binding = current_dir().unwrap();
    let mut test_path = binding.as_path();
    let binding = test_path.join("assets");
    test_path = binding.as_path();
    let mut app = App::new();

    if is_server() {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;

        app.add_plugins(LogPlugin::default())
            .add_plugins(TaskPoolPlugin {
                task_pool_options: TaskPoolOptions::with_num_threads(1),
            })
            .add_plugins(TypeRegistrationPlugin)
            .add_plugins(FrameCountPlugin)
            .add_plugins(AssetPlugin {
                asset_folder: test_path.to_str().unwrap().to_owned(),
                ..Default::default()
            })
            .add_plugins(WindowPlugin::default())
            .add_plugins(TimePlugin::default())
            .add_plugins(TransformPlugin::default())
            .add_plugins(HierarchyPlugin::default())
            .add_plugins(DiagnosticsPlugin::default())
            .add_plugins(ScenePlugin::default())
            .add_plugins(RenderPlugin {
                wgpu_settings: wgpu_settings,
            })
            .add_plugins(ImagePlugin::default());
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
                    asset_folder: test_path.to_str().unwrap().to_owned(),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(WinitSettings::game())
        .add_plugins(EguiPlugin)
        .insert_resource(ClientInformation {
            version: APP_VERSION.to_string(),
        })
        .add_plugins(GraphicsPlugin);
    }
    app.add_plugins(TokenPlugin)
        .add_plugins(AsanaPlugin)
        .add_plugins(GridmapPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(PawnPlugin)
        .add_plugins(HumanMalePlugin)
        .add_plugins(SfxPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(ConsoleCommandsPlugin)
        .add_plugins(ConstructionToolAdminPlugin)
        .add_plugins(ActionsPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(AirLocksPlugin)
        .add_plugins(CounterWindowsPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(NetworkingPlugin)
        .add_plugins(HumanoidPlugin)
        .add_plugins(ComputersPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(JumpsuitsPlugin)
        .add_plugins(HelmetsPlugin)
        .add_plugins(PistolL1Plugin)
        .add_plugins(LineArrowPlugin)
        .add_plugins(PointArrowPlugin)
        .add_plugins(SoundsPlugin)
        .add_plugins(ChatPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SetupMenuPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PointLightPlugin)
        .add_plugins(BasicConsoleCommandsPlugin {
            give_all_rcon: true,
        })
        .add_systems(
            Startup,
            live.in_set(StartupLabels::ServerIsLive)
                .after(StartupLabels::InitAtmospherics),
        )
        .insert_resource(MOTD::new_default(APP_VERSION.to_string()))
        .add_plugins(MainMenuPlugin)
        .add_plugins(EscapeMenuPlugin)
        .add_plugins(ControllerPlugin::default())
        .add_plugins(WorldPlugin)
        .add_plugins(HudPlugin)
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_plugins(MetadataPlugin)
        .init_resource::<TickRate>()
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
            1. / TickRate::default().bevy_rate as f32,
        )))
        .run();
}
