use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use bevy_renet::{RenetClientPlugin, RenetServerPlugin};
use iyes_loopless::prelude::IntoConditionalSystem;

use super::server::{souls, startup_server_listen_connections};
use crate::{
    client::{
        connect_to_server, is_client_connected, receive_incoming_reliable_server_messages,
        receive_incoming_unreliable_server_messages, ConnectToServer, Connection,
        ConnectionPreferences, IncomingRawReliableServerMessage,
        IncomingRawUnreliableServerMessage,
    },
    messaging::{
        generate_typenames, init_reliable_message, init_unreliable_message, MessageSender,
        Typenames, TypenamesLabel,
    },
    server::{
        receive_incoming_reliable_client_messages, receive_incoming_unreliable_client_messages,
        GreetingClientServerMessage, IncomingRawReliableClientMessage,
        IncomingRawUnreliableClientMessage, NetworkingChatServerMessage, NetworkingClientMessage,
        UnreliableServerMessage,
    },
};
use bevy::app::CoreStage::PreUpdate;
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_plugin(RenetServerPlugin::default())
                .insert_resource(startup_server_listen_connections())
                .add_system(souls)
                .add_event::<IncomingRawReliableClientMessage>()
                .add_event::<IncomingRawUnreliableClientMessage>()
                .add_system_to_stage(
                    PreUpdate,
                    receive_incoming_reliable_client_messages.label(TypenamesLabel::SendRawEvents),
                )
                .add_system_to_stage(
                    PreUpdate,
                    receive_incoming_unreliable_client_messages
                        .label(TypenamesLabel::SendRawEvents),
                );
        } else {
            app.add_plugin(RenetClientPlugin::default())
                .add_system(connect_to_server)
                .add_event::<ConnectToServer>()
                .init_resource::<ConnectionPreferences>()
                .init_resource::<Connection>()
                .add_system_to_stage(
                    PreUpdate,
                    receive_incoming_reliable_server_messages
                        .run_if(is_client_connected)
                        .label(TypenamesLabel::SendRawEvents),
                )
                .add_system_to_stage(
                    PreUpdate,
                    receive_incoming_unreliable_server_messages
                        .run_if(is_client_connected)
                        .label(TypenamesLabel::SendRawEvents),
                )
                .add_event::<IncomingRawReliableServerMessage>()
                .add_event::<IncomingRawUnreliableServerMessage>();
        }

        app.init_resource::<Typenames>()
            .add_startup_system(generate_typenames.after(TypenamesLabel::Generate));
        init_reliable_message::<NetworkingClientMessage>(app, MessageSender::Client);
        init_unreliable_message::<UnreliableServerMessage>(app, MessageSender::Server);
        init_reliable_message::<NetworkingChatServerMessage>(app, MessageSender::Server);
        init_reliable_message::<GreetingClientServerMessage>(app, MessageSender::Both);
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;
