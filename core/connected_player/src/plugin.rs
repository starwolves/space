use bevy::{
    core::FixedTimestep,
    prelude::{App, CoreStage, ParallelSystemDescriptorCoercion, Plugin, SystemSet},
};
use networking::{
    messages::{net_system, InputTabDataEntity},
    plugin::NetTabData,
};
use api::{
    data::{
        HandleToEntity, PostUpdateLabels, PreUpdateLabels, ServerId, SummoningLabels, UpdateLabels,
    },
    examinable::ExamineLabels,
    gridmap::GridmapExamineMessages,
    network::{InputUIInput, InputUIInputTransmitText},
    tab_actions::TabActionsQueueLabels,
};

use super::{
    boarding::{done_boarding, on_boarding, ui_input_transmit_data_event, BoardingPlayer},
    input::{controller_input, player_input_event, text_tree_input_selection},
    net::{
        build_graphics_event, mouse_direction_update, scene_ready_event, send_server_time,
        update_player_count, NetDoneBoarding, NetExamineEntity, NetOnBoarding,
        NetOnNewPlayerConnection, NetOnSetupUI, NetOnSpawning, NetSendServerTime,
        NetSendWorldEnvironment, NetUIInputTransmitData, NetUpdatePlayerCount, NetUserName,
    },
    setup_ui::ui_input_event,
};
use crate::{
    boarding::{on_spawning, BoardingAnnouncements},
    broadcast_interpolation_transforms::broadcast_interpolation_transforms,
    chat::{chat_message_input_event, NetChatMessage},
    connection::{connections, AuthidI},
    console_commands::{entity_console_commands, inventory_item_console_commands},
    examine_events::{
        examine_entity, examine_map, finalize_examine_entity, finalize_examine_map, NetConnExamine,
    },
    execute_tab_action::execute_tab_actions,
    health_ui::{health_ui_update, ClientHealthUICache},
    humanoid::humanoid_update,
    send_entity_update::send_entity_updates,
    send_net::process_net,
    setup_ui::on_setupui,
};
use bevy::app::CoreStage::PreUpdate;

pub struct ConnectedPlayerPlugin {
    pub custom_motd: Option<String>,
}

impl Plugin for ConnectedPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HandleToEntity>()
            .add_event::<NetUserName>()
            .add_event::<InputTabDataEntity>()
            .add_system(player_input_event.label(UpdateLabels::ProcessMovementInput))
            .add_system(mouse_direction_update.before(UpdateLabels::StandardCharacters))
            .add_system(controller_input.before(UpdateLabels::StandardCharacters))
            .add_event::<BoardingPlayer>()
            .add_system(done_boarding)
            .add_system(ui_input_event)
            .add_system(
                examine_map
                    .after(ExamineLabels::Start)
                    .label(ExamineLabels::Default),
            )
            .init_resource::<GridmapExamineMessages>()
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
            .add_system(ui_input_transmit_data_event)
            .add_system(on_boarding)
            .add_system(text_tree_input_selection.label(UpdateLabels::TextTreeInputSelection))
            .add_event::<NetUpdatePlayerCount>()
            .add_system(build_graphics_event)
            .add_system(scene_ready_event)
            .add_event::<NetSendServerTime>()
            .add_system(on_setupui.label(SummoningLabels::TriggerSummon))
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
            .add_event::<NetConnExamine>()
            .add_system_to_stage(CoreStage::Update, broadcast_interpolation_transforms)
            .add_system(execute_tab_actions.label(TabActionsQueueLabels::TabAction))
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(humanoid_update),
            )
            .add_system_to_stage(
                PostUpdate,
                finalize_examine_map.before(PostUpdateLabels::EntityUpdate),
            )
            .add_system_to_stage(
                PostUpdate,
                finalize_examine_entity.before(PostUpdateLabels::EntityUpdate),
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
                    .with_system(net_system::<NetTabData>)
                    .with_system(net_system::<NetSendServerTime>)
                    .with_system(net_system::<NetUpdatePlayerCount>)
                    .with_system(net_system::<NetConnExamine>)
                    .with_system(net_system::<NetChatMessage>),
            )
            .add_system_to_stage(
                PostUpdate,
                send_entity_updates
                    .after(PostUpdateLabels::EntityUpdate)
                    .label(PostUpdateLabels::SendEntityUpdates),
            )
            .add_system(console_commands)
            .add_system(
                inventory_item_console_commands
                    .before(SummoningLabels::TriggerSummon)
                    .label(SummoningLabels::NormalSummon),
            )
            .add_system(entity_console_commands.after(SummoningLabels::DefaultSummon))
            .add_system(chat_message_input_event)
            .add_event::<NetChatMessage>()
            .add_system_to_stage(PostUpdate, process_net.after(PostUpdateLabels::Net))
            .add_system(
                examine_entity
                    .after(ExamineLabels::Start)
                    .label(ExamineLabels::Default),
            )
            .init_resource::<ClientHealthUICache>()
            .init_resource::<BoardingAnnouncements>()
            .init_resource::<ServerId>();
    }
}
use crate::console_commands::console_commands;
use bevy::app::CoreStage::PostUpdate;
