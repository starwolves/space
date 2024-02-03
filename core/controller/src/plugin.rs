use std::time::Duration;

use crate::connections::handle_disconnects;
use crate::controller::{
    clean_controller_cache, controller_input_entity_update, look_transform_entity_update,
    server_sync_look_transform, ControllerCache,
};
use crate::input::{
    apply_controller_cache_to_peers, cache_peer_sync_look_transform, controller_input,
    create_input_map, keyboard_input, process_peer_input, sync_controller_input, ControllerSet,
    InputMovementInput, LastPeerLookTransform, PeerInputCache, PeerSyncLookTransform,
    SyncControllerInput,
};
use crate::net::ControllerClientMessage;
use crate::networking::{
    incoming_messages, server_replicate_peer_input_messages, syncable_entity, PeerLatestLookSync,
    PeerReliableControllerMessage, PeerUnreliableControllerMessage,
};
use bevy::app::PreUpdate as BevyPreUpdate;
use bevy::app::Update as BevyUpdate;

use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::ecs::schedule::IntoSystemSetConfigs;
use bevy::input::InputSystem;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};

use bevy::time::common_conditions::on_timer;
use bevy_renet::renet::RenetClient;
use networking::client::BevyPreUpdateSendMessage;
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender, MessagingSet,
};
use networking::server::EntityUpdatesSet;
use pawn::camera::LookTransformSet;
use physics::sync::SpawningSimulation;
use player::boarding::BoardingPlayer;
use resources::input::InputSet;
use resources::modes::{is_correction_mode, is_server_mode};
use resources::ordering::{Fin, PreUpdate, SensingSet, Update, UpdateSet};
use resources::physics::PhysicsSet;

use super::net::update_player_count;
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) && !is_correction_mode(app) {
            app.add_systems(
                Update,
                (
                    update_player_count.run_if(on_timer(Duration::from_secs_f32(5.))),
                    handle_disconnects,
                    controller_input_entity_update
                        .after(controller_input)
                        .in_set(EntityUpdatesSet::BuildUpdates),
                    look_transform_entity_update
                        .before(server_sync_look_transform)
                        .in_set(EntityUpdatesSet::BuildUpdates),
                    server_sync_look_transform.in_set(LookTransformSet::Sync),
                    syncable_entity,
                    server_replicate_peer_input_messages.after(SensingSet::VisibleChecker),
                ),
            )
            .init_resource::<PeerLatestLookSync>();

            app.add_event::<BoardingPlayer>().add_systems(
                PreUpdate,
                incoming_messages
                    .in_set(InputSet::Prepare)
                    .after(MessagingSet::DeserializeIncoming),
            );
        }
        if !is_server_mode(app) {
            app.add_systems(Startup, create_input_map)
                .add_systems(Fin, clean_controller_cache.in_set(PhysicsSet::Cache))
                .add_systems(
                    Update,
                    (
                        process_peer_input
                            .run_if(resource_exists::<RenetClient>())
                            .in_set(InputSet::Prepare)
                            .before(UpdateSet::StandardCharacters),
                        cache_peer_sync_look_transform
                            .in_set(InputSet::Cache)
                            .after(SpawningSimulation::Spawn),
                        sync_controller_input
                            .after(controller_input)
                            .in_set(InputSet::Cache),
                        apply_controller_cache_to_peers.in_set(InputSet::ApplyLiveCache),
                    )
                        .before(UpdateSet::StandardCharacters),
                )
                .add_systems(
                    BevyPreUpdate,
                    keyboard_input
                        .before(BevyPreUpdateSendMessage)
                        .before(UpdateSet::StandardCharacters)
                        .in_set(InputSet::Prepare)
                        .run_if(resource_exists::<RenetClient>())
                        .after(InputSystem),
                )
                .add_event::<PeerSyncLookTransform>()
                .init_resource::<LastPeerLookTransform>()
                .init_resource::<PeerInputCache>();
        }
        let list = (InputSet::Prepare, InputSet::Cache, InputSet::ApplyLiveCache);
        app.configure_sets(Update, list.clone().chain());
        app.configure_sets(BevyUpdate, list.clone().chain());
        app.configure_sets(BevyPreUpdate, list.chain());

        if !is_correction_mode(app) {
            app.add_systems(
                Update,
                controller_input
                    .in_set(InputSet::Cache)
                    .before(UpdateSet::StandardCharacters)
                    .in_set(ControllerSet::Input),
            )
            .add_event::<InputMovementInput>()
            .add_event::<SyncControllerInput>();
            register_reliable_message::<ControllerClientMessage>(app, MessageSender::Client, true);
            register_reliable_message::<PeerReliableControllerMessage>(
                app,
                MessageSender::Server,
                true,
            );
            register_unreliable_message::<PeerUnreliableControllerMessage>(
                app,
                MessageSender::Server,
            );
        }

        app.init_resource::<ControllerCache>();
    }
}
