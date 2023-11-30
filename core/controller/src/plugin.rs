use std::time::Duration;

use crate::connections::connections;
use crate::controller::{
    cache_controller, controller_input_entity_update, look_transform_entity_update, ControllerCache,
};
use crate::input::{
    apply_peer_sync_transform, clean_recorded_input, controller_input, create_input_map,
    get_client_input, get_peer_input, sync_controller_input, Controller, InputMovementInput,
    InputSet, LastPeerLookTransform, PeerSyncLookTransform, RecordedControllerInput,
    SyncControllerInput,
};
use crate::net::ControllerClientMessage;
use crate::networking::{
    incoming_messages, peer_replication, PeerReliableControllerMessage,
    PeerUnreliableControllerMessage,
};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};

use bevy::time::common_conditions::on_timer;
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender, MessagingSet,
};
use networking::server::EntityUpdatesSet;
use player::boarding::BoardingPlayer;
use resources::modes::{is_correction_mode, is_server_mode};
use resources::physics::PhysicsSet;
use resources::sets::{MainSet, UpdateSet};

use super::net::update_player_count;

#[derive(Default)]
pub struct ControllerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                FixedUpdate,
                (
                    update_player_count.run_if(on_timer(Duration::from_secs_f32(5.))),
                    connections,
                    peer_replication,
                )
                    .in_set(MainSet::Update),
            );
            if !is_correction_mode(app) {
                app.add_systems(
                    FixedUpdate,
                    (
                        controller_input_entity_update.in_set(EntityUpdatesSet::BuildUpdates),
                        look_transform_entity_update.in_set(EntityUpdatesSet::BuildUpdates),
                    )
                        .in_set(MainSet::PostUpdate),
                );
            }
            app.add_event::<BoardingPlayer>().add_systems(
                FixedUpdate,
                incoming_messages
                    .in_set(InputSet::First)
                    .in_set(MainSet::PreUpdate)
                    .after(MessagingSet::DeserializeIncoming),
            );
        } else {
            app.add_systems(Startup, create_input_map)
                .add_systems(
                    FixedUpdate,
                    (get_client_input, get_peer_input)
                        .in_set(InputSet::First)
                        .before(UpdateSet::StandardCharacters)
                        .in_set(MainSet::Update),
                )
                .add_systems(
                    FixedUpdate,
                    (
                        clean_recorded_input.in_set(MainSet::PreUpdate),
                        cache_controller
                            .after(MainSet::PostUpdate)
                            .in_set(PhysicsSet::Cache),
                        apply_peer_sync_transform
                            .after(InputSet::First)
                            .in_set(MainSet::Update),
                        sync_controller_input
                            .after(InputSet::First)
                            .in_set(MainSet::Update),
                    ),
                )
                .add_event::<PeerSyncLookTransform>()
                .init_resource::<LastPeerLookTransform>();
        }

        app.add_systems(
            FixedUpdate,
            controller_input
                .after(InputSet::First)
                .before(UpdateSet::StandardCharacters)
                .in_set(MainSet::Update)
                .in_set(Controller::Input),
        )
        .init_resource::<RecordedControllerInput>()
        .init_resource::<ControllerCache>()
        .add_event::<InputMovementInput>()
        .add_event::<SyncControllerInput>();
        register_reliable_message::<ControllerClientMessage>(app, MessageSender::Client, true);
        register_reliable_message::<PeerReliableControllerMessage>(
            app,
            MessageSender::Server,
            true,
        );
        register_unreliable_message::<PeerUnreliableControllerMessage>(app, MessageSender::Server);
    }
}
