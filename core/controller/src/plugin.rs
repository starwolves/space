use std::time::Duration;

use crate::connections::connections;
use crate::input::{
    controller_input, create_input_map, get_client_input, InputAltItemAttack, InputAttackCell,
    InputAttackEntity, InputBuildGraphics, InputMouseAction, InputMouseDirectionUpdate,
    InputMovementInput, InputSprinting, InputToggleAutoMove, InputToggleCombatMode,
};
use crate::net::{ControllerClientMessage, ControllerUnreliableClientMessage};
use crate::networking::incoming_messages;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Startup, Update};

use bevy::time::common_conditions::on_fixed_timer;
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender,
};
use player::boarding::BoardingPlayer;
use resources::is_server::is_server;
use resources::labels::UpdateSets;

use super::net::{send_server_time, update_player_count};

#[derive(Default)]
pub struct ControllerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                Update,
                (
                    send_server_time.run_if(on_fixed_timer(Duration::from_secs_f32(2.))),
                    update_player_count.run_if(on_fixed_timer(Duration::from_secs_f32(10.))),
                    connections,
                ),
            )
            .add_event::<BoardingPlayer>()
            .add_systems(PreUpdate, incoming_messages);
        } else {
            app.add_systems(Startup, create_input_map).add_systems(
                Update,
                get_client_input.before(UpdateSets::StandardCharacters),
            );
        }

        app.add_systems(
            Update,
            controller_input.before(UpdateSets::StandardCharacters),
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
        register_unreliable_message::<ControllerUnreliableClientMessage>(
            app,
            MessageSender::Client,
        );
    }
}
