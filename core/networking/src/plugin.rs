use bevy::prelude::{resource_exists, App, IntoSystemConfigs, Plugin, PreUpdate, Startup, Update};
use bevy_renet::{
    renet::RenetClient,
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetClientPlugin, RenetServerPlugin,
};
use resources::is_server::is_server;

use super::server::{souls, startup_server_listen_connections};
use crate::{
    client::{
        confirm_connection, connect_to_server, connected, is_client_connected, on_disconnect,
        process_response, receive_incoming_reliable_server_messages,
        receive_incoming_unreliable_server_messages, token_assign_server, AssignTokenToServer,
        AssigningServerToken, ConnectToServer, Connection, ConnectionPreferences,
        IncomingRawReliableServerMessage, IncomingRawUnreliableServerMessage, TokenAssignServer,
    },
    messaging::{
        generate_typenames, register_reliable_message, register_unreliable_message, MessageSender,
        Typenames, TypenamesLabel,
    },
    server::{
        receive_incoming_reliable_client_messages, receive_incoming_unreliable_client_messages,
        IncomingRawReliableClientMessage, IncomingRawUnreliableClientMessage,
        NetworkingChatServerMessage, NetworkingClientMessage, NetworkingServerMessage,
        UnreliableServerMessage,
    },
};
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            let res = startup_server_listen_connections();
            app.add_plugins(RenetServerPlugin)
                .add_plugins(NetcodeServerPlugin)
                .insert_resource(res.0)
                .insert_resource(res.1)
                .add_systems(Update, souls)
                .add_event::<IncomingRawReliableClientMessage>()
                .add_event::<IncomingRawUnreliableClientMessage>()
                .add_systems(
                    PreUpdate,
                    receive_incoming_reliable_client_messages.in_set(TypenamesLabel::SendRawEvents),
                )
                .add_systems(
                    PreUpdate,
                    receive_incoming_unreliable_client_messages
                        .in_set(TypenamesLabel::SendRawEvents),
                );
        } else {
            app.add_systems(
                Update,
                (
                    process_response.run_if(resource_exists::<TokenAssignServer>()),
                    token_assign_server,
                    connect_to_server,
                ),
            )
            .add_event::<ConnectToServer>()
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .add_event::<AssignTokenToServer>()
            .init_resource::<ConnectionPreferences>()
            .init_resource::<Connection>()
            .init_resource::<AssigningServerToken>()
            .add_systems(
                PreUpdate,
                receive_incoming_reliable_server_messages
                    .in_set(TypenamesLabel::SendRawEvents)
                    .run_if(resource_exists::<RenetClient>()),
            )
            .add_systems(
                PreUpdate,
                receive_incoming_unreliable_server_messages
                    .in_set(TypenamesLabel::SendRawEvents)
                    .run_if(resource_exists::<RenetClient>()),
            )
            .add_event::<IncomingRawReliableServerMessage>()
            .add_event::<IncomingRawUnreliableServerMessage>()
            .add_systems(
                Update,
                (
                    confirm_connection.run_if(is_client_connected),
                    on_disconnect.run_if(connected),
                ),
            );
        }

        app.init_resource::<Typenames>()
            .add_systems(Startup, generate_typenames.after(TypenamesLabel::Generate));
        register_reliable_message::<NetworkingClientMessage>(app, MessageSender::Client);
        register_unreliable_message::<UnreliableServerMessage>(app, MessageSender::Server);
        register_reliable_message::<NetworkingChatServerMessage>(app, MessageSender::Server);
        register_reliable_message::<NetworkingServerMessage>(app, MessageSender::Server);
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;
