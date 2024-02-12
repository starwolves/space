use bevy::{
    app::PostUpdate as BevyPostUpdate,
    app::PreUpdate as BevyPreUpdate,
    ecs::schedule::{IntoSystemSetConfigs, ScheduleLabel},
    prelude::{resource_exists, App, IntoSystemConfigs, Plugin, Startup},
};
use bevy_renet::{
    renet::RenetClient,
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    CoreSet, RenetClientPlugin, RenetReceive, RenetServerPlugin,
};
use resources::{
    modes::{is_correction_mode, is_server_mode},
    ordering::{BuildingSet, First, PostUpdate, PreUpdate, Update},
};

use super::server::{souls, startup_server_listen_connections};
use crate::{
    client::{
        clear_raw_spawn_entity_queue, confirm_connection, connect_to_server, connected,
        detect_client_world_loaded, is_client_connected, on_disconnect, post_update_send_messages,
        pre_update_send_messages, receive_incoming_reliable_server_messages,
        receive_incoming_unreliable_server_messages, starwolves_response, step_buffer,
        sync_check_client, token_assign_server, update_tick_latency, AssignTokenToServer,
        AssigningServerToken, BevyPreUpdateSendMessage, ClientGameWorldLoaded, ConnectToServer,
        Connection, ConnectionPreferences, IncomingRawReliableServerMessage,
        IncomingRawUnreliableServerMessage, LoadedGameWorldBuffer, NetworkingClientMessage,
        NetworkingUnreliableClientMessage, OutgoingBuffer, PostUpdateSendMessage,
        QueuedSpawnEntityRaw, TickLatency, TokenAssignServer, TotalAdjustment,
    },
    messaging::{
        generate_typenames, register_reliable_message, register_unreliable_message, MessageSender,
        MessagingSet, Typenames, TypenamesSet,
    },
    server::{
        adjust_clients, clean_latency_reports, clear_construct_entity_updates,
        clear_serialized_entity_updates, client_loaded_game_world,
        latency_report_incoming_messages, process_sync_confirmation,
        receive_incoming_reliable_client_messages, receive_incoming_unreliable_client_messages,
        server_events, start_sync_confirmation, ClientsReadyForSync, ConstructEntityUpdates,
        EntityUpdatesSerialized, EntityUpdatesSet, HandleToEntity,
        IncomingRawReliableClientMessage, IncomingRawUnreliableClientMessage,
        IncomingReliableClientMessageToReport, IncomingUnreliableClientMessageToReport, Latency,
        LatencyLimits, NetworkingChatServerMessage, NetworkingServerMessage, SyncConfirmations,
        UnreliableServerMessage,
    },
    stamp::{step_tickrate_stamp, PauseTickStep, TickRateStamp},
};
use bevy_renet::NetSchedules;
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        let schedules = NetSchedules {
            pre: BevyPreUpdate.intern(),
            post: BevyPostUpdate.intern(),
        };
        if is_server_mode(app) {
            if !is_correction_mode(app) {
                let res = startup_server_listen_connections();
                app.insert_resource(res.0)
                    .insert_resource(res.1)
                    .init_resource::<LatencyLimits>()
                    .add_systems(
                        PreUpdate,
                        (
                            receive_incoming_reliable_client_messages,
                            receive_incoming_unreliable_client_messages
                                .after(receive_incoming_reliable_client_messages),
                        )
                            .in_set(TypenamesSet::SendRawEvents),
                    )
                    .add_systems(
                        PreUpdate,
                        (
                            latency_report_incoming_messages
                                .in_set(TypenamesSet::SendRawEvents)
                                .after(receive_incoming_unreliable_client_messages),
                            adjust_clients.after(TypenamesSet::SendRawEvents),
                            server_events.after(RenetReceive).after(CoreSet::Pre),
                            /*_adjust_latency_limits
                            .after(TypenamesSet::SendRawEvents)
                            .before(adjust_clients),*/
                        ),
                    )
                    .add_systems(PostUpdate, clean_latency_reports)
                    .add_systems(
                        Update,
                        (
                            client_loaded_game_world,
                            start_sync_confirmation,
                            process_sync_confirmation,
                        ),
                    )
                    .configure_sets(
                        Update,
                        (
                            EntityUpdatesSet::Write,
                            EntityUpdatesSet::Prepare,
                            EntityUpdatesSet::BuildUpdates,
                            EntityUpdatesSet::Serialize,
                            EntityUpdatesSet::Ready,
                        )
                            .chain(),
                    )
                    .init_resource::<ClientsReadyForSync>();
            }

            app.add_plugins(RenetServerPlugin {
                schedules: schedules,
            })
            .add_plugins(NetcodeServerPlugin {
                schedules: schedules,
            })
            .init_resource::<Latency>()
            .init_resource::<SyncConfirmations>()
            .add_systems(Update, souls)
            .add_event::<IncomingRawReliableClientMessage>()
            .add_event::<IncomingRawUnreliableClientMessage>()
            .add_event::<IncomingReliableClientMessageToReport>()
            .add_event::<IncomingUnreliableClientMessageToReport>()
            .add_systems(
                PreUpdate,
                (
                    clear_construct_entity_updates,
                    clear_serialized_entity_updates,
                ),
            )
            .init_resource::<ConstructEntityUpdates>()
            .init_resource::<EntityUpdatesSerialized>();
        } else {
            app.init_resource::<TickLatency>()
                .add_systems(
                    BevyPreUpdate,
                    pre_update_send_messages
                        .in_set(BevyPreUpdateSendMessage)
                        .run_if(resource_exists::<RenetClient>()),
                )
                .init_resource::<QueuedSpawnEntityRaw>()
                .add_systems(
                    Update,
                    (
                        update_tick_latency.run_if(resource_exists::<RenetClient>()),
                        starwolves_response.run_if(resource_exists::<TokenAssignServer>()),
                        token_assign_server,
                        connect_to_server.after(starwolves_response),
                        clear_raw_spawn_entity_queue,
                    ),
                )
                .add_event::<ConnectToServer>()
                .add_plugins(RenetClientPlugin {
                    schedules: schedules,
                })
                .add_plugins(NetcodeClientPlugin {
                    schedules: schedules,
                })
                .add_event::<AssignTokenToServer>()
                .init_resource::<ConnectionPreferences>()
                .init_resource::<Connection>()
                .init_resource::<AssigningServerToken>()
                .init_resource::<TotalAdjustment>()
                .add_systems(
                    PreUpdate,
                    (
                        receive_incoming_reliable_server_messages,
                        receive_incoming_unreliable_server_messages,
                    )
                        .run_if(resource_exists::<RenetClient>())
                        .in_set(TypenamesSet::SendRawEvents),
                )
                .add_event::<IncomingRawReliableServerMessage>()
                .add_event::<IncomingRawUnreliableServerMessage>()
                .add_systems(
                    Update,
                    (
                        confirm_connection.run_if(is_client_connected),
                        /*start_sync_frequency
                        .before(sync_test_client)
                        .run_if(on_timer(Duration::from_secs_f32(0.1))),*/
                        sync_check_client
                            .run_if(is_client_connected)
                            .after(confirm_connection),
                        on_disconnect.run_if(connected),
                    ),
                )
                .init_resource::<OutgoingBuffer>()
                .add_systems(
                    PostUpdate,
                    (
                        step_buffer.run_if(resource_exists::<RenetClient>()),
                        post_update_send_messages
                            .in_set(PostUpdateSendMessage)
                            .run_if(resource_exists::<RenetClient>()),
                    ),
                )
                .add_systems(Update, detect_client_world_loaded)
                .init_resource::<LoadedGameWorldBuffer>()
                .add_event::<ClientGameWorldLoaded>();
        }

        app.configure_sets(
            PreUpdate,
            (
                MessagingSet::DeserializeIncoming,
                BuildingSet::RawTriggerBuild,
            )
                .chain(),
        )
        .init_resource::<TickRateStamp>()
        .init_resource::<HandleToEntity>()
        .init_resource::<PauseTickStep>()
        .add_systems(First, step_tickrate_stamp)
        .init_resource::<Typenames>()
        .add_systems(Startup, generate_typenames.after(TypenamesSet::Generate));
        register_reliable_message::<NetworkingClientMessage>(app, MessageSender::Client, true);
        register_unreliable_message::<UnreliableServerMessage>(app, MessageSender::Server);
        register_reliable_message::<NetworkingChatServerMessage>(app, MessageSender::Server, true);
        register_reliable_message::<NetworkingServerMessage>(app, MessageSender::Server, true);
        register_unreliable_message::<NetworkingUnreliableClientMessage>(
            app,
            MessageSender::Client,
        );
    }
}

pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_RELIABLE_UNORDERED_ID: u8 = 1;
pub const RENET_RELIABLE_ORDERED_ID: u8 = 2;
