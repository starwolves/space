use crate::server_is_live;
use bevy::prelude::App;
use bevy::prelude::ParallelSystemDescriptorCoercion;
use server::labels::StartupLabels;
use space_plugin::plugin::SpacePlugin;

/// Version of this crate as defined in this Cargo.toml.
const SERVER_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Initiate and configure server. Include [SpacePlugin] in loop.
/// If you want to configure the server at start up do so here by modifying [SpacePlugin].
pub(super) fn configure_and_start_server() {
    App::new()
        .add_startup_system(
            server_is_live
                .label(StartupLabels::ServerIsLive)
                .after(StartupLabels::InitAtmospherics),
        )
        .add_plugin(SpacePlugin {
            server_version: SERVER_VERSION.to_owned(),
            ..Default::default()
        })
        .run();
}
