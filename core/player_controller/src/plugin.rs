use crate::console_commands::rcon_console_commands;
use crate::humanoid::default_human_dummy;
use api::data::{
    HandleToEntity, PostUpdateLabels, PreUpdateLabels, ServerId, SummoningLabels, UpdateLabels,
};
use bevy::app::CoreStage::PostUpdate;
use bevy::{
    prelude::{App, CoreStage, ParallelSystemDescriptorCoercion, Plugin, SystemSet},
    time::FixedTimestep,
};

use networking::{
    messages::{net_system, InputListActionsEntity},
    plugin::NetActionData,
};
use ui::ui::{InputUIInput, InputUIInputTransmitText, NetUIInputTransmitData};

use super::{
    boarding::{done_boarding, on_boarding, ui_input_boarding, BoardingPlayer},
    input::{apply_movement_input_controller, humanoid_controller_input},
    net::{
        build_graphics, mouse_direction_update, scene_ready_event, send_server_time,
        update_player_count, NetDoneBoarding, NetExamineEntity, NetOnBoarding,
        NetOnNewPlayerConnection, NetOnSetupUI, NetOnSpawning, NetSendServerTime,
        NetSendWorldEnvironment, NetUpdatePlayerCount, NetUserName,
    },
    setup_ui::register_ui_input_boarding,
};
use crate::{
    boarding::{on_spawning, BoardingAnnouncements},
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    connection::{connections, AuthidI},
    console_commands::{entity_console_commands, inventory_item_console_commands},
    health_ui::{health_ui_update, ClientHealthUICache},
    humanoid::humanoid_core_entity_updates,
    send_entity_update::finalize_entity_updates,
    send_net::process_finalize_net,
    setup_ui::initialize_setupui,
};
use bevy::app::CoreStage::PreUpdate;

pub struct ConnectedPlayerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ConnectedPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HandleToEntity>()
            .add_event::<NetUserName>()
            .add_event::<InputListActionsEntity>()
            .add_system(apply_movement_input_controller.label(UpdateLabels::ProcessMovementInput))
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
            .add_system(
                (default_human_dummy)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
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
            .add_system_to_stage(PostUpdate, on_spawning.after(PostUpdateLabels::Net))
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(health_ui_update),
            )
            .add_event::<NetUIInputTransmitData>()
            .add_event::<NetOnSpawning>()
            .add_event::<NetSendWorldEnvironment>()
            .add_event::<NetOnBoarding>()
            .add_event::<NetOnNewPlayerConnection>()
            .add_event::<NetExamineEntity>()
            .add_event::<InputUIInputTransmitText>()
            .add_event::<NetDoneBoarding>()
            .add_event::<NetOnSetupUI>()
            .add_event::<InputUIInput>()
            .init_resource::<AuthidI>()
            .add_system_to_stage(CoreStage::Update, broadcast_interpolation_transforms)
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(humanoid_core_entity_updates),
            )
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
                    .with_system(net_system::<NetOnSpawning>)
                    .with_system(net_system::<NetUserName>)
                    .with_system(net_system::<NetUIInputTransmitData>)
                    .with_system(net_system::<NetExamineEntity>)
                    .with_system(net_system::<NetActionData>)
                    .with_system(net_system::<NetSendServerTime>)
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
            .init_resource::<ServerId>();
    }
}
