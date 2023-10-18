use std::time::Duration;

use crate::connections::connections;
use crate::input::{
    controller_input, create_input_map, get_client_input, get_peer_input, Controller,
    InputAltItemAttack, InputAttackCell, InputAttackEntity, InputBuildGraphics, InputMouseAction,
    InputMouseDirectionUpdate, InputMovementInput, InputSet, InputSprinting, InputToggleAutoMove,
    InputToggleCombatMode,
};
use crate::net::{ControllerClientMessage, ControllerUnreliableClientMessage};
use crate::networking::{incoming_messages, peer_replication, PeerReliableControllerMessage};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};

use bevy::time::common_conditions::on_fixed_timer;
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender, MessagingSet,
};
use player::boarding::BoardingPlayer;
use resources::is_server::is_server;
use resources::sets::{MainSet, UpdateSet};

use super::net::update_player_count;

#[derive(Default)]
pub struct ControllerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    update_player_count.run_if(on_fixed_timer(Duration::from_secs_f32(5.))),
                    connections,
                    peer_replication,
                )
                    .in_set(MainSet::Update),
            )
            .add_event::<BoardingPlayer>()
            .add_systems(
                FixedUpdate,
                incoming_messages
                    .in_set(InputSet::First)
                    .in_set(MainSet::PreUpdate)
                    .after(MessagingSet::DeserializeIncoming),
            );
        } else {
            app.add_systems(Startup, create_input_map).add_systems(
                FixedUpdate,
                (get_client_input, get_peer_input)
                    .in_set(InputSet::First)
                    .before(UpdateSet::StandardCharacters)
                    .in_set(MainSet::Update),
            );
        }

        app.add_systems(
            FixedUpdate,
            controller_input
                .after(InputSet::First)
                .before(UpdateSet::StandardCharacters)
                .in_set(MainSet::Update)
                .in_set(Controller::Input),
        )
        .add_event::<InputMovementInput>()
        .add_event::<InputAttackCell>()
        .add_event::<InputToggleCombatMode>()
        .add_event::<InputToggleAutoMove>()
        .add_event::<InputAttackEntity>()
        .add_event::<InputAltItemAttack>()
        .add_event::<InputMouseAction>()
        .add_event::<InputSprinting>()
        .add_event::<InputBuildGraphics>()
        .add_event::<InputMouseDirectionUpdate>();
        register_reliable_message::<ControllerClientMessage>(app, MessageSender::Client);
        register_reliable_message::<PeerReliableControllerMessage>(app, MessageSender::Server);

        register_unreliable_message::<ControllerUnreliableClientMessage>(
            app,
            MessageSender::Client,
        );
    }
}
