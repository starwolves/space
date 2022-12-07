use std::env;

use crate::connections::{configure, connections};
use crate::input::{
    InputAltItemAttack, InputAttackCell, InputAttackEntity, InputBuildGraphics, InputMouseAction,
    InputMouseDirectionUpdate, InputMovementInput, InputSelectBodyPart, InputSprinting,
    InputToggleAutoMove, InputToggleCombatMode,
};
use crate::networking::{
    incoming_messages, ControllerClientMessage, ControllerUnreliableClientMessage,
};
use bevy::prelude::IntoSystemDescriptor;
use bevy::{
    prelude::{App, Plugin, SystemSet},
    time::FixedTimestep,
};

use networking::messaging::{init_reliable_message, init_unreliable_message, MessageSender};
use player::boarding::BoardingPlayer;
use player::plugin::ConfigurationLabel;
use resources::labels::UpdateLabels;

use super::{
    input::apply_movement_input_controller,
    net::{build_graphics, send_server_time, update_player_count},
};

use bevy::app::CoreStage::PreUpdate;

#[derive(Default)]
pub struct ControllerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(
                apply_movement_input_controller.label(UpdateLabels::ProcessMovementInput),
            )
            .add_event::<BoardingPlayer>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(10.))
                    .with_system(update_player_count),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.))
                    .with_system(send_server_time),
            )
            .add_system(build_graphics)
            .add_system(connections)
            .add_system_to_stage(PreUpdate, incoming_messages)
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
                    .label(ConfigurationLabel::Main)
                    .after(ConfigurationLabel::SpawnEntity),
            );
        }

        init_reliable_message::<ControllerClientMessage>(app, MessageSender::Client);
        init_unreliable_message::<ControllerUnreliableClientMessage>(app, MessageSender::Client);
    }
}
