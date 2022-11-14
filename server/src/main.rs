//! Server launcher and loop initializer.

use bevy::prelude::info;
use bevy::prelude::App;
use plugin::ServerPlugin;

/// The function that launches the server on application start.
fn main() {
    configure_and_start_server();
}

/// Prints "Live." from main module for fancy text output.
fn server_is_live() {
    info!("Live.");
}

/// The main plugin where all other plugins come together.
pub mod plugin;

/// Version of this crate as defined in this Cargo.toml.
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Initiate and configure server. Include [SpacePlugin] in loop.
/// If you want to configure the server at start up do so here by modifying [SpacePlugin].
pub(crate) fn configure_and_start_server() {
    App::new()
        .add_plugin(ServerPlugin {
            version: APP_VERSION.to_owned(),
            ..Default::default()
        })
        .run();
}
