use std::env;

use crate::connections::{configure, connections, NetConfigure};
use crate::input::{
    InputAltItemAttack, InputAttackCell, InputAttackEntity, InputBuildGraphics, InputMouseAction,
    InputMouseDirectionUpdate, InputMovementInput, InputSceneReady, InputSelectBodyPart,
    InputSprinting, InputToggleAutoMove, InputToggleCombatMode,
};
use crate::networking::incoming_messages;
use bevy::app::CoreStage::PostUpdate;
use bevy::prelude::IntoSystemDescriptor;
use bevy::{
    prelude::{App, Plugin, SystemSet},
    time::FixedTimestep,
};

use networking::server::net_system;
use player::boarding::BoardingPlayer;
use resources::labels::{PostUpdateLabels, PreUpdateLabels, UpdateLabels};

use super::{
    input::apply_movement_input_controller,
    net::{
        build_graphics, scene_ready_event, send_server_time, update_player_count, NetExamineEntity,
        NetOnNewPlayerConnection, NetSendServerTime, NetSendWorldEnvironment, NetUpdatePlayerCount,
        NetUserName,
    },
};

use bevy::app::CoreStage::PreUpdate;

pub struct ControllerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_event::<NetUserName>()
                .add_system(
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
                .add_event::<NetUpdatePlayerCount>()
                .add_system(build_graphics)
                .add_system(scene_ready_event)
                .add_event::<NetSendServerTime>()
                .add_event::<NetSendWorldEnvironment>()
                .add_event::<NetOnNewPlayerConnection>()
                .add_event::<NetExamineEntity>()
                .add_system_to_stage(PreUpdate, connections.label(PreUpdateLabels::NetEvents))
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetOnNewPlayerConnection>)
                        .with_system(net_system::<NetSendWorldEnvironment>)
                        .with_system(net_system::<NetUserName>)
                        .with_system(net_system::<NetExamineEntity>)
                        .with_system(net_system::<NetSendServerTime>)
                        .with_system(net_system::<NetUpdatePlayerCount>)
                        .with_system(net_system::<NetConfigure>),
                )
                .add_system_to_stage(
                    PreUpdate,
                    incoming_messages
                        .after(PreUpdateLabels::NetEvents)
                        .label(PreUpdateLabels::ProcessInput),
                )
                .add_event::<InputAttackCell>()
                .add_event::<InputToggleCombatMode>()
                .add_event::<InputToggleAutoMove>()
                .add_event::<InputAttackEntity>()
                .add_event::<InputAltItemAttack>()
                .add_event::<InputMouseAction>()
                .add_event::<InputSelectBodyPart>()
                .add_event::<InputMovementInput>()
                .add_event::<InputSprinting>()
                .add_event::<InputSceneReady>()
                .add_event::<InputBuildGraphics>()
                .add_event::<InputMouseDirectionUpdate>()
                .add_event::<NetConfigure>()
                .add_system_to_stage(PreUpdate, configure.label(PreUpdateLabels::NetEvents));
        }
    }
}
