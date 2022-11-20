use std::env;

use bevy::prelude::{App, CoreStage, IntoSystemDescriptor, Plugin, SystemSet};
use bevy::time::FixedTimestep;
use console_commands::commands::ConsoleCommandsLabels;
use networking::server::net_system;
use resources::labels::{
    ActionsLabels, PostUpdateLabels, PreUpdateLabels, StartupLabels, SummoningLabels,
};

use crate::actions::build_actions;
use crate::broadcast_interpolation_transforms::broadcast_interpolation_transforms;
use crate::despawn::{despawn_entity, DespawnEntity, NetDespawnEntity};
use crate::entity_data::{world_mode_update, RawSpawnEvent, INTERPOLATION_LABEL1};
use crate::examine::{
    examine_entity, examine_entity_health, finalize_entity_examine_input, finalize_examine_entity,
    ExamineEntityMessages, InputExamineEntity, NetConnExamine, NetExamine,
};
use crate::finalize_entity_updates::{
    finalize_entity_updates, NetEntityUpdate, NetFinalizeEntityUpdates,
};
use crate::init::{initialize_console_commands, startup_entities};
use crate::meta::EntityDataResource;
use crate::networking::{
    incoming_messages, load_entity, LoadEntity, NetLoadEntity, NetUnloadEntity,
};
use crate::spawn::DefaultSpawnEvent;
use crate::visible_checker::visible_checker;

use super::entity_data::{broadcast_position_updates, NetShowcase};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_event::<NetShowcase>()
                .init_resource::<EntityDataResource>()
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
                .add_startup_system(
                    initialize_console_commands
                        .before(ConsoleCommandsLabels::Finalize)
                        .label(SummoningLabels::TriggerSummon),
                )
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetShowcase>),
                )
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(world_mode_update),
                )
                .add_system_to_stage(
                    PostUpdate,
                    visible_checker
                        .after(PostUpdateLabels::SendEntityUpdates)
                        .label(PostUpdateLabels::VisibleChecker),
                )
                .add_system_to_stage(
                    PostUpdate,
                    despawn_entity.after(PostUpdateLabels::VisibleChecker),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                )
                .add_event::<NetExamine>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetExamine>)
                        .with_system(net_system::<NetConnExamine>)
                        .with_system(net_system::<NetLoadEntity>)
                        .with_system(net_system::<NetUnloadEntity>)
                        .with_system(net_system::<NetFinalizeEntityUpdates>)
                        .with_system(net_system::<NetEntityUpdate>)
                        .with_system(net_system::<NetDespawnEntity>),
                )
                .add_event::<NetConnExamine>()
                .add_system_to_stage(
                    PostUpdate,
                    finalize_examine_entity.before(PostUpdateLabels::EntityUpdate),
                )
                .add_system(examine_entity_health.after(ActionsLabels::Action))
                .init_resource::<ExamineEntityMessages>()
                .add_system_to_stage(
                    PreUpdate,
                    finalize_entity_examine_input.after(PreUpdateLabels::ProcessInput),
                )
                .add_system(examine_entity.after(ActionsLabels::Action))
                .add_system_to_stage(
                    PreUpdate,
                    incoming_messages
                        .after(PreUpdateLabels::NetEvents)
                        .label(PreUpdateLabels::ProcessInput),
                )
                .add_event::<NetFinalizeEntityUpdates>()
                .add_event::<InputExamineEntity>()
                .add_event::<NetUnloadEntity>()
                .add_event::<NetLoadEntity>()
                .add_event::<NetEntityUpdate>()
                .add_system_to_stage(CoreStage::Update, broadcast_interpolation_transforms)
                .add_system_to_stage(
                    PostUpdate,
                    finalize_entity_updates
                        .after(PostUpdateLabels::EntityUpdate)
                        .label(PostUpdateLabels::SendEntityUpdates),
                )
                .add_system(load_entity)
                .add_event::<NetDespawnEntity>()
                .add_event::<DespawnEntity>()
                .add_event::<LoadEntity>();
        }
    }
}
