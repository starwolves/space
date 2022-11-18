use std::env;

use crate::boarding::NetUIInputTransmitData;
use crate::connection::NetPlayerConn;
use crate::console_commands::rcon_console_commands;
use crate::health_ui::NetHealthUpdate;
use crate::input::InputAttackCell;
use crate::networking::incoming_messages;
use bevy::app::CoreStage::PostUpdate;
use bevy::{
    prelude::{App, CoreStage, ParallelSystemDescriptorCoercion, Plugin, SystemSet},
    time::FixedTimestep,
};

use networking::server::{net_system, InputListActionsEntity};
use resources::labels::{PostUpdateLabels, PreUpdateLabels, SummoningLabels, UpdateLabels};

use super::{
    boarding::{done_boarding, on_boarding, ui_input_boarding, BoardingPlayer},
    input::{apply_movement_input_controller, humanoid_controller_input},
    net::{
        build_graphics, mouse_direction_update, scene_ready_event, send_server_time,
        update_player_count, NetDoneBoarding, NetExamineEntity, NetOnBoarding,
        NetOnNewPlayerConnection, NetOnSetupUI, NetSendServerTime, NetSendWorldEnvironment,
        NetUpdatePlayerCount, NetUserName,
    },
    setup_ui::register_ui_input_boarding,
};
use crate::{
    boarding::BoardingAnnouncements,
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    connection::{connections, AuthidI},
    console_commands::{entity_console_commands, inventory_item_console_commands},
    finalize_entity_updates::finalize_entity_updates,
    health_ui::{health_ui_update, ClientHealthUICache},
    send_net::process_finalize_net,
    setup_ui::initialize_setupui,
};
use bevy::app::CoreStage::PreUpdate;

pub struct ConnectedPlayerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ConnectedPlayerPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_event::<NetUserName>()
                .add_event::<InputListActionsEntity>()
                .add_system(
                    apply_movement_input_controller.label(UpdateLabels::ProcessMovementInput),
                )
                .add_system(mouse_direction_update.before(UpdateLabels::StandardCharacters))
                .add_system(humanoid_controller_input.before(UpdateLabels::StandardCharacters))
                .add_event::<BoardingPlayer>()
                .add_system(done_boarding)
                .add_system(register_ui_input_boarding)
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
                .add_system(ui_input_boarding)
                .add_system(on_boarding)
                .add_event::<NetUpdatePlayerCount>()
                .add_system(build_graphics)
                .add_system(scene_ready_event)
                .add_event::<NetSendServerTime>()
                .add_system(initialize_setupui.label(SummoningLabels::TriggerSummon))
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(health_ui_update),
                )
                .add_event::<NetUIInputTransmitData>()
                .add_event::<NetSendWorldEnvironment>()
                .add_event::<NetOnBoarding>()
                .add_event::<NetOnNewPlayerConnection>()
                .add_event::<NetExamineEntity>()
                .add_event::<NetDoneBoarding>()
                .add_event::<NetOnSetupUI>()
                .init_resource::<AuthidI>()
                .add_event::<NetHealthUpdate>()
                .add_system_to_stage(CoreStage::Update, broadcast_interpolation_transforms)
                .add_system_to_stage(PreUpdate, connections.label(PreUpdateLabels::NetEvents))
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetOnBoarding>)
                        .with_system(net_system::<NetOnNewPlayerConnection>)
                        .with_system(net_system::<NetOnSetupUI>)
                        .with_system(net_system::<NetDoneBoarding>)
                        .with_system(net_system::<NetSendWorldEnvironment>)
                        .with_system(net_system::<NetUserName>)
                        .with_system(net_system::<NetUIInputTransmitData>)
                        .with_system(net_system::<NetExamineEntity>)
                        .with_system(net_system::<NetSendServerTime>)
                        .with_system(net_system::<NetHealthUpdate>)
                        .with_system(net_system::<NetPlayerConn>)
                        .with_system(net_system::<NetUpdatePlayerCount>),
                )
                .add_system_to_stage(
                    PostUpdate,
                    finalize_entity_updates
                        .after(PostUpdateLabels::EntityUpdate)
                        .label(PostUpdateLabels::SendEntityUpdates),
                )
                .add_system(rcon_console_commands)
                .add_system(
                    inventory_item_console_commands
                        .before(SummoningLabels::TriggerSummon)
                        .label(SummoningLabels::NormalSummon),
                )
                .add_system(entity_console_commands.after(SummoningLabels::DefaultSummon))
                .add_system_to_stage(
                    PostUpdate,
                    process_finalize_net.after(PostUpdateLabels::Net),
                )
                .init_resource::<ClientHealthUICache>()
                .init_resource::<BoardingAnnouncements>()
                .add_system_to_stage(
                    PreUpdate,
                    incoming_messages
                        .after(PreUpdateLabels::NetEvents)
                        .label(PreUpdateLabels::ProcessInput),
                )
                .add_event::<NetPlayerConn>()
                .add_event::<InputAttackCell>();
        }
    }
}
