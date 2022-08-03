use crate::server_is_live;
use api::data::StartupLabels;
use bevy::prelude::App;
use bevy::prelude::ParallelSystemDescriptorCoercion;
use space_plugin::plugin::SpacePlugin;

const SERVER_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn start_server() {
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
