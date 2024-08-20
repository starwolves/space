//! Launcher and loop initializer.

use actions::plugin::ActionsPlugin;
use airlocks::plugin::AirLocksPlugin;
use asana::plugin::AsanaPlugin;
use ball::plugin::BallPlugin;
use basic_console_commands::plugin::BasicConsoleCommandsPlugin;
use bevy::app::FixedUpdate;
use bevy::app::Main;
use bevy::app::MainScheduleOrder;
use bevy::app::ScheduleRunnerPlugin;
use bevy::app::Update as BevyUpdate;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::ecs::reflect::AppTypeRegistry;
use bevy::ecs::schedule::IntoSystemSetConfigs;
use bevy::ecs::world::World;
use bevy::log::info;
use bevy::log::warn;
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
use cameras::controllers::fps::FpsCameraPlugin;
use cameras::LookTransformPlugin;
use chat::plugin::ChatPlugin;
use combat::plugin::CombatPlugin;
use computers::plugin::ComputersPlugin;
use console_commands::plugins::ConsoleCommandsPlugin;
use construction_tool::plugin::ConstructionToolAdminPlugin;
use controller::plugin::ControllerPlugin;
use correction::server_start_correcting;
use correction::Correction;
use correction::CorrectionApp;
use correction::CorrectionMessengers;
use correction::CorrectionPlugin;
use correction::CorrectionResultsSender;
use correction::CorrectionServerPlugin;
use correction::StartCorrectingMessage;
use counter_windows::plugin::CounterWindowsPlugin;
use entity::plugin::EntityPlugin;
use escape_menu::plugin::EscapeMenuPlugin;
use graphics::plugin::GraphicsPlugin;
use graphics::settings::get_settings;
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
use networking::client::PostUpdateSendMessage;
use networking::plugin::NetworkingPlugin;
use pawn::plugin::PawnPlugin;
use physics::correction_mode::CorrectionResults;
use physics::plugin::PhysicsPlugin;
use physics::plugin::PhysicsStepSet;
use pistol_l1::plugin::PistolL1Plugin;
use player::plugin::PlayerPlugin;
use point_light::plugin::PointLightPlugin;
use resources::core::ClientInformation;
use resources::core::TickRate;
use resources::correction::ObtainedSynchronousSyncData;
use resources::correction::SynchronousCorrection;
use resources::correction::SynchronousCorrectionOnGoing;
use resources::modes::is_correction_mode;
use resources::modes::is_server_mode;
use resources::modes::AppMode;
use resources::ordering::Fin;
use resources::ordering::First;
use resources::ordering::PostUpdate;
use resources::ordering::PreUpdate;
use resources::ordering::StartupSet;
use resources::ordering::Update;
use resources::plugin::ResourcesPlugin;
use setup_menu::plugin::SetupMenuPlugin;
use sfx::plugin::SfxPlugin;
use sounds::plugin::SoundsPlugin;
use std::env::current_dir;
use token::plugin::TokenPlugin;
use ui::plugin::UiPlugin;

pub mod correction;

/// The function that launches the server on application start.
fn main() {
    init_app(None);
}

/// Prints "Live." from main module for fancy text output.
fn live() {
    info!("Live.");
}

/// Version of this crate as defined in this Cargo.toml.
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Start client with correction app, or server.
pub(crate) fn init_app(correction: Option<CorrectionMessengers>) {
    let mut app = App::new();

    let perf = get_settings();
    app.insert_resource(SynchronousCorrection(perf.synchronous_correction))
        .insert_resource(perf.clone())
        .init_resource::<SynchronousCorrectionOnGoing>()
        .init_resource::<ObtainedSynchronousSyncData>();

    match correction {
        Some(messengers) => {
            app.insert_resource(AppMode::Correction);
            app.insert_non_send_resource(messengers.rx);
            app.insert_resource(CorrectionResultsSender {
                tx: Some(messengers.tx),
            });
        }
        None => {
            app.insert_resource(AppMode::Standard);
            app.add_systems(FixedUpdate, step_game_schedules);
        }
    }

    init_shedules(&mut app);
    setup_plugins(&mut app);

    if !is_server_mode(&mut app) && !perf.synchronous_correction {
        let mut correction_sub_app = App::empty();
        correction_sub_app.insert_resource(AppMode::Correction);
        correction_sub_app.insert_resource(SynchronousCorrection(perf.synchronous_correction));
        correction_sub_app.insert_resource(CorrectionResultsSender { tx: None });

        setup_plugins(&mut correction_sub_app);

        let sub_app = correction_sub_app.main_mut();
        sub_app.set_extract(|main_world, sub_app| {
            *sub_app.resource_mut::<StartCorrectingMessage>() =
                main_world.resource::<StartCorrectingMessage>().clone();
            *main_world.resource_mut::<CorrectionResults>() =
                sub_app.resource::<CorrectionResults>().clone();
        });
        // Run sub app once to initiate its world with Startup Schedule.
        sub_app.update();

        app.insert_non_send_resource(CorrectionApp {
            app: correction_sub_app,
        });
    }

    app.run();
}
fn setup_plugins(mut app: &mut App) {
    let binding = current_dir().unwrap();
    let mut test_path = binding.as_path();
    let binding = test_path.join("assets");
    test_path = binding.as_path();

    let num_threads = 2;
    let syncronous_correction = app.world().resource::<SynchronousCorrection>().0;

    let task_pool = TaskPoolPlugin {
        task_pool_options: TaskPoolOptions::with_num_threads(num_threads),
    };
    if is_correction_mode(app) && !syncronous_correction {
        app.init_resource::<AppTypeRegistry>()
            .init_resource::<MainScheduleOrder>()
            .add_systems(Main, server_start_correcting);
    } else if is_correction_mode(app) {
        app.add_systems(BevyUpdate, server_start_correcting);
    }
    if is_server_mode(app) {
        let mut wgpu_settings = WgpuSettings::default();
        wgpu_settings.backends = None;
        app.add_plugins(AssetPlugin {
            file_path: test_path.to_str().unwrap().to_owned(),
            ..Default::default()
        });
        if !is_correction_mode(app) || syncronous_correction {
            if !(is_correction_mode(app) && syncronous_correction) {
                app.add_plugins(LogPlugin::default());
            }
            app.add_plugins(TypeRegistrationPlugin)
                .add_plugins(ScheduleRunnerPlugin::default())
                .add_plugins(FrameCountPlugin)
                .add_plugins(DiagnosticsPlugin::default())
                .add_plugins(ImagePlugin::default())
                .add_plugins(ScenePlugin::default());
        }
        if is_correction_mode(app) {
            app.add_plugins(CorrectionPlugin);
            if !syncronous_correction {
                app.add_plugins(ScheduleRunnerPlugin::run_once());
            }
        }
        app.add_plugins(WindowPlugin::default())
            .add_plugins(TimePlugin::default())
            .add_plugins(TransformPlugin::default())
            .add_plugins(HierarchyPlugin::default())
            .add_plugins(RenderPlugin {
                render_creation: RenderCreation::Automatic(wgpu_settings),
                ..Default::default()
            })
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
        .add_plugins(CorrectionPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(GraphicsPlugin)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(ClientInformation {
            version: APP_VERSION.to_string(),
        })
        .add_plugins(FpsCameraPlugin::default());
    }

    app.configure_sets(
        PostUpdate,
        (PostUpdateSendMessage, PhysicsStepSet, Correction).chain(),
    )
    .add_plugins(ResourcesPlugin)
    .add_plugins(PhysicsPlugin)
    .add_plugins(EntityPlugin)
    .add_plugins(NetworkingPlugin)
    .add_plugins(GridmapPlugin)
    .add_plugins(ControllerPlugin)
    .add_plugins(HumanoidPlugin)
    .insert_resource(Time::<Fixed>::from_hz(
        TickRate::default().fixed_rate as f64,
    ))
    .insert_resource(Time::new_with(Physics::fixed_once_hz(
        TickRate::default().fixed_rate as f64,
    )))
    .init_resource::<TickRate>()
    .add_plugins(MetadataPlugin)
    .add_plugins(LookTransformPlugin)
    .add_plugins(HumanMalePlugin);

    if is_correction_mode(&mut app) {
        app.add_plugins(CorrectionServerPlugin);
    }
    if !is_correction_mode(&mut app) {
        app.add_plugins(ActionsPlugin)
            .add_plugins(PawnPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ConstructionToolAdminPlugin)
            .add_plugins(AirLocksPlugin)
            .add_plugins(CounterWindowsPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(TokenPlugin)
            .add_plugins(AsanaPlugin)
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
    }
}
fn init_shedules(app: &mut App) {
    app.init_schedule(First);
    app.init_schedule(PreUpdate);
    app.init_schedule(Update);
    app.init_schedule(PostUpdate);
    app.init_schedule(Fin);
}
fn step_game_schedules(world: &mut World) {
    match world.try_run_schedule(First) {
        Ok(_) => {}
        Err(rr) => {
            warn!("First: {}", rr);
        }
    }
    match world.try_run_schedule(PreUpdate) {
        Ok(_) => {}
        Err(rr) => {
            warn!("PreFixedUpdate: {}", rr);
        }
    }
    match world.try_run_schedule(Update) {
        Ok(_) => {}
        Err(rr) => {
            warn!("MainFixedUpdate: {}", rr);
        }
    }
    match world.try_run_schedule(PostUpdate) {
        Ok(_) => {}
        Err(rr) => {
            warn!("PostFixedUpdate: {}", rr);
        }
    }
    match world.try_run_schedule(Fin) {
        Ok(_) => {}
        Err(rr) => {
            warn!("Fin: {}", rr);
        }
    }
}
