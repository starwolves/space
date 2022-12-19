use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use bevy_renet::{RenetClientPlugin, RenetServerPlugin};
use iyes_loopless::prelude::IntoConditionalSystem;
use resources::is_server::is_server;

use super::server::{souls, startup_server_listen_connections};
use crate::{
    client::{
        confirm_connection, connect_to_server, connected, is_client_connected, on_disconnect,
        receive_incoming_reliable_server_messages, receive_incoming_unreliable_server_messages,
        ConnectToServer, Connection, ConnectionPreferences, IncomingRawReliableServerMessage,
        IncomingRawUnreliableServerMessage,
    },
    messaging::{
        generate_typenames, init_reliable_message, init_unreliable_message, MessageSender,
        Typenames, TypenamesLabel,
    },
    server::{
        receive_incoming_reliable_client_messages, receive_incoming_unreliable_client_messages,
        IncomingRawReliableClientMessage, IncomingRawUnreliableClientMessage,
        NetworkingChatServerMessage, NetworkingClientMessage, NetworkingServerMessage,
        UnreliableServerMessage,
    },
};
use bevy::app::CoreStage::PreUpdate;
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
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
                .add_event::<IncomingRawUnreliableServerMessage>()
                .add_system(confirm_connection.run_if(is_client_connected))
                .add_system(on_disconnect.run_if(connected));
        }

        app.init_resource::<Typenames>()
            .add_startup_system(generate_typenames.after(TypenamesLabel::Generate));
        init_reliable_message::<NetworkingClientMessage>(app, MessageSender::Client);
        init_unreliable_message::<UnreliableServerMessage>(app, MessageSender::Server);
        init_reliable_message::<NetworkingChatServerMessage>(app, MessageSender::Server);
        init_reliable_message::<NetworkingServerMessage>(app, MessageSender::Server);
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;
