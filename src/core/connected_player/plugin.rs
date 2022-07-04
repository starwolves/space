use bevy::{
    core::FixedTimestep,
    prelude::{App, Entity, ParallelSystemDescriptorCoercion, Plugin, SystemSet},
};

use crate::core::{
    gridmap::gridmap::Vec3Int,
    humanoid::humanoid::InputToggleCombatMode,
    networking::networking::GridMapType,
    pawn::user_name::InputUserName,
    space_plugin::plugin::{PostUpdateLabels, SummoningLabels, UpdateLabels},
    tab_actions::tab_data::InputTabDataEntity,
};

use super::{
    boarding::{
        done_boarding, on_boarding, ui_input_transmit_data_event, BoardingPlayer,
        InputUIInputTransmitText,
    },
    examine::{examine_entity, examine_map, InputExamineEntity, InputExamineMap},
    input::{
        controller_input, player_input_event, text_tree_input_selection, InputAltItemAttack,
        InputAttackCell, InputAttackEntity, InputMouseAction, InputMovementInput,
        InputSelectBodyPart, InputSprinting, InputToggleAutoMove, TextTreeInputSelection,
    },
    net::{
        build_graphics_event, mouse_direction_update, net_system, scene_ready_event,
        send_server_time, update_player_count, InputBuildGraphics, InputMouseDirectionUpdate,
        InputSceneReady, NetDoneBoarding, NetExamineEntity, NetOnBoarding,
        NetOnNewPlayerConnection, NetOnSetupUI, NetOnSpawning, NetSendServerTime,
        NetSendWorldEnvironment, NetTabData, NetUIInputTransmitData, NetUpdatePlayerCount,
        NetUserName,
    },
    spawn::on_spawning,
    ui::{on_setupui, ui_input_event, InputUIInput},
};

#[derive(Debug)]
pub struct InputTabDataMap {
    pub player_entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
}

pub struct ConnectedPlayerPlugin;

impl Plugin for ConnectedPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HandleToEntity>()
            .add_event::<InputBuildGraphics>()
            .add_event::<InputMovementInput>()
            .add_event::<InputExamineEntity>()
            .add_event::<InputExamineMap>()
            .add_event::<InputSprinting>()
            .add_event::<InputToggleCombatMode>()
            .add_event::<NetUserName>()
            .add_event::<InputTabDataEntity>()
            .add_system(player_input_event.label(UpdateLabels::ProcessMovementInput))
            .add_system(mouse_direction_update.before(UpdateLabels::StandardCharacters))
            .add_system(controller_input.before(UpdateLabels::StandardCharacters))
            .add_event::<BoardingPlayer>()
            .add_system(done_boarding)
            .add_system(ui_input_event)
            .add_system(examine_map)
            .add_system(examine_entity)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.))
                    .with_system(send_server_time)
                    .with_system(update_player_count),
            )
            .add_event::<InputTabDataMap>()
            .add_system(ui_input_transmit_data_event)
            .add_system(on_boarding)
            .add_system(text_tree_input_selection.label(UpdateLabels::TextTreeInputSelection))
            .add_event::<InputToggleAutoMove>()
            .add_event::<TextTreeInputSelection>()
            .add_event::<NetUpdatePlayerCount>()
            .add_event::<NetTabData>()
            .add_system(build_graphics_event)
            .add_system(scene_ready_event)
            .add_event::<NetSendServerTime>()
            .add_system(on_setupui.label(SummoningLabels::TriggerSummon))
            .add_system_to_stage(PostUpdate, on_spawning.after(PostUpdateLabels::Net))
            .add_event::<InputMouseAction>()
            .add_event::<InputAttackCell>()
            .add_event::<InputAltItemAttack>()
            .add_event::<NetUIInputTransmitData>()
            .add_event::<NetOnSpawning>()
            .add_event::<InputUserName>()
            .add_event::<InputAttackEntity>()
            .add_event::<InputSelectBodyPart>()
            .add_event::<InputMouseDirectionUpdate>()
            .add_event::<NetSendWorldEnvironment>()
            .add_event::<NetOnBoarding>()
            .add_event::<NetOnNewPlayerConnection>()
            .add_event::<NetExamineEntity>()
            .add_event::<InputUIInputTransmitText>()
            .add_event::<NetDoneBoarding>()
            .add_event::<NetOnSetupUI>()
            .add_event::<InputUIInput>()
            .add_event::<InputSceneReady>()
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            );
    }
}
use bevy::app::CoreStage::PostUpdate;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleToEntity {
    pub map: HashMap<u64, Entity>,
    pub inv_map: HashMap<Entity, u64>,
}
