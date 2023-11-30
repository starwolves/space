use bevy::{
    ecs::schedule::IntoSystemSetConfigs,
    prelude::{resource_exists, App, FixedUpdate, IntoSystemConfigs, Plugin, Startup},
};
use bevy_renet::{
    renet::RenetClient,
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetClientPlugin, RenetServerPlugin,
};
use resources::{
    modes::{is_correction_mode, is_server_mode},
    sets::MainSet,
};

use super::server::{souls, startup_server_listen_connections};
use crate::{
    client::{
        confirm_connection, connect_to_server, connected, is_client_connected, on_disconnect,
        receive_incoming_reliable_server_messages, receive_incoming_unreliable_server_messages,
        starwolves_response, step_buffer, sync_client, token_assign_server, AssignTokenToServer,
        AssigningServerToken, ConnectToServer, Connection, ConnectionPreferences,
        IncomingRawReliableServerMessage, IncomingRawUnreliableServerMessage,
        NetworkingClientMessage, OutgoingBuffer, SyncClient, TokenAssignServer,
    },
    messaging::{
        generate_typenames, register_reliable_message, register_unreliable_message, MessageSender,
        Typenames, TypenamesSet,
    },
    server::{
        adjust_clients, clear_construct_entity_updates, clear_serialized_entity_updates,
        receive_incoming_reliable_client_messages, receive_incoming_unreliable_client_messages,
        ConstructEntityUpdates, EntityUpdatesSerialized, EntityUpdatesSet, HandleToEntity,
        IncomingRawReliableClientMessage, IncomingRawUnreliableClientMessage, Latency,
        NetworkingChatServerMessage, NetworkingServerMessage, SyncConfirmations,
        UnreliableServerMessage,
    },
    stamp::{setup_client_tickrate_stamp, step_tickrate_stamp, PauseTickStep, TickRateStamp},
};
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            if !is_correction_mode(app) {
                let res = startup_server_listen_connections();
                app.insert_resource(res.0)
                    .insert_resource(res.1)
                    .add_systems(
                        FixedUpdate,
                        (
                            receive_incoming_reliable_client_messages
                                .in_set(TypenamesSet::SendRawEvents),
                            receive_incoming_unreliable_client_messages
                                .in_set(TypenamesSet::SendRawEvents)
                                .after(receive_incoming_reliable_client_messages),
                        )
                            .in_set(MainSet::PreUpdate),
                    )
                    .configure_sets(
                        FixedUpdate,
                        (
                            EntityUpdatesSet::Write,
                            EntityUpdatesSet::Prepare,
                            EntityUpdatesSet::BuildUpdates,
                            EntityUpdatesSet::Serialize,
                            EntityUpdatesSet::Ready,
                        )
                            .chain(),
                    );
            }

            app.add_plugins(RenetServerPlugin)
                .add_plugins(NetcodeServerPlugin)
                .init_resource::<Latency>()
                .init_resource::<SyncConfirmations>()
                .add_systems(FixedUpdate, souls.in_set(MainSet::Update))
                .add_event::<IncomingRawReliableClientMessage>()
                .add_event::<IncomingRawUnreliableClientMessage>()
                .add_systems(
                    FixedUpdate,
                    (
                        adjust_clients
                            .after(TypenamesSet::SendRawEvents)
                            .in_set(MainSet::PreUpdate),
                        clear_construct_entity_updates.in_set(MainSet::PreUpdate),
                        clear_serialized_entity_updates.in_set(MainSet::PreUpdate),
                    ),
                )
                .init_resource::<ConstructEntityUpdates>()
                .init_resource::<EntityUpdatesSerialized>();
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    starwolves_response.run_if(resource_exists::<TokenAssignServer>()),
                    token_assign_server,
                    connect_to_server.after(starwolves_response),
                    setup_client_tickrate_stamp,
                )
                    .in_set(MainSet::Update),
            )
            .add_event::<ConnectToServer>()
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .add_event::<AssignTokenToServer>()
            .init_resource::<ConnectionPreferences>()
            .init_resource::<Connection>()
            .init_resource::<AssigningServerToken>()
            .add_systems(
                FixedUpdate,
                receive_incoming_reliable_server_messages
                    .in_set(TypenamesSet::SendRawEvents)
                    .run_if(resource_exists::<RenetClient>())
                    .in_set(MainSet::PreUpdate),
            )
            .add_systems(
                FixedUpdate,
                receive_incoming_unreliable_server_messages
                    .in_set(TypenamesSet::SendRawEvents)
                    .run_if(resource_exists::<RenetClient>())
                    .in_set(MainSet::PreUpdate),
            )
            .add_event::<IncomingRawReliableServerMessage>()
            .add_event::<IncomingRawUnreliableServerMessage>()
            .add_systems(
                FixedUpdate,
                (
                    confirm_connection.run_if(is_client_connected),
                    sync_client
                        .run_if(is_client_connected)
                        .after(confirm_connection),
                    on_disconnect.run_if(connected),
                )
                    .in_set(MainSet::Update),
            )
            .init_resource::<OutgoingBuffer>()
            .add_systems(
                FixedUpdate,
                step_buffer
                    .run_if(resource_exists::<RenetClient>())
                    .in_set(MainSet::PostUpdate),
            )
            .init_resource::<SyncClient>();
        }

        app.init_resource::<TickRateStamp>()
            .init_resource::<HandleToEntity>()
            .init_resource::<PauseTickStep>()
            .add_systems(FixedUpdate, step_tickrate_stamp.in_set(MainSet::PreUpdate))
            .init_resource::<Typenames>()
            .add_systems(Startup, generate_typenames.after(TypenamesSet::Generate));
        register_reliable_message::<NetworkingClientMessage>(app, MessageSender::Client, true);
        register_unreliable_message::<UnreliableServerMessage>(app, MessageSender::Server);
        register_reliable_message::<NetworkingChatServerMessage>(app, MessageSender::Server, true);
        register_reliable_message::<NetworkingServerMessage>(app, MessageSender::Server, true);
    }
}

pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_RELIABLE_UNORDERED_ID: u8 = 1;
pub const RENET_RELIABLE_ORDERED_ID: u8 = 2;
