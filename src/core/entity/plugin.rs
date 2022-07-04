use bevy::core::FixedTimestep;
use bevy::prelude::{info, App, ParallelSystemDescriptorCoercion, Plugin, Res, ResMut, SystemSet};

use crate::core::console_commands::commands::AllConsoleCommands;
use crate::core::console_commands::plugins::ConsoleCommandsLabels;
use crate::core::networking::networking::ConsoleCommandVariant;
use crate::core::space_plugin::plugin::{PostUpdateLabels, StartupLabels, SummoningLabels};
use crate::entities::line_arrow::console_command::entity_console_commands;

use super::entity_data::{
    broadcast_position_updates, EntityDataResource, NetShowcase, RawSpawnEvent,
    INTERPOLATION_LABEL1,
};
use super::entity_updates::{send_entity_updates, NetSendEntityUpdates};
use super::load_entity::{NetLoadEntity, NetUnloadEntity};
use super::net::net_system;
use super::spawn::DefaultSpawnEvent;


pub fn startup_entities(entity_data: Res<EntityDataResource>) {
    info!("Loaded {} different entity types.", entity_data.data.len());
}

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityDataResource>()
            .add_system_to_stage(
                PostUpdate,
                send_entity_updates
                    .after(PostUpdateLabels::EntityUpdate)
                    .label(PostUpdateLabels::SendEntityUpdates),
            )
            .add_event::<NetShowcase>()
            .add_event::<NetSendEntityUpdates>()
            .add_event::<NetUnloadEntity>()
            .add_event::<NetLoadEntity>()
            .add_event::<RawSpawnEvent>()
            .add_event::<DefaultSpawnEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(broadcast_position_updates),
            )
            .add_startup_system(
                startup_entities
                    .before(StartupLabels::BuildGridmap)
                    .label(StartupLabels::InitEntities),
            )
            .add_system(entity_console_commands.after(SummoningLabels::DefaultSummon))
            .add_startup_system(
                initialize_console_commands
                    .before(ConsoleCommandsLabels::Finalize)
                    .label(SummoningLabels::TriggerSummon),
            )
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            );
    }
}
use bevy::app::CoreStage::PostUpdate;
pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "spawn".to_string(),
        "For server administrators only. Spawn in entities in proximity.".to_string(),
        vec![
            ("entity_name".to_string(), ConsoleCommandVariant::String),
            ("amount".to_string(), ConsoleCommandVariant::Int),
            ("player_selector".to_string(), ConsoleCommandVariant::String),
        ],
    ));
}
