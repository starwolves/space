use std::env;

use bevy::prelude::{App, Plugin};
use bevy_renet::{RenetClientPlugin, RenetServerPlugin};
use iyes_loopless::prelude::IntoConditionalSystem;

use super::server::{souls, startup_server_listen_connections};
use crate::client::{
    connect_to_server, connecting, messages_to_event, ConnectToServer, Connection,
    ConnectionPreferences, InboundReliableServerMessages, InboundUnreliableServerMessages,
};
use bevy::app::CoreStage::PreUpdate;
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_plugin(RenetServerPlugin::default())
                .insert_resource(startup_server_listen_connections())
                .add_system(souls);
        } else {
            app.add_plugin(RenetClientPlugin::default())
                .add_system(connect_to_server)
                .add_event::<ConnectToServer>()
                .init_resource::<ConnectionPreferences>()
                .add_system_to_stage(PreUpdate, messages_to_event.run_if(connecting))
                .add_event::<InboundReliableServerMessages>()
                .add_event::<InboundUnreliableServerMessages>()
                .init_resource::<Connection>();
        }
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;
