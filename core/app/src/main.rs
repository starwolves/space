//! Launcher and loop initializer.

use std::env::current_dir;
use std::sync::mpsc::SyncSender;

use actions::plugin::ActionsPlugin;
use airlocks::plugin::AirLocksPlugin;
use asana::plugin::AsanaPlugin;
use ball::plugin::BallPlugin;
use basic_console_commands::plugin::BasicConsoleCommandsPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::log::info;
use bevy::log::LogPlugin;
use bevy::prelude::App;
use bevy::prelude::AssetPlugin;
use bevy::prelude::FrameCountPlugin;
use bevy::prelude::HierarchyPlugin;
use bevy::prelude::ImagePlugin;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::PluginGroup;
use bevy::prelude::Startup;
use bevy::prelude::TaskPoolOptions;
use bevy::prelude::TaskPoolPlugin;
use bevy::prelude::TypeRegistrationPlugin;
use bevy::render::settings::RenderCreation;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::time::Fixed;
use bevy::time::Time;
use bevy::time::TimePlugin;
use bevy::transform::TransformPlugin;
use bevy::window::PresentMode;
use bevy::window::Window;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin;
use bevy::window::WindowPosition;
use bevy::winit::UpdateMode;
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::prelude::Physics;
use chat::plugin::ChatPlugin;
use combat::plugin::CombatPlugin;
use computers::plugin::ComputersPlugin;
use console_commands::plugins::ConsoleCommandsPlugin;
use construction_tool::plugin::ConstructionToolAdminPlugin;
use controller::plugin::ControllerPlugin;
use correction::CorrectionPlugin;
use correction::CorrectionServerMessage;
use correction::CorrectionServerPlugin;
use correction::CorrectionServerReceiveMessage;
use correction::CorrectionServerSendMessage;
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
use resources::modes::is_correction_mode;
use resources::modes::is_server;
use resources::modes::is_server_mode;
use resources::modes::AppMode;
use resources::plugin::ResourcesPlugin;
use resources::sets::StartupSet;
use setup_menu::plugin::SetupMenuPlugin;
use sfx::plugin::SfxPlugin;
use sounds::plugin::SoundsPlugin;
use token::plugin::TokenPlugin;
use ui::plugin::UiPlugin;

pub mod correction;

/// The function that launches the server on application start.
fn main() {
    start_app(Mode::Standard);
}

/// Prints "Live." from main module for fancy text output.
fn live() {
    info!("Live.");
}

/// Version of this crate as defined in this Cargo.toml.
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub(crate) enum Mode {
    Standard,
    Correction(
        CorrectionServerReceiveMessage,
        SyncSender<CorrectionServerMessage>,
    ),
}

/// Start client or server. Optionally start client in simulation correction mode and return new data.
pub(crate) fn start_app(mode: Mode) {
    let binding = current_dir().unwrap();
    let mut test_path = binding.as_path();
    let binding = test_path.join("assets");
    test_path = binding.as_path();
    let mut app = App::new();

    match mode {
        Mode::Standard => {
            app.insert_resource(AppMode::Standard);
        }
        Mode::Correction(receiver, sender) => {
            if !is_server() {
                app.insert_non_send_resource(receiver)
                    .insert_resource(CorrectionServerSendMessage { sender })
                    .insert_resource(AppMode::Correction);
            }
        }
    }

    let num_threads = 2;

    let task_pool = TaskPoolPlugin {
        task_pool_options: TaskPoolOptions::with_num_threads(num_threads),
    };

    if is_server_mode(&mut app) {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;

        if !is_correction_mode(&mut app) {
            app.add_plugins(LogPlugin::default());
        }

        app.add_plugins(TypeRegistrationPlugin)
            .add_plugins(FrameCountPlugin)
            .add_plugins(AssetPlugin {
                file_path: test_path.to_str().unwrap().to_owned(),
                ..Default::default()
            })
            .add_plugins(WindowPlugin::default())
            .add_plugins(TimePlugin::default())
            .add_plugins(TransformPlugin::default())
            .add_plugins(HierarchyPlugin::default())
            .add_plugins(DiagnosticsPlugin::default())
            .add_plugins(ScenePlugin::default())
            .add_plugins(RenderPlugin {
                render_creation: RenderCreation::Automatic(wgpu_settings),
            })
            .add_plugins(ImagePlugin::default())
            .add_plugins(ScheduleRunnerPlugin::default())
            .add_plugins(task_pool);
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
                    file_path: test_path.to_str().unwrap().to_owned(),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(task_pool),
        )
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
            ..Default::default()
        })
        .add_plugins(EguiPlugin)
        .add_plugins(GraphicsPlugin)
        .add_plugins(CorrectionPlugin)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(ClientInformation {
            version: APP_VERSION.to_string(),
        });
    }
    app.add_plugins(GridmapPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(PawnPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(ActionsPlugin)
        .add_plugins(NetworkingPlugin)
        .add_plugins(ControllerPlugin::default())
        .insert_resource(Time::<Fixed>::from_hz(
            TickRate::default().fixed_rate as f64,
        ))
        .insert_resource(Time::new_with(Physics::fixed_once_hz(
            TickRate::default().fixed_rate as f64,
        )))
        .init_resource::<TickRate>()
        .add_plugins(MetadataPlugin)
        .add_plugins(PlayerPlugin);

    if is_correction_mode(&mut app) {
        app.add_plugins(CorrectionServerPlugin);
    }
    app.add_plugins(ConstructionToolAdminPlugin)
        .add_plugins(AirLocksPlugin)
        .add_plugins(CounterWindowsPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(TokenPlugin)
        .add_plugins(AsanaPlugin)
        .add_plugins(HumanoidPlugin)
        .add_plugins(HumanMalePlugin)
        .add_plugins(SfxPlugin)
        .add_plugins(ComputersPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(ConsoleCommandsPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(JumpsuitsPlugin)
        .add_plugins(HelmetsPlugin)
        .add_plugins(PistolL1Plugin)
        .add_plugins(LineArrowPlugin)
        .add_plugins(PointArrowPlugin)
        .add_plugins(SoundsPlugin)
        .add_plugins(ChatPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(SetupMenuPlugin)
        .add_plugins(PointLightPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(BasicConsoleCommandsPlugin {
            give_all_rcon: true,
        })
        .add_systems(
            Startup,
            live.in_set(StartupSet::ServerIsLive)
                .after(StartupSet::InitAtmospherics),
        )
        .insert_resource(MOTD::new_default(APP_VERSION.to_string()))
        .add_plugins(MainMenuPlugin)
        .add_plugins(EscapeMenuPlugin)
        .add_plugins(HudPlugin);

    app.run();
}
