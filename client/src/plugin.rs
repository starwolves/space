use std::time::Duration;

use bevy::{
    app::{RunMode, ScheduleRunnerSettings},
    prelude::{App, DefaultTaskPoolOptions, Plugin},
    window::{MonitorSelection, PresentMode, WindowDescriptor, WindowMode, WindowPosition},
    winit::WinitSettings,
    DefaultPlugins,
};
use menu_main::plugin::MainMenuPlugin;
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
        /*.insert_resource(startup_client_listen_connections(
            ServerAddress {
                address: socket_address,
            },
            encryption_key_string,
        ))*/
        app.insert_resource(WindowDescriptor {
            title: "Space Frontiers ".to_string() + &self.version,
            width: 1280.,
            height: 720.,
            present_mode: PresentMode::AutoNoVsync,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            mode: WindowMode::Windowed,
            transparent: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::game())
        .add_plugin(MainMenuPlugin)
        .add_plugin(WinitWindowsPlugin)
        .add_startup_system(client_is_live)
        .insert_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_secs_f64(1. / (64. as f64))),
            },
        });
        match self.threads_amount {
            Some(amn) => {
                app.insert_resource(DefaultTaskPoolOptions::with_num_threads(amn.into()));
            }
            None => {}
        }
    }
}
