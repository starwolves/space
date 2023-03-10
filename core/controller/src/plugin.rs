use std::time::Duration;

use crate::connections::{configure, connections};
use crate::input::{
    InputAltItemAttack, InputAttackCell, InputAttackEntity, InputBuildGraphics, InputMouseAction,
    InputMouseDirectionUpdate, InputMovementInput, InputSelectBodyPart, InputSprinting,
    InputToggleAutoMove, InputToggleCombatMode,
};
use crate::net::{ControllerClientMessage, ControllerUnreliableClientMessage};
use crate::networking::incoming_messages;
use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};

use bevy::time::common_conditions::on_fixed_timer;
use networking::messaging::{
    register_reliable_message, register_unreliable_message, MessageSender,
};
use player::boarding::BoardingPlayer;
use player::plugin::ConfigurationLabel;
use resources::is_server::is_server;
use resources::labels::UpdateLabels;

use super::{
    input::apply_movement_input_controller,
    net::{send_server_time, update_player_count},
};

#[derive(Default)]
pub struct ControllerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                apply_movement_input_controller.in_set(UpdateLabels::ProcessMovementInput),
            )
            .add_event::<BoardingPlayer>()
            .add_system(update_player_count.run_if(on_fixed_timer(Duration::from_secs_f32(10.))))
            .add_system(send_server_time.run_if(on_fixed_timer(Duration::from_secs_f32(2.))))
            .add_system(connections)
            .add_system(incoming_messages.in_base_set(CoreSet::PreUpdate))
            .add_event::<InputAttackCell>()
            .add_event::<InputToggleCombatMode>()
            .add_event::<InputToggleAutoMove>()
            .add_event::<InputAttackEntity>()
            .add_event::<InputAltItemAttack>()
            .add_event::<InputMouseAction>()
            .add_event::<InputSelectBodyPart>()
            .add_event::<InputMovementInput>()
            .add_event::<InputSprinting>()
            .add_event::<InputBuildGraphics>()
            .add_event::<InputMouseDirectionUpdate>()
            .add_system(
                configure
                    .in_set(ConfigurationLabel::Main)
                    .after(ConfigurationLabel::SpawnEntity),
            );
        }

        register_reliable_message::<ControllerClientMessage>(app, MessageSender::Client);
        register_unreliable_message::<ControllerUnreliableClientMessage>(
            app,
            MessageSender::Client,
        );
    }
}
