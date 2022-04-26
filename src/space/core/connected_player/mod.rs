use bevy_app::{App, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};

use crate::space::{PostUpdateLabels, UpdateLabels};

use self::{
    events::{
        net_system, BoardingPlayer, InputAltItemAttack, InputAttackCell, InputAttackEntity,
        InputBuildGraphics, InputExamineEntity, InputExamineMap, InputMouseAction,
        InputMouseDirectionUpdate, InputMovementInput, InputSceneReady, InputSelectBodyPart,
        InputSprinting, InputTabDataEntity, InputTabDataMap, InputToggleAutoMove,
        InputToggleCombatMode, InputUIInput, InputUIInputTransmitText, InputUserName,
        NetDoneBoarding, NetExamineEntity, NetOnBoarding, NetOnNewPlayerConnection, NetOnSetupUI,
        NetOnSpawning, NetSendServerTime, NetSendWorldEnvironment, NetTabData,
        NetUIInputTransmitData, NetUpdatePlayerCount, NetUserName, TextTreeInputSelection,
    },
    resources::HandleToEntity,
    systems::{
        build_graphics_event::build_graphics_event, controller_input::controller_input,
        done_boarding::done_boarding, examine_entity::examine_entity, examine_map::examine_map,
        mouse_direction_update::mouse_direction_update, on_boarding::on_boarding,
        on_setupui::on_setupui, on_spawning::on_spawning, player_input_event::player_input_event,
        scene_ready_event::scene_ready_event, send_server_time::send_server_time,
        text_tree_input_selection::text_tree_input_selection, ui_input_event::ui_input_event,
        ui_input_transmit_data_event::ui_input_transmit_data_event,
        update_player_count::update_player_count,
    },
};

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

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
            .add_system(on_spawning)
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
            .add_system(on_setupui)
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
                net_system.after(PostUpdateLabels::VisibleChecker),
            );
    }
}
use bevy_app::CoreStage::PostUpdate;
