use std::time::Duration;

use crate::connections::connections;
use crate::controller::{
    clean_controller_cache, controller_input_entity_update, look_transform_entity_update,
    ControllerCache,
};
use crate::input::{
    apply_controller_cache_to_peers, cache_peer_sync_look_transform, controller_input,
    create_input_map, keyboard_input, process_peer_input, sync_controller_input, ControllerSet,
    InputMovementInput, LastPeerLookTransform, PeerInputCache, PeerSyncLookTransform,
    SyncControllerInput,
};
use crate::net::ControllerClientMessage;
use crate::networking::{
    incoming_messages, peer_replicate_input_messages, syncable_entity, PeerLatestLookSync,
    PeerReliableControllerMessage, PeerUnreliableControllerMessage,
};
use bevy::app::Update;
use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::ecs::schedule::IntoSystemSetConfigs;
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};

use bevy::time::common_conditions::on_timer;
use bevy_renet::renet::RenetClient;
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender, MessagingSet,
};
use networking::server::EntityUpdatesSet;
use physics::sync::SpawningSimulation;
use player::boarding::BoardingPlayer;
use resources::input::InputSet;
use resources::modes::{is_correction_mode, is_server_mode};
use resources::physics::PhysicsSet;
use resources::sets::{MainSet, UpdateSet};

use super::net::update_player_count;
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) && !is_correction_mode(app) {
            app.add_systems(
                FixedUpdate,
                (
                    update_player_count.run_if(on_timer(Duration::from_secs_f32(5.))),
                    connections,
                )
                    .in_set(MainSet::Update),
            )
            .add_systems(
                FixedUpdate,
                (
                    (
                        controller_input_entity_update.in_set(EntityUpdatesSet::BuildUpdates),
                        look_transform_entity_update.in_set(EntityUpdatesSet::BuildUpdates),
                    )
                        .in_set(MainSet::PostUpdate),
                    syncable_entity.in_set(MainSet::Update),
                ),
            )
            .add_systems(
                Update,
                peer_replicate_input_messages.in_set(MainSet::Update),
            )
            .init_resource::<PeerLatestLookSync>();

            app.add_event::<BoardingPlayer>().add_systems(
                FixedUpdate,
                incoming_messages
                    .in_set(InputSet::Prepare)
                    .in_set(MainSet::PreUpdate)
                    .after(MessagingSet::DeserializeIncoming),
            );
        }
        if !is_server_mode(app) {
            app.add_systems(Startup, create_input_map)
                .add_systems(
                    FixedUpdate,
                    (process_peer_input.run_if(resource_exists::<RenetClient>()),)
                        .in_set(InputSet::Prepare)
                        .before(UpdateSet::StandardCharacters)
                        .in_set(MainSet::Update),
                )
                .add_systems(
                    Update,
                    keyboard_input
                        .before(UpdateSet::StandardCharacters)
                        .in_set(InputSet::Prepare)
                        .in_set(MainSet::Update)
                        .run_if(resource_exists::<RenetClient>()),
                )
                .add_systems(
                    FixedUpdate,
                    (
                        clean_controller_cache
                            .in_set(MainSet::Update)
                            .in_set(PhysicsSet::Cache),
                        cache_peer_sync_look_transform
                            .in_set(InputSet::Cache)
                            .in_set(MainSet::Update)
                            .after(SpawningSimulation::Spawn),
                        sync_controller_input
                            .in_set(InputSet::Cache)
                            .in_set(MainSet::Update),
                        apply_controller_cache_to_peers
                            .in_set(MainSet::Update)
                            .in_set(InputSet::ApplyLiveCache),
                    ),
                )
                .add_event::<PeerSyncLookTransform>()
                .init_resource::<LastPeerLookTransform>()
                .init_resource::<PeerInputCache>();
        }
        let list = (InputSet::Prepare, InputSet::Cache, InputSet::ApplyLiveCache);
        app.configure_sets(FixedUpdate, list.clone().chain());
        app.configure_sets(Update, list.chain());

        if !is_correction_mode(app) {
            app.add_systems(
                FixedUpdate,
                controller_input
                    .in_set(InputSet::Cache)
                    .before(UpdateSet::StandardCharacters)
                    .in_set(MainSet::Update)
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
