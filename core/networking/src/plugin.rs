use std::env;

use bevy::prelude::{App, Plugin};
use bevy::prelude::{IntoSystemDescriptor, SystemSet};
use bevy_renet::{RenetClientPlugin, RenetServerPlugin};
use resources::labels::PostUpdateLabels;

use super::server::{souls, startup_server_listen_connections};
use crate::client::{connect_to_server, ConnectToServer, Connection, ConnectionPreferences};
use crate::server::process_finalize_net;
use crate::server::PendingNetworkMessage;
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_plugin(RenetServerPlugin::default())
                .insert_resource(startup_server_listen_connections())
                .add_system_to_stage(PreUpdate, souls)
                .add_event::<PendingNetworkMessage>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net),
                )
                .add_system_to_stage(
                    PostUpdate,
                    process_finalize_net.after(PostUpdateLabels::Net),
                );
        } else {
            app.add_plugin(RenetClientPlugin::default())
                .add_system(connect_to_server)
                .add_event::<ConnectToServer>()
                .init_resource::<ConnectionPreferences>()
                .init_resource::<Connection>();
        }
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;
