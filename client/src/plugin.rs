use std::time::Duration;

use bevy::{
    app::{RunMode, ScheduleRunnerSettings},
    prelude::{App, CorePlugin, Plugin, PluginGroup, TaskPoolOptions},
    window::{PresentMode, WindowDescriptor, WindowMode, WindowPlugin, WindowPosition},
    winit::WinitSettings,
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use main_menu::plugin::MainMenuPlugin;
use networking::plugin::NetworkingPlugin;
use resources::{core::ClientInformation, plugin::ResourcesPlugin};
use ui::plugin::UiPlugin;
use winit_windows::plugin::WinitWindowsPlugin;

use crate::client_is_live;

/// The main plugin to add to execute the client.
pub struct ClientPlugin {
    pub version: String,
    pub threads_amount: Option<u8>,
}

impl Default for ClientPlugin {
    fn default() -> Self {
        Self {
            version: "0.0.0".to_string(),
            threads_amount: Some(2),
        }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let task_pool;
        match self.threads_amount {
            Some(amn) => {
                task_pool = TaskPoolOptions::with_num_threads(amn.into());
            }
            None => task_pool = TaskPoolOptions::default(),
        }

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Space Frontiers ".to_string() + &self.version,
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
                .set(CorePlugin {
                    task_pool_options: task_pool,
                }),
        )
        .insert_resource(WinitSettings::game())
        .add_plugin(MainMenuPlugin)
        .add_plugin(WinitWindowsPlugin)
        .add_plugin(ResourcesPlugin)
        .add_plugin(EguiPlugin)
        .insert_resource(ClientInformation {
            version: self.version.clone(),
        })
        .add_plugin(UiPlugin)
        .add_plugin(NetworkingPlugin)
        .add_startup_system(client_is_live)
        .insert_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_secs_f64(1. / (64. as f64))),
            },
        });
    }
}
